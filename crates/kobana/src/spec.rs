use std::collections::BTreeMap;

use crate::error::KobanaError;

/// Parsed OpenAPI spec — only the parts we need
#[derive(Debug, Clone)]
pub struct ApiSpec {
    pub version: String,
    pub paths: BTreeMap<String, PathItem>,
}

#[derive(Debug, Clone)]
pub struct PathItem {
    pub operations: BTreeMap<HttpMethod, Operation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
    pub request_body: Option<serde_json::Value>,
    pub responses: BTreeMap<String, serde_json::Value>,
    pub tags: Vec<String>,
    /// Whether a 2xx response wraps one record, found at parse time with the
    /// whole document at hand: the envelope is often a `$ref`
    /// (`#/components/schemas/OrganizationResponse`) that `responses`, kept as
    /// written for `kobana schema`, does not resolve.
    pub single_record: bool,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub location: ParameterLocation,
    pub required: bool,
    pub description: Option<String>,
    pub schema: Option<serde_json::Value>,
    pub example: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterLocation {
    Query,
    Path,
    Header,
    Cookie,
}

/// A resolved CLI command derived from an OpenAPI path + method
#[derive(Debug, Clone)]
pub struct ResolvedEndpoint {
    pub http_method: HttpMethod,
    pub path_template: String,
    pub cli_method: String,
    pub operation: Operation,
    /// Path parameter names extracted from the template (e.g., ["id", "uid"])
    pub path_params: Vec<String>,
    /// An action of its parent resource (`inboxes recipients remove`) that
    /// used to be a resource of its own with a `create` method
    /// (`inboxes recipients remove create`). The old line still runs it.
    pub legacy_create: bool,
}

impl ResolvedEndpoint {
    /// Other names this command answers to. A top-level singleton (`me`,
    /// `organization`) was once inferred as `list`; it keeps answering to it
    /// so scripts written before the singleton rule do not break.
    pub fn aliases(&self) -> &'static [&'static str] {
        if self.http_method == HttpMethod::Get
            && self.cli_method == "get"
            && self.path_params.is_empty()
        {
            &["list"]
        } else {
            &[]
        }
    }

    /// Whether `name` calls this command, by its name or an alias.
    pub fn answers_to(&self, name: &str) -> bool {
        self.cli_method == name || self.aliases().contains(&name)
    }
}

/// Tree node for organizing endpoints into CLI subcommands
#[derive(Debug, Clone, Default)]
pub struct CommandNode {
    pub children: BTreeMap<String, CommandNode>,
    pub endpoints: Vec<ResolvedEndpoint>,
}

impl ApiSpec {
    /// Parse an OpenAPI 3.1 JSON spec
    pub fn parse(json_str: &str) -> Result<Self, KobanaError> {
        let raw: serde_json::Value =
            serde_json::from_str(json_str).map_err(|e| KobanaError::Schema(e.to_string()))?;
        Self::from_value(&raw)
    }

    /// Build from an already parsed OpenAPI document (e.g. after its text was
    /// translated with [`crate::i18n::translate_spec`])
    pub fn from_value(raw: &serde_json::Value) -> Result<Self, KobanaError> {
        let version = raw["info"]["version"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let paths_obj = raw["paths"]
            .as_object()
            .ok_or_else(|| KobanaError::Schema("missing 'paths' in spec".into()))?;

        let mut paths = BTreeMap::new();

        for (path, path_value) in paths_obj {
            let path_obj = path_value
                .as_object()
                .ok_or_else(|| KobanaError::Schema(format!("invalid path item: {path}")))?;

            let mut operations = BTreeMap::new();

            for (method_str, op_value) in path_obj {
                let method = match method_str.as_str() {
                    "get" => HttpMethod::Get,
                    "post" => HttpMethod::Post,
                    "put" => HttpMethod::Put,
                    "patch" => HttpMethod::Patch,
                    "delete" => HttpMethod::Delete,
                    _ => continue, // skip "parameters", "summary", etc.
                };

                let mut op = parse_operation(op_value)?;
                op.single_record = op
                    .responses
                    .iter()
                    .filter(|(status, _)| status.starts_with('2'))
                    .any(|(_, response)| wraps_single_record(response, Some(raw)));
                operations.insert(method, op);
            }

            if !operations.is_empty() {
                paths.insert(path.clone(), PathItem { operations });
            }
        }

        Ok(ApiSpec { version, paths })
    }

    /// Build a command tree from the spec, stripping `version_prefix`
    /// (e.g. `/v1`) from each path.
    ///
    /// All non-parameter segments become nodes in the command tree.
    /// Path parameters are skipped (they become --params values).
    /// The CLI method (list, get, create, etc.) is inferred from the HTTP method
    /// and whether the path ends with a parameter.
    ///
    /// Every Kobana product carries the version in its own spec paths, so the
    /// prefix only decides tree placement — the endpoint keeps the full path.
    pub fn build_command_tree(&self, version_prefix: &str) -> CommandNode {
        let mut root = CommandNode::default();

        for (path, path_item) in &self.paths {
            let stripped = path.strip_prefix(version_prefix).unwrap_or(path);

            let raw_segments: Vec<&str> = stripped.split('/').filter(|s| !s.is_empty()).collect();

            // Collect resource segments (non-param) and path params
            let mut resource_segments = Vec::new();
            let mut path_params = Vec::new();

            for seg in &raw_segments {
                if seg.starts_with('{') && seg.ends_with('}') {
                    let param_name = seg[1..seg.len() - 1].to_string();
                    path_params.push(param_name);
                } else {
                    resource_segments.push(seg.replace('_', "-"));
                }
            }

            // Determine if the last segment is an "action" (appears after a param)
            // e.g., /pix/{uid}/cancel → last_is_action = true, action = "cancel"
            let is_action_path = raw_segments.len() >= 2 && {
                let last = raw_segments.last().unwrap();
                let second_last = raw_segments[raw_segments.len() - 2];
                !last.starts_with('{') && second_last.starts_with('{')
            };

            // An action becomes the command name itself (`pix {uid} cancel` →
            // `cancel`), but only when it has a single operation. Paths like
            // `/inboxes/{id}/recipients` (POST + DELETE) would otherwise emit
            // the same command name twice, so the action becomes a resource
            // node and each verb gets its own inferred method underneath it.
            // A POST-only path under another route (`/holidays/sync` next to
            // `/holidays`, `/inboxes/{id}/recipients/remove` next to
            // `…/recipients`) is an action of that resource too: `holidays
            // sync`, `inboxes recipients remove` — not a resource `sync` with
            // a `create` in it, its name until the next release after 0.8.0.
            let is_nested_action = raw_segments.len() >= 2
                && path_item.operations.len() == 1
                && path_item.operations.contains_key(&HttpMethod::Post)
                && {
                    let last = raw_segments.last().unwrap();
                    let second_last = raw_segments[raw_segments.len() - 2];
                    !last.starts_with('{') && !second_last.starts_with('{')
                }
                && path
                    .rsplit_once('/')
                    .is_some_and(|(parent, _)| self.paths.contains_key(parent));
            let last_is_action =
                (is_action_path || is_nested_action) && path_item.operations.len() == 1;

            // Build endpoints for each HTTP method
            for (http_method, operation) in &path_item.operations {
                let cli_method = if last_is_action {
                    resource_segments.last().unwrap().clone()
                } else {
                    let inferred = infer_cli_method(*http_method, path, operation);
                    // A top-level singleton cannot take `get` from the item
                    // route beside it (`/data_purges` counts, `/data_purges/{id}`
                    // reads one job): two `get`s on one node would drop one
                    if inferred == "get" && path_params.is_empty() && self.has_item_get(path) {
                        "list".to_string()
                    } else {
                        inferred
                    }
                };

                let endpoint = ResolvedEndpoint {
                    http_method: *http_method,
                    path_template: path.clone(),
                    cli_method: cli_method.clone(),
                    operation: operation.clone(),
                    path_params: path_params.clone(),
                    legacy_create: is_nested_action,
                };

                // Navigate to the right node in the tree
                // For action paths, place on parent UNLESS the action name
                // already exists as a child node (sub-resource)
                let tree_segments = if last_is_action {
                    &resource_segments[..resource_segments.len() - 1]
                } else {
                    &resource_segments[..]
                };

                let mut node = &mut root;
                for seg in tree_segments {
                    node = node.children.entry(seg.clone()).or_default();
                }

                // One command per name and path: PUT and PATCH on one path
                // both infer `update`. Two paths may still meet on a name here
                // (`POST /entities/join` and `PUT /entities/{id}/join` are both
                // `join`); `resolve_conflicts` moves them apart before the
                // last pass keeps one command per name.
                if !node
                    .endpoints
                    .iter()
                    .any(|e| e.cli_method == cli_method && e.path_template == endpoint.path_template)
                {
                    node.endpoints.push(endpoint);
                }
            }
        }

        // Post-process: resolve conflicts where an endpoint's cli_method
        // matches a child node name. Move such endpoints into the child.
        resolve_conflicts(&mut root);
        dedupe_command_names(&mut root);

        root
    }
}

impl ApiSpec {
    /// Whether `path` has a GET item route right under it (`<path>/{id}`).
    fn has_item_get(&self, path: &str) -> bool {
        let base = path.trim_end_matches('/');
        self.paths.iter().any(|(other, item)| {
            other
                .strip_prefix(base)
                .and_then(|rest| rest.strip_prefix('/'))
                .is_some_and(|seg| seg.starts_with('{') && seg.ends_with('}') && !seg.contains('/'))
                && item.operations.contains_key(&HttpMethod::Get)
        })
    }
}

/// A node cannot expose the same subcommand twice: the first endpoint of a
/// name stays (paths in spec order), as before two paths could meet on it.
fn dedupe_command_names(node: &mut CommandNode) {
    let mut seen = std::collections::BTreeSet::new();
    node.endpoints.retain(|e| seen.insert(e.cli_method.clone()));
    for child in node.children.values_mut() {
        dedupe_command_names(child);
    }
}

/// Recursively resolve conflicts where endpoint cli_method matches a child node name
fn resolve_conflicts(node: &mut CommandNode) {
    // Collect endpoints that conflict with children
    let conflicting: Vec<ResolvedEndpoint> = node
        .endpoints
        .iter()
        .filter(|e| node.children.contains_key(&e.cli_method))
        .cloned()
        .collect();

    // Remove them from this node's endpoints
    node.endpoints
        .retain(|e| !node.children.contains_key(&e.cli_method));

    // Move them into the matching child with a new inferred method name
    for mut ep in conflicting {
        let child_name = ep.cli_method.clone();
        let new_method = infer_cli_method(ep.http_method, &ep.path_template, &ep.operation);
        ep.cli_method = new_method.clone();
        // Back inside its own resource it is `create` again, as it always was
        ep.legacy_create = false;
        if let Some(child) = node.children.get_mut(&child_name) {
            if !child
                .endpoints
                .iter()
                .any(|e| e.cli_method == new_method && e.http_method == ep.http_method)
            {
                child.endpoints.push(ep);
            }
        }
    }

    // Recurse into children
    for child in node.children.values_mut() {
        resolve_conflicts(child);
    }
}

/// Query parameters that only a collection takes
const PAGINATION_PARAMS: &[&str] = &["page", "per_page", "cursor", "limit"];

/// Infer the CLI method name from the HTTP method, path and operation.
///
/// A GET without path parameters lists. With them it reads one thing — an item
/// (`/webhooks/{id}`) or a singleton under a parent (`/companies/{id}/fiscal`)
/// — unless it is a collection under a parent (`/webhooks/{id}/deliveries`),
/// which the path alone cannot tell from a singleton. Pagination parameters
/// can: a GET that takes them, on a path not ending in a parameter, lists.
///
/// A top-level GET without pagination may still be a singleton (`/v1/me`,
/// `/v1/organization`: the token's own record, no id). The response tells it
/// apart: an envelope whose `data` is an object, not an array, reads one thing.
fn infer_cli_method(http_method: HttpMethod, path: &str, operation: &Operation) -> String {
    let ends_with_param = path
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .is_some_and(|seg| seg.starts_with('{') && seg.ends_with('}'));
    let has_path_params = path.contains('{');
    let paginated = operation.parameters.iter().any(|p| {
        p.location == ParameterLocation::Query && PAGINATION_PARAMS.contains(&p.name.as_str())
    });
    let lists = if has_path_params {
        !ends_with_param && paginated
    } else {
        paginated || !returns_single_record(operation)
    };
    match (http_method, lists) {
        (HttpMethod::Get, true) => "list".to_string(),
        (HttpMethod::Get, false) => "get".to_string(),
        (HttpMethod::Post, _) => "create".to_string(),
        (HttpMethod::Put, _) => "update".to_string(),
        (HttpMethod::Patch, _) => "update".to_string(),
        (HttpMethod::Delete, _) => "delete".to_string(),
    }
}

/// Whether the 2xx JSON response wraps one record: `{ data: { … } }`, with
/// `data` an object (inline or a `$ref`) rather than an array.
fn returns_single_record(operation: &Operation) -> bool {
    operation.single_record
        || operation
            .responses
            .iter()
            .filter(|(status, _)| status.starts_with('2'))
            .any(|(_, response)| wraps_single_record(response, None))
}

/// Whether one response's JSON schema is a `{ data: <object> }` envelope. The
/// schema itself may be a local `$ref`, followed through `doc` when given.
fn wraps_single_record(response: &serde_json::Value, doc: Option<&serde_json::Value>) -> bool {
    let Some(mut schema) = response.pointer("/content/application~1json/schema") else {
        return false;
    };
    // Bounded: a reference cycle in a malformed spec must not hang the CLI
    for _ in 0..8 {
        let Some(target) = schema.get("$ref").and_then(|r| r.as_str()) else {
            break;
        };
        let Some(resolved) = doc
            .zip(target.strip_prefix('#'))
            .and_then(|(doc, pointer)| doc.pointer(pointer))
        else {
            return false;
        };
        schema = resolved;
    }
    let Some(data) = schema.pointer("/properties/data") else {
        return false;
    };
    let is_array = match data.get("type") {
        Some(serde_json::Value::String(t)) => t == "array",
        Some(serde_json::Value::Array(ts)) => ts.iter().any(|t| t == "array"),
        _ => false,
    };
    !is_array && (data.get("$ref").is_some() || data.get("type").is_some())
}

fn parse_operation(value: &serde_json::Value) -> Result<Operation, KobanaError> {
    let summary = value["summary"].as_str().map(String::from);
    let description = value["description"].as_str().map(String::from);
    let tags = value["tags"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let parameters = value["parameters"]
        .as_array()
        .map(|arr| arr.iter().filter_map(parse_parameter).collect())
        .unwrap_or_default();

    let request_body = value.get("requestBody").cloned();

    let responses = value["responses"]
        .as_object()
        .map(|obj| {
            obj.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        })
        .unwrap_or_default();

    Ok(Operation {
        summary,
        description,
        parameters,
        request_body,
        responses,
        tags,
        single_record: false,
    })
}

fn parse_parameter(value: &serde_json::Value) -> Option<Parameter> {
    let name = value["name"].as_str()?.to_string();
    let location = match value["in"].as_str()? {
        "query" => ParameterLocation::Query,
        "path" => ParameterLocation::Path,
        "header" => ParameterLocation::Header,
        "cookie" => ParameterLocation::Cookie,
        _ => return None,
    };
    let required = value["required"].as_bool().unwrap_or(false);
    let description = value["description"].as_str().map(String::from);
    let schema = value.get("schema").cloned();
    let example = value.get("example").cloned();

    Some(Parameter {
        name,
        location,
        required,
        description,
        schema,
        example,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every Kobana product spec carries the version in its own paths
    /// (`/v1/subscriptions`, `/v2/charge/pix`).
    const SPEC: &str = r#"{
        "info": {"version": "1.0"},
        "paths": {
            "/v1/subscriptions": {"get": {"responses": {}}, "post": {"responses": {}}},
            "/v1/subscriptions/{id}/pause": {"post": {"responses": {}}}
        }
    }"#;

    #[test]
    fn version_prefix_is_stripped_from_the_tree_but_kept_in_the_path() {
        let spec = ApiSpec::parse(SPEC).unwrap();
        let tree = spec.build_command_tree("/v1");

        // `v1` must not become a tree node...
        assert!(tree.children.contains_key("subscriptions"));
        assert!(!tree.children.contains_key("v1"));

        // ...but the request path has to keep it
        let mut got: Vec<(String, String)> = tree.children["subscriptions"]
            .endpoints
            .iter()
            .map(|e| (e.cli_method.clone(), e.path_template.clone()))
            .collect();
        got.sort();

        let mut want = vec![
            ("create".to_string(), "/v1/subscriptions".to_string()),
            ("list".to_string(), "/v1/subscriptions".to_string()),
            ("pause".to_string(), "/v1/subscriptions/{id}/pause".to_string()),
        ];
        want.sort();
        assert_eq!(got, want);
    }

    #[test]
    fn a_non_matching_prefix_leaves_paths_untouched() {
        let spec = ApiSpec::parse(SPEC).unwrap();
        let tree = spec.build_command_tree("/v2");

        // Nothing is stripped, so `v1` shows up as a node instead
        assert!(tree.children.contains_key("v1"));
    }

    /// `/resource/{id}/action` with one verb: the action IS the command.
    /// With several verbs it becomes a node, or clap would see the same
    /// subcommand name twice and panic at startup.
    const ACTION_SPEC: &str = r#"{
        "info": {"version": "1.0"},
        "paths": {
            "/v1/webhooks/{id}/test": {"post": {"responses": {}}},
            "/v1/inboxes/{id}/recipients": {"post": {"responses": {}}, "delete": {"responses": {}}}
        }
    }"#;

    #[test]
    fn single_verb_action_becomes_the_command_name() {
        let tree = ApiSpec::parse(ACTION_SPEC)
            .unwrap()
            .build_command_tree("/v1");

        let webhooks = &tree.children["webhooks"];
        let methods: Vec<&str> = webhooks.endpoints.iter().map(|e| e.cli_method.as_str()).collect();
        assert_eq!(methods, vec!["test"]);
        assert!(!webhooks.children.contains_key("test"));
    }

    #[test]
    fn multi_verb_action_becomes_a_resource_node() {
        let tree = ApiSpec::parse(ACTION_SPEC)
            .unwrap()
            .build_command_tree("/v1");

        let inboxes = &tree.children["inboxes"];
        // The action must not sit on the parent twice
        assert!(inboxes.endpoints.is_empty());

        let recipients = inboxes.children.get("recipients").expect("recipients node");
        let mut methods: Vec<&str> =
            recipients.endpoints.iter().map(|e| e.cli_method.as_str()).collect();
        methods.sort();
        assert_eq!(methods, vec!["create", "delete"]);
        assert!(recipients
            .endpoints
            .iter()
            .all(|e| e.path_template == "/v1/inboxes/{id}/recipients"));
    }

    #[test]
    fn test_infer_cli_method() {
        let plain = Operation::default();
        let paginated = Operation {
            parameters: vec![Parameter {
                name: "page".into(),
                location: ParameterLocation::Query,
                required: false,
                description: None,
                schema: None,
                example: None,
            }],
            ..Operation::default()
        };

        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/webhooks", &plain), "list");
        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/webhooks/{id}", &plain), "get");
        assert_eq!(infer_cli_method(HttpMethod::Post, "/v1/webhooks", &plain), "create");
        assert_eq!(infer_cli_method(HttpMethod::Delete, "/v1/webhooks/{uid}", &plain), "delete");

        // Under a parent, pagination tells a collection from a singleton
        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/webhooks/{id}/deliveries", &paginated), "list");
        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/companies/{id}/fiscal", &plain), "get");
        // An item is read one at a time even if the spec lists `page` on it
        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/webhooks/{id}", &paginated), "get");

        // A top-level singleton (the token's own record) is told by its
        // response: `data` an object reads one thing, `data` an array lists
        let with_data = |data: serde_json::Value| Operation {
            responses: [(
                "200".to_string(),
                serde_json::json!({
                    "content": {"application/json": {"schema": {
                        "type": "object", "properties": {"data": data}
                    }}}
                }),
            )]
            .into(),
            ..Operation::default()
        };
        let single_ref = with_data(serde_json::json!({"$ref": "#/components/schemas/Organization"}));
        let single_inline = with_data(serde_json::json!({"type": "object", "properties": {}}));
        let many = with_data(serde_json::json!({"type": "array", "items": {}}));
        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/organization", &single_ref), "get");
        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/me", &single_inline), "get");
        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/organization/slack_installations", &many), "list");
        // No usable response schema keeps the old rule: a top-level GET lists
        assert_eq!(infer_cli_method(HttpMethod::Get, "/v1/webhooks", &plain), "list");
    }

    #[test]
    fn a_singleton_whose_envelope_is_a_ref_reads_one_record() {
        // billing, dashboards and finance publish the envelope as a component
        let spec = ApiSpec::parse(
            r##"{
            "paths": {
                "/v1/organization": {"get": {"responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/OrganizationResponse"}}}}}}},
                "/v1/webhooks": {"get": {"responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/WebhookList"}}}}}}},
                "/v1/loop": {"get": {"responses": {"200": {"content": {"application/json": {
                    "schema": {"$ref": "#/components/schemas/Loop"}}}}}}}
            },
            "components": {"schemas": {
                "OrganizationResponse": {"type": "object", "properties": {
                    "data": {"$ref": "#/components/schemas/Organization"}}},
                "Organization": {"type": "object"},
                "WebhookList": {"type": "object", "properties": {
                    "data": {"type": "array", "items": {}}}},
                "Loop": {"$ref": "#/components/schemas/Loop"}
            }}
        }"##,
        )
        .unwrap();
        let tree = spec.build_command_tree("/v1");
        let method = |resource: &str| tree.children[resource].endpoints[0].cli_method.clone();
        assert_eq!(method("organization"), "get");
        assert_eq!(method("webhooks"), "list");
        // A reference cycle is bounded and falls back to the path rule
        assert_eq!(method("loop"), "list");
    }

    #[test]
    fn a_post_only_subpath_is_an_action_of_its_parent() {
        let spec = ApiSpec::parse(
            r#"{"paths": {
                "/v1/inboxes/{id}/recipients": {"post": {"responses": {}}},
                "/v1/inboxes/{id}/recipients/remove": {"post": {"responses": {}}},
                "/v1/holidays": {"get": {"responses": {}}, "post": {"responses": {}}},
                "/v1/holidays/sync": {"post": {"responses": {}}},
                "/v1/data_sources": {"get": {"responses": {}}},
                "/v1/data_sources/openapi": {"post": {"responses": {}}},
                "/v1/data_sources/openapi/analyze": {"post": {"responses": {}}},
                "/v1/reports/monthly": {"post": {"responses": {}}}
            }}"#,
        )
        .unwrap();
        let tree = spec.build_command_tree("/v1");
        let methods = |node: &CommandNode| -> Vec<(String, bool)> {
            let mut m: Vec<_> = node.endpoints.iter().map(|e| (e.cli_method.clone(), e.legacy_create)).collect();
            m.sort();
            m
        };

        let recipients = &tree.children["inboxes"].children["recipients"];
        assert_eq!(methods(recipients), [("create".to_string(), false), ("remove".to_string(), true)]);
        assert!(recipients.children.is_empty());
        let holidays = &tree.children["holidays"];
        assert!(methods(holidays).contains(&("sync".to_string(), true)));
        // A name that is also a resource with children stays that resource,
        // and its own sub-action is an action of it
        let openapi = &tree.children["data-sources"].children["openapi"];
        assert_eq!(methods(openapi), [("analyze".to_string(), true), ("create".to_string(), false)]);
        // Two paths meeting on one name are moved apart, none is lost
        let spec2 = ApiSpec::parse(
            r#"{"paths": {
                "/v1/entities": {"get": {"responses": {}}},
                "/v1/entities/join": {"post": {"responses": {}}},
                "/v1/entities/join/preview": {"post": {"responses": {}}},
                "/v1/entities/{id}/join": {"put": {"responses": {}}}
            }}"#,
        )
        .unwrap()
        .build_command_tree("/v1");
        assert_eq!(
            methods(&spec2.children["entities"].children["join"]),
            [("create".to_string(), false), ("preview".to_string(), true), ("update".to_string(), false)]
        );
        // Without a parent route it is still a resource of its own
        assert_eq!(methods(&tree.children["reports"].children["monthly"]), [("create".to_string(), false)]);
    }

    #[test]
    fn a_singleton_beside_an_item_route_keeps_list() {
        // dashboards: `/data_purges` answers counts, `/data_purges/{id}` one job
        let spec = ApiSpec::parse(
            r#"{"paths": {
                "/v1/data_purges": {"get": {"responses": {"200": {"content": {"application/json": {
                    "schema": {"type": "object", "properties": {"data": {"type": "object"}}}}}}}}},
                "/v1/data_purges/{id}": {"get": {"responses": {}}}
            }}"#,
        )
        .unwrap();
        let tree = spec.build_command_tree("/v1");
        let mut methods: Vec<String> = tree.children["data-purges"]
            .endpoints
            .iter()
            .map(|e| e.cli_method.clone())
            .collect();
        methods.sort();
        assert_eq!(methods, ["get", "list"]);
    }
}
