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

#[derive(Debug, Clone)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
    pub request_body: Option<serde_json::Value>,
    pub responses: BTreeMap<String, serde_json::Value>,
    pub tags: Vec<String>,
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

                let op = parse_operation(op_value)?;
                operations.insert(method, op);
            }

            if !operations.is_empty() {
                paths.insert(path.clone(), PathItem { operations });
            }
        }

        Ok(ApiSpec { version, paths })
    }

    /// Build a command tree from the spec, stripping the version prefix.
    ///
    /// All non-parameter segments become nodes in the command tree.
    /// Path parameters are skipped (they become --params values).
    /// The CLI method (list, get, create, etc.) is inferred from the HTTP method
    /// and whether the path ends with a parameter.
    pub fn build_command_tree(&self) -> CommandNode {
        let mut root = CommandNode::default();

        for (path, path_item) in &self.paths {
            // Strip version prefix: /v1/... or /v2/...
            let stripped = path
                .strip_prefix("/v1/")
                .or_else(|| path.strip_prefix("/v2/"))
                .unwrap_or(path);

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
            let last_is_action = raw_segments.len() >= 2 && {
                let last = raw_segments.last().unwrap();
                let second_last = raw_segments[raw_segments.len() - 2];
                !last.starts_with('{') && second_last.starts_with('{')
            };

            // Build endpoints for each HTTP method
            for (http_method, operation) in &path_item.operations {
                let cli_method = if last_is_action {
                    resource_segments.last().unwrap().clone()
                } else {
                    infer_cli_method(*http_method, &path_params)
                };

                let endpoint = ResolvedEndpoint {
                    http_method: *http_method,
                    path_template: path.clone(),
                    cli_method: cli_method.clone(),
                    operation: operation.clone(),
                    path_params: path_params.clone(),
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

                // Deduplicate: don't add if same cli_method already exists
                if !node.endpoints.iter().any(|e| e.cli_method == cli_method && e.http_method == *http_method) {
                    node.endpoints.push(endpoint);
                }
            }
        }

        // Post-process: resolve conflicts where an endpoint's cli_method
        // matches a child node name. Move such endpoints into the child.
        resolve_conflicts(&mut root);

        root
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
        let new_method = infer_cli_method(ep.http_method, &ep.path_params);
        ep.cli_method = new_method.clone();
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

/// Infer the CLI method name from HTTP method and path params
fn infer_cli_method(http_method: HttpMethod, path_params: &[String]) -> String {
    match (http_method, path_params.is_empty()) {
        (HttpMethod::Get, true) => "list".to_string(),
        (HttpMethod::Get, false) => "get".to_string(),
        (HttpMethod::Post, _) => "create".to_string(),
        (HttpMethod::Put, _) => "update".to_string(),
        (HttpMethod::Patch, _) => "update".to_string(),
        (HttpMethod::Delete, _) => "delete".to_string(),
    }
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

    #[test]
    fn test_infer_cli_method() {
        assert_eq!(infer_cli_method(HttpMethod::Get, &[]), "list");
        assert_eq!(infer_cli_method(HttpMethod::Get, &["id".into()]), "get");
        assert_eq!(infer_cli_method(HttpMethod::Post, &[]), "create");
        assert_eq!(infer_cli_method(HttpMethod::Delete, &["uid".into()]), "delete");
    }
}
