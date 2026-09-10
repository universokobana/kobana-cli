use kobana::error::KobanaError;
use kobana::spec::{CommandNode, ParameterLocation, ResolvedEndpoint};

use crate::product::{self, LoadedProduct};

/// Handle the `kobana schema` command
pub fn handle_schema(
    matches: &clap::ArgMatches,
    products: &[LoadedProduct],
) -> Result<(), KobanaError> {
    let list = matches.get_flag("list");

    if let Some(endpoint_path) = matches.get_one::<String>("endpoint") {
        return show_endpoint_schema(endpoint_path, products);
    }

    if list {
        return list_products(products);
    }

    Err(KobanaError::Validation(
        "Usage: kobana schema <produto>.<servico>.<recurso>.<metodo> or kobana schema --list".into(),
    ))
}

fn list_products(products: &[LoadedProduct]) -> Result<(), KobanaError> {
    let mut out = serde_json::Map::new();

    for loaded in products {
        let mut services = serde_json::Map::new();
        for (name, service) in &loaded.services {
            services.insert(
                name.clone(),
                serde_json::json!({ "resources": list_resources(&service.tree) }),
            );
        }

        out.insert(
            loaded.product.slug.to_string(),
            serde_json::json!({
                "description": loaded.product.about,
                "services": services,
            }),
        );
    }

    println!("{}", serde_json::to_string_pretty(&out)?);
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
    products: &[LoadedProduct],
) -> Result<(), KobanaError> {
    // Parse a path like "banking.charge.pix.create" or "banking.v1.bank-billets.list"
    let parts: Vec<&str> = endpoint_path.split('.').collect();
    if parts.len() < 3 || !product::is_product(parts[0]) {
        return Err(KobanaError::Validation(format!(
            "invalid endpoint path '{endpoint_path}'. Use format: product.service.resource.method (e.g., banking.charge.pix.create)"
        )));
    }

    let (product_name, service_name, method_name) =
        (parts[0], parts[1], *parts.last().unwrap());
    let resource_parts = &parts[2..parts.len() - 1];

    let loaded = product::find(products, product_name)
        .ok_or_else(|| KobanaError::Schema(format!("product '{product_name}' not found")))?;
    let service = loaded.service(service_name).ok_or_else(|| {
        KobanaError::Schema(format!(
            "service '{service_name}' not found in product '{product_name}'"
        ))
    })?;

    // Walk the same tree the CLI dispatches on, so schema output can never
    // drift from what the commands actually accept
    let mut node = service.tree.as_ref();
    for part in resource_parts {
        node = node.children.get(*part).ok_or_else(|| {
            KobanaError::Schema(format!("endpoint '{endpoint_path}' not found"))
        })?;
    }

    let endpoint = node
        .endpoints
        .iter()
        .find(|e| e.cli_method == method_name)
        .ok_or_else(|| KobanaError::Schema(format!("endpoint '{endpoint_path}' not found")))?;

    print_endpoint(loaded, endpoint)
}

fn print_endpoint(
    loaded: &LoadedProduct,
    endpoint: &ResolvedEndpoint,
) -> Result<(), KobanaError> {
    let query_params: Vec<serde_json::Value> = endpoint
        .operation
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
        "product": loaded.product.slug,
        "method": endpoint.http_method.as_str(),
        "path": endpoint.path_template,
        "path_params": endpoint.path_params,
        "summary": endpoint.operation.summary,
        "description": endpoint.operation.description,
        "parameters": query_params,
        "request_body": endpoint.operation.request_body,
        "responses": endpoint.operation.responses,
    });

    println!("{}", serde_json::to_string_pretty(&schema_output)?);
    Ok(())
}
