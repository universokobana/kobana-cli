use kobana::error::KobanaError;
use kobana::spec::{ApiSpec, CommandNode, ParameterLocation};

/// Handle the `kobana schema` command
pub fn handle_schema(
    matches: &clap::ArgMatches,
    v1_spec: &ApiSpec,
    v2_spec: &ApiSpec,
    v1_tree: &CommandNode,
    v2_tree: &CommandNode,
) -> Result<(), KobanaError> {
    let list = matches.get_flag("list");

    if list {
        // If no endpoint specified, list all services
        if matches.get_one::<String>("endpoint").is_none() {
            return list_services(v1_tree, v2_tree);
        }
    }

    if let Some(endpoint_path) = matches.get_one::<String>("endpoint") {
        return show_endpoint_schema(endpoint_path, v1_spec, v2_spec);
    }

    if list {
        return list_services(v1_tree, v2_tree);
    }

    Err(KobanaError::Validation(
        "Usage: kobana schema <endpoint> or kobana schema --list".into(),
    ))
}

fn list_services(v1_tree: &CommandNode, v2_tree: &CommandNode) -> Result<(), KobanaError> {
    let mut services = serde_json::json!({
        "v1": {
            "resources": list_resources(v1_tree),
        },
    });

    for (name, node) in &v2_tree.children {
        services[name] = serde_json::json!({
            "resources": list_resources(node),
        });
    }

    println!("{}", serde_json::to_string_pretty(&services)?);
    Ok(())
}

fn list_resources(node: &CommandNode) -> Vec<serde_json::Value> {
    let mut resources = Vec::new();

    // Direct endpoints (methods on this node)
    if !node.endpoints.is_empty() {
        let methods: Vec<&str> = node.endpoints.iter().map(|e| e.cli_method.as_str()).collect();
        resources.push(serde_json::json!({
            "methods": methods,
        }));
    }

    // Child resources
    for (name, child) in &node.children {
        let methods: Vec<String> = collect_methods(child);
        resources.push(serde_json::json!({
            "name": name,
            "methods": methods,
        }));
    }

    resources
}

fn collect_methods(node: &CommandNode) -> Vec<String> {
    let mut methods: Vec<String> = node
        .endpoints
        .iter()
        .map(|e| e.cli_method.clone())
        .collect();

    for (child_name, child) in &node.children {
        for m in collect_methods(child) {
            methods.push(format!("{child_name}.{m}"));
        }
    }

    methods
}

fn show_endpoint_schema(
    endpoint_path: &str,
    v1_spec: &ApiSpec,
    v2_spec: &ApiSpec,
) -> Result<(), KobanaError> {
    // Parse endpoint path like "charge.pix.create" or "v1.bank-billets.list"
    let parts: Vec<&str> = endpoint_path.split('.').collect();
    if parts.len() < 2 {
        return Err(KobanaError::Validation(format!(
            "invalid endpoint path '{endpoint_path}'. Use format: service.resource.method (e.g., charge.pix.create)"
        )));
    }

    let method_name = parts.last().unwrap();
    let resource_parts = &parts[..parts.len() - 1];

    // Determine which spec to search
    let spec = if resource_parts[0] == "v1" {
        v1_spec
    } else {
        v2_spec
    };

    // Build the API path pattern to search for
    let search_segments: Vec<String> = if resource_parts[0] == "v1" {
        resource_parts[1..]
            .iter()
            .map(|s| s.replace('-', "_"))
            .collect()
    } else {
        resource_parts.iter().map(|s| s.replace('-', "_")).collect()
    };

    // Search for matching endpoint
    let version_prefix = if resource_parts[0] == "v1" {
        "/v1"
    } else {
        "/v2"
    };

    for (api_path, path_item) in &spec.paths {
        if !api_path.starts_with(version_prefix) {
            continue;
        }

        let stripped = &api_path[version_prefix.len()..];
        let path_segments: Vec<&str> = stripped
            .split('/')
            .filter(|s| !s.is_empty() && !s.starts_with('{'))
            .collect();

        if path_segments.len() != search_segments.len() {
            continue;
        }

        let matches = path_segments
            .iter()
            .zip(search_segments.iter())
            .all(|(a, b)| *a == b.as_str());

        if !matches {
            continue;
        }

        // Find the operation matching the method name
        for (http_method, operation) in &path_item.operations {
            let inferred = infer_method_name(http_method, api_path);
            if inferred == *method_name {
                let query_params: Vec<serde_json::Value> = operation
                    .parameters
                    .iter()
                    .filter(|p| p.location == ParameterLocation::Query)
                    .map(|p| {
                        serde_json::json!({
                            "name": p.name,
                            "required": p.required,
                            "description": p.description,
                            "schema": p.schema,
                        })
                    })
                    .collect();

                let schema_output = serde_json::json!({
                    "method": http_method.as_str(),
                    "path": api_path,
                    "summary": operation.summary,
                    "description": operation.description,
                    "parameters": query_params,
                    "request_body": operation.request_body,
                    "responses": operation.responses,
                });

                println!("{}", serde_json::to_string_pretty(&schema_output)?);
                return Ok(());
            }
        }
    }

    Err(KobanaError::Schema(format!(
        "endpoint '{endpoint_path}' not found"
    )))
}

fn infer_method_name(http_method: &kobana::spec::HttpMethod, path: &str) -> String {
    use kobana::spec::HttpMethod;

    // Check for action suffix (e.g., /cancel, /approve)
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if let Some(last) = segments.last() {
        if !last.starts_with('{') && segments.len() > 2 {
            // Check if the previous segment is a parameter
            if segments.len() >= 2 {
                let prev = segments[segments.len() - 2];
                if prev.starts_with('{') {
                    return last.replace('_', "-");
                }
            }
        }
    }

    let has_path_param = path.contains('{');
    match (http_method, has_path_param) {
        (HttpMethod::Get, false) => "list".to_string(),
        (HttpMethod::Get, true) => "get".to_string(),
        (HttpMethod::Post, _) => "create".to_string(),
        (HttpMethod::Put, _) => "update".to_string(),
        (HttpMethod::Patch, _) => "update".to_string(),
        (HttpMethod::Delete, _) => "delete".to_string(),
    }
}
