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
        out.insert(
            loaded.product.slug.to_string(),
            serde_json::json!({
                "description": loaded.product.about,
                "resources": list_resources(&loaded.tree),
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
    // Parse a path like "banking.charge.pix.create" or "inbox.emails.list"
    let parts: Vec<&str> = endpoint_path.split('.').collect();
    if parts.len() < 3 || !product::is_product(parts[0]) {
        return Err(KobanaError::Validation(format!(
            "invalid endpoint path '{endpoint_path}'. Use format: product.resource.method (e.g., banking.charge.pix.create)"
        )));
    }

    let (product_name, method_name) = (parts[0], *parts.last().unwrap());
    let resource_parts = &parts[1..parts.len() - 1];

    let loaded = product::find(products, product_name)
        .ok_or_else(|| KobanaError::Schema(format!("product '{product_name}' not found")))?;

    // API versions stopped being path segments; say so rather than reporting
    // a missing endpoint for every pre-existing `banking.v1.…` path
    if let Some(version) = resource_parts.first().filter(|p| is_version(p)) {
        let without: Vec<&str> = std::iter::once(product_name)
            .chain(resource_parts[1..].iter().copied())
            .chain(std::iter::once(method_name))
            .collect();
        return Err(KobanaError::Validation(format!(
            "'{version}' is not part of endpoint paths — API versions are resolved from the spec. Use '{}'",
            without.join(".")
        )));
    }

    // Walk the same tree the CLI dispatches on, so schema output can never
    // drift from what the commands actually accept
    let mut node = &loaded.tree;
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

/// `v1`, `v2`, … — a leftover API version in a path
fn is_version(part: &str) -> bool {
    part.strip_prefix('v')
        .is_some_and(|rest| !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_api_version_segments() {
        for part in ["v1", "v2", "v10"] {
            assert!(is_version(part), "{part} should read as a version");
        }
        // Resources that merely start with a v must not be mistaken for one
        for part in ["v", "va", "pix", "vouchers", "v1x", ""] {
            assert!(!is_version(part), "{part} should not read as a version");
        }
    }

    #[test]
    fn old_versioned_paths_get_a_migration_error() {
        let products = crate::product::load_all().unwrap();
        let err = show_endpoint_schema("banking.v1.bank-billets.list", &products).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("banking.bank-billets.list"), "{msg}");
    }
}
