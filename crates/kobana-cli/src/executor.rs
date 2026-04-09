use kobana::client::{ApiRequest, KobanaClient};
use kobana::error::KobanaError;
use kobana::spec::{HttpMethod, ResolvedEndpoint};
use kobana::validate::validate_identifier;

use crate::formatter::{filter_fields, format_output};

/// Execute an API request based on the resolved endpoint and CLI args
pub async fn execute(
    client: &KobanaClient,
    endpoint: &ResolvedEndpoint,
    _method_matches: &clap::ArgMatches,
    root_matches: &clap::ArgMatches,
) -> Result<(), KobanaError> {
    let params_json = root_matches
        .get_one::<String>("params")
        .map(|s| serde_json::from_str::<serde_json::Value>(s))
        .transpose()
        .map_err(|e| KobanaError::Validation(format!("invalid --params JSON: {e}")))?;

    let body_json = root_matches
        .get_one::<String>("json")
        .map(|s| serde_json::from_str::<serde_json::Value>(s))
        .transpose()
        .map_err(|e| KobanaError::Validation(format!("invalid --json JSON: {e}")))?;

    let fields = root_matches.get_one::<String>("fields").cloned();
    let dry_run = root_matches.get_flag("dry-run");
    let verbose = root_matches.get_flag("verbose");
    let output_path = root_matches.get_one::<String>("output").cloned();
    let output_format = root_matches
        .get_one::<String>("output-format")
        .map(|s| s.as_str())
        .unwrap_or("json");

    // Resolve path parameters from --params
    let path = resolve_path_template(&endpoint.path_template, &params_json)?;

    // Generate idempotency key for mutations
    let idempotency_key = if matches!(
        endpoint.http_method,
        HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch
    ) {
        root_matches
            .get_one::<String>("idempotency-key")
            .cloned()
            .or_else(|| {
                if endpoint.http_method == HttpMethod::Post {
                    Some(uuid::Uuid::new_v4().to_string())
                } else {
                    None
                }
            })
    } else {
        None
    };

    // Strip path params from query params (they're already in the URL)
    let query_params = params_json.map(|mut params| {
        if let Some(obj) = params.as_object_mut() {
            for param_name in &endpoint.path_params {
                obj.remove(param_name);
            }
        }
        params
    });

    let api_request = ApiRequest {
        method: endpoint.http_method,
        path,
        query_params,
        body: body_json,
        idempotency_key,
    };

    // Dry run: show the request without executing
    if dry_run {
        let dry_run_output = serde_json::json!({
            "dry_run": true,
            "method": endpoint.http_method.as_str(),
            "url": format!("{}{}", client.base_url(), api_request.path),
            "query_params": api_request.query_params,
            "body": api_request.body,
            "idempotency_key": api_request.idempotency_key,
        });
        println!("{}", serde_json::to_string_pretty(&dry_run_output)?);
        return Ok(());
    }

    if verbose {
        eprintln!(
            ">> {} {}{}",
            endpoint.http_method,
            client.base_url(),
            api_request.path
        );
        if let Some(body) = &api_request.body {
            eprintln!(">> Body: {}", serde_json::to_string(body)?);
        }
    }

    let response = client.execute(&api_request).await?;

    if verbose {
        eprintln!("<< Status: {}", response.status);
        for (key, value) in response.headers.iter() {
            if let Ok(v) = value.to_str() {
                eprintln!("<< {}: {}", key, v);
            }
        }
    }

    // Apply field mask
    let output = if let Some(ref fields_str) = fields {
        filter_fields(&response.body, fields_str)
    } else {
        response.body.clone()
    };

    // Format and output
    let formatted = format_output(&output, output_format)?;

    if let Some(path) = output_path {
        std::fs::write(&path, &formatted)
            .map_err(|e| KobanaError::Internal(format!("failed to write to {path}: {e}")))?;
        eprintln!("Response saved to {path}");
    } else {
        println!("{formatted}");
    }

    Ok(())
}

/// Replace path template parameters with values from --params
fn resolve_path_template(
    template: &str,
    params: &Option<serde_json::Value>,
) -> Result<String, KobanaError> {
    let mut path = template.to_string();

    // Find all {param} placeholders
    let mut start = 0;
    while let Some(open) = path[start..].find('{') {
        let open = start + open;
        if let Some(close) = path[open..].find('}') {
            let close = open + close;
            let param_name = &path[open + 1..close];

            // Look up value in params
            let value = params
                .as_ref()
                .and_then(|p| p.get(param_name))
                .or_else(|| {
                    // Try with underscored variant (e.g., pix_uid -> pix_uid)
                    params.as_ref().and_then(|p| {
                        // Also try common aliases: uid, id
                        if param_name.ends_with("_uid") || param_name.ends_with("_id") {
                            p.get("uid").or_else(|| p.get("id"))
                        } else {
                            None
                        }
                    })
                })
                .ok_or_else(|| {
                    KobanaError::Validation(format!(
                        "missing required path parameter '{param_name}' in --params"
                    ))
                })?;

            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                other => other.to_string().trim_matches('"').to_string(),
            };

            validate_identifier(&value_str, param_name)?;

            path = format!("{}{}{}", &path[..open], value_str, &path[close + 1..]);
            start = open + value_str.len();
        } else {
            break;
        }
    }

    Ok(path)
}
