use kobana::client::{ApiRequest, KobanaClient};
use kobana::error::KobanaError;
use kobana::{tr, trf};
use kobana::spec::{HttpMethod, ParameterLocation, ResolvedEndpoint};
use kobana::validate::validate_identifier;

use crate::formatter::{filter_fields, format_output};
use crate::pagination;
use crate::validate;

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
        .map_err(|e| KobanaError::Validation(trf!(
            pt: "JSON inválido em --params: {e}",
            en: "invalid --params JSON: {e}",
            es: "JSON inválido en --params: {e}",
        )))?;

    let body_json = root_matches
        .get_one::<String>("json")
        .map(|s| serde_json::from_str::<serde_json::Value>(s))
        .transpose()
        .map_err(|e| KobanaError::Validation(trf!(
            pt: "JSON inválido em --json: {e}",
            en: "invalid --json JSON: {e}",
            es: "JSON inválido en --json: {e}",
        )))?;

    // Validate inputs
    if let Some(ref params) = params_json {
        validate::validate_params(params)?;
    }
    if let Some(ref body) = body_json {
        validate::validate_body(body)?;
    }

    let fields = root_matches.get_one::<String>("fields").cloned();
    let dry_run = root_matches.get_flag("dry-run");
    let verbose = root_matches.get_flag("verbose");
    let output_path = root_matches.get_one::<String>("output").cloned();
    let output_format = root_matches
        .get_one::<String>("output-format")
        .map(|s| s.as_str())
        .unwrap_or("json");

    let page_all = root_matches.get_flag("page-all");
    let page_limit: usize = root_matches
        .get_one::<String>("page-limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let page_delay: u64 = root_matches
        .get_one::<String>("page-delay")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    // Resolve path parameters from --params
    let path = resolve_path_template(&endpoint.path_template, &params_json)?;

    // Header parameters travel in --params too; move them out of the query
    let mut params_json = params_json;
    let headers = take_header_params(endpoint, &mut params_json)?;

    // Handle auto-pagination (skip if dry-run)
    if page_all && !dry_run && endpoint.http_method == HttpMethod::Get {
        // Strip path params from query params
        let query_params = params_json.map(|mut params| {
            if let Some(obj) = params.as_object_mut() {
                for param_name in &endpoint.path_params {
                    obj.remove(param_name);
                }
            }
            params
        });

        return pagination::paginate_all(
            client,
            endpoint,
            &path,
            &query_params,
            &headers,
            page_limit,
            page_delay,
            &fields,
            output_format,
            verbose,
        )
        .await;
    }

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
        headers,
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
            "headers": client
                .request_headers(&api_request)
                .into_iter()
                .map(|(name, value)| (name, serde_json::Value::String(value)))
                .collect::<serde_json::Map<_, _>>(),
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
            eprintln!(
                ">> {} {}",
                tr!(pt: "Corpo:", en: "Body:", es: "Cuerpo:"),
                serde_json::to_string(body)?
            );
        }
    }

    let response = client.execute(&api_request).await?;

    if verbose {
        eprintln!(
            "<< {} {}",
            tr!(pt: "Status:", en: "Status:", es: "Estado:"),
            response.status
        );
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
            .map_err(|e| {
                KobanaError::Internal(trf!(
                    pt: "não foi possível gravar em {path}: {e}",
                    en: "failed to write to {path}: {e}",
                    es: "no se pudo guardar en {path}: {e}",
                ))
            })?;
        eprintln!(
            "{}",
            trf!(
                pt: "Resposta salva em {path}",
                en: "Response saved to {path}",
                es: "Respuesta guardada en {path}",
            )
        );
    } else {
        println!("{formatted}");
    }

    Ok(())
}

/// Headers the client always sets itself; a spec declaring them as parameters
/// must not make the CLI send them twice.
pub(crate) const CLIENT_MANAGED_HEADERS: &[&str] = &["user-agent", "x-idempotency-key", "authorization"];

/// Remove the endpoint's header parameters from `--params` and return them as
/// headers. Names match case-insensitively, as HTTP header names do.
fn take_header_params(
    endpoint: &ResolvedEndpoint,
    params: &mut Option<serde_json::Value>,
) -> Result<Vec<(String, String)>, KobanaError> {
    let Some(obj) = params.as_mut().and_then(|p| p.as_object_mut()) else {
        return Ok(Vec::new());
    };

    let mut headers = Vec::new();
    for param in &endpoint.operation.parameters {
        if param.location != ParameterLocation::Header
            || CLIENT_MANAGED_HEADERS.contains(&param.name.to_ascii_lowercase().as_str())
        {
            continue;
        }
        let Some(key) = obj.keys().find(|k| k.eq_ignore_ascii_case(&param.name)).cloned() else {
            continue;
        };
        let value = match obj.remove(&key) {
            Some(serde_json::Value::String(s)) => s,
            Some(other) => other.to_string(),
            None => continue,
        };
        if value.chars().any(char::is_control) {
            let name = &param.name;
            return Err(KobanaError::Validation(trf!(
                pt: "o parâmetro de cabeçalho '{name}' contém caracteres de controle",
                en: "header parameter '{name}' contains control characters",
                es: "el parámetro de encabezado '{name}' contiene caracteres de control",
            )));
        }
        headers.push((param.name.clone(), value));
    }
    Ok(headers)
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
                    // Also try common aliases: uid, id
                    params.as_ref().and_then(|p| {
                        if param_name.ends_with("_uid") || param_name.ends_with("_id") {
                            p.get("uid").or_else(|| p.get("id"))
                        } else {
                            None
                        }
                    })
                })
                .ok_or_else(|| {
                    KobanaError::Validation(trf!(
                        pt: "falta o parâmetro de caminho obrigatório '{param_name}' em --params",
                        en: "missing required path parameter '{param_name}' in --params",
                        es: "falta el parámetro de ruta obligatorio '{param_name}' en --params",
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

#[cfg(test)]
mod tests {
    use super::*;
    use kobana::spec::ApiSpec;

    fn endpoint() -> ResolvedEndpoint {
        const SPEC: &str = r#"{
            "info": {"version": "1.0"},
            "paths": {
                "/v1/payables": {"get": {
                    "parameters": [
                        {"name": "X-Workspace-Id", "in": "header", "required": true},
                        {"name": "X-Idempotency-Key", "in": "header"},
                        {"name": "status", "in": "query"}
                    ],
                    "responses": {}
                }}
            }
        }"#;
        let tree = ApiSpec::parse(SPEC).unwrap().build_command_tree("/v1");
        tree.children["payables"].endpoints[0].clone()
    }

    #[test]
    fn header_params_leave_the_query_and_become_headers() {
        let mut params = Some(serde_json::json!({
            "x-workspace-id": "ws-1",
            "status": "pending"
        }));
        let headers = take_header_params(&endpoint(), &mut params).unwrap();

        // Named as the spec declares it, matched case-insensitively
        assert_eq!(headers, vec![("X-Workspace-Id".to_string(), "ws-1".to_string())]);
        assert_eq!(params, Some(serde_json::json!({"status": "pending"})));
    }

    #[test]
    fn client_managed_headers_are_not_taken_from_params() {
        let mut params = Some(serde_json::json!({"X-Idempotency-Key": "k"}));
        let headers = take_header_params(&endpoint(), &mut params).unwrap();

        assert!(headers.is_empty());
        assert_eq!(params, Some(serde_json::json!({"X-Idempotency-Key": "k"})));
    }

    #[test]
    fn rejects_header_values_with_control_characters() {
        let mut params = Some(serde_json::json!({"X-Workspace-Id": "ws\r\nX-Evil: 1"}));
        let err = take_header_params(&endpoint(), &mut params).unwrap_err();
        assert!(matches!(err, KobanaError::Validation(_)));
    }

    #[test]
    fn request_headers_override_client_defaults() {
        let client = KobanaClient::new("https://api.example", "t")
            .unwrap()
            .with_default_headers(vec![
                ("X-Workspace-Id".to_string(), "from-env".to_string()),
                ("X-Other".to_string(), "kept".to_string()),
            ]);
        let request = ApiRequest {
            method: HttpMethod::Get,
            path: "/v1/payables".to_string(),
            query_params: None,
            body: None,
            idempotency_key: None,
            headers: vec![("x-workspace-id".to_string(), "from-params".to_string())],
        };

        assert_eq!(
            client.request_headers(&request),
            vec![
                ("x-workspace-id".to_string(), "from-params".to_string()),
                ("X-Other".to_string(), "kept".to_string()),
            ]
        );
    }
}
