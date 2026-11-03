use kobana::client::{ApiRequest, FilePart, KobanaClient};
use kobana::error::KobanaError;
use kobana::{tr, trf};
use kobana::spec::{HttpMethod, ParameterLocation, ResolvedEndpoint};
use kobana::validate::validate_identifier;

use crate::formatter::{filter_fields, format_output};
use crate::pagination;
use crate::validate;

/// How a response is shown: the global `--fields`, `--output-format`,
/// `--output` and `--verbose`. Dynamic commands and helpers share it.
pub struct Output {
    pub fields: Option<String>,
    pub format: String,
    pub path: Option<String>,
    pub verbose: bool,
}

impl Output {
    /// Read from any matches the global options reach (root or a subcommand)
    pub fn from_matches(matches: &clap::ArgMatches) -> Self {
        Self {
            fields: matches.get_one::<String>("fields").cloned(),
            format: matches
                .get_one::<String>("output-format")
                .cloned()
                .unwrap_or_else(|| "json".into()),
            path: matches.get_one::<String>("output").cloned(),
            verbose: matches.get_flag("verbose"),
        }
    }

    /// Apply `--fields`, format, and print or save to `--output`
    pub fn print(&self, value: &serde_json::Value) -> Result<(), KobanaError> {
        let output = match &self.fields {
            Some(fields) => filter_fields(value, fields),
            None => value.clone(),
        };
        let formatted = format_output(&output, &self.format)?;

        let Some(path) = &self.path else {
            println!("{formatted}");
            return Ok(());
        };
        std::fs::write(path, &formatted).map_err(|e| {
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
        Ok(())
    }
}

/// `--params` or `--json`, parsed
pub fn json_arg(matches: &clap::ArgMatches, name: &str) -> Result<Option<serde_json::Value>, KobanaError> {
    matches
        .get_one::<String>(name)
        .map(|s| serde_json::from_str::<serde_json::Value>(s))
        .transpose()
        .map_err(|e| KobanaError::Validation(trf!(
            pt: "JSON inválido em --{name}: {e}",
            en: "invalid --{name} JSON: {e}",
            es: "JSON inválido en --{name}: {e}",
        )))
}

/// `--idempotency-key`, or a new key for a POST
pub fn idempotency_key(matches: &clap::ArgMatches, method: HttpMethod) -> Option<String> {
    match method {
        HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch => matches
            .get_one::<String>("idempotency-key")
            .cloned()
            .or_else(|| (method == HttpMethod::Post).then(|| uuid::Uuid::new_v4().to_string())),
        _ => None,
    }
}

/// What `--dry-run` prints for one request
pub fn dry_run_report(client: &KobanaClient, request: &ApiRequest) -> serde_json::Value {
    serde_json::json!({
        "dry_run": true,
        "method": request.method.as_str(),
        "url": format!("{}{}", client.base_url(), request.path),
        "query_params": request.query_params,
        "body": request.body,
        "content_type": if request.multipart { "multipart/form-data" } else { "application/json" },
        "files": request
            .files
            .iter()
            .map(|f| serde_json::json!({
                "field": f.field,
                "path": f.path.display().to_string(),
                "media_type": kobana::client::file_media_type(&f.path),
            }))
            .collect::<Vec<_>>(),
        "idempotency_key": request.idempotency_key,
        "headers": client
            .request_headers(request)
            .into_iter()
            .map(|(name, value)| (name, serde_json::Value::String(value)))
            .collect::<serde_json::Map<_, _>>(),
    })
}

/// Send one request, logging it to stderr with `--verbose`, and return the
/// response body
pub async fn send(client: &KobanaClient, request: &ApiRequest, verbose: bool) -> Result<serde_json::Value, KobanaError> {
    if verbose {
        eprintln!(">> {} {}{}", request.method, client.base_url(), request.path);
        if let Some(body) = &request.body {
            eprintln!(
                ">> {} {}",
                tr!(pt: "Corpo:", en: "Body:", es: "Cuerpo:"),
                serde_json::to_string(body)?
            );
        }
    }

    let response = client.execute(request).await?;

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
    Ok(response.body)
}

/// Execute an API request based on the resolved endpoint and CLI args
pub async fn execute(
    client: &KobanaClient,
    endpoint: &ResolvedEndpoint,
    root_matches: &clap::ArgMatches,
) -> Result<(), KobanaError> {
    let params_json = json_arg(root_matches, "params")?;
    let body_json = json_arg(root_matches, "json")?;

    // Validate inputs
    if let Some(ref params) = params_json {
        validate::validate_params(params)?;
    }
    if let Some(ref body) = body_json {
        validate::validate_body(body)?;
    }

    let files = root_matches
        .get_many::<String>("file")
        .map(|values| values.map(|v| parse_file_arg(v)).collect::<Result<Vec<_>, _>>())
        .transpose()?
        .unwrap_or_default();
    let multipart = sends_multipart(endpoint, !files.is_empty(), body_json.is_some())?;
    if multipart {
        // Fail before anything is sent, with the same message a dry run gives
        kobana::client::multipart_text_fields(body_json.as_ref())?;
    }

    let output = Output::from_matches(root_matches);
    let dry_run = root_matches.get_flag("dry-run");

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
    let (path, path_keys) = resolve_path_template(&endpoint.path_template, &params_json)?;

    // Header parameters travel in --params too; move them out of the query
    let mut params_json = params_json;
    let headers = take_header_params(endpoint, &mut params_json)?;

    // Handle auto-pagination (skip if dry-run)
    if page_all && !dry_run && endpoint.http_method == HttpMethod::Get {
        let query_params = query_without(params_json, &path_keys);

        return pagination::paginate_all(
            client,
            endpoint,
            &path,
            &query_params,
            &headers,
            page_limit,
            page_delay,
            &output,
        )
        .await;
    }

    let idempotency_key = idempotency_key(root_matches, endpoint.http_method);

    // The keys that went into the path stay out of the query
    let query_params = query_without(params_json, &path_keys);

    let api_request = ApiRequest {
        method: endpoint.http_method,
        path,
        query_params,
        body: body_json,
        idempotency_key,
        headers,
        multipart,
        files,
    };

    // Dry run: show the request without executing
    if dry_run {
        println!("{}", serde_json::to_string_pretty(&dry_run_report(client, &api_request))?);
        return Ok(());
    }

    let body = send(client, &api_request, output.verbose).await?;
    output.print(&body)
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

/// The `{name}` placeholders of a path template, in order
fn template_params(template: &str) -> Vec<&str> {
    template
        .split('{')
        .skip(1)
        .filter_map(|rest| rest.split_once('}').map(|(name, _)| name))
        .collect()
}

/// Replace path template parameters with values from --params. Returns the
/// path and the `--params` keys it consumed, which must leave the query.
///
/// A `*_uid`/`*_id` placeholder missing from `--params` falls back to `uid`
/// or `id` — but only to one that is not a placeholder of the same template:
/// in `/charge/pix/{pix_uid}/commands/{uid}` the `uid` is the command's, never
/// the Pix's.
fn resolve_path_template(
    template: &str,
    params: &Option<serde_json::Value>,
) -> Result<(String, Vec<String>), KobanaError> {
    let placeholders = template_params(template);
    let lookup = |key: &str| params.as_ref().and_then(|p| p.get(key));

    // Built segment by segment, so a value is never read as a placeholder
    let mut path = String::new();
    let mut rest = template;
    let mut used = Vec::new();
    for param_name in &placeholders {
        let fallbacks: &[&str] = if param_name.ends_with("_uid") || param_name.ends_with("_id") {
            &["uid", "id"]
        } else {
            &[]
        };
        let key = std::iter::once(*param_name)
            .chain(fallbacks.iter().copied().filter(|f| !placeholders.contains(f)))
            .find(|key| lookup(key).is_some())
            .ok_or_else(|| {
                KobanaError::Validation(trf!(
                    pt: "falta o parâmetro de caminho obrigatório '{param_name}' em --params",
                    en: "missing required path parameter '{param_name}' in --params",
                    es: "falta el parámetro de ruta obligatorio '{param_name}' en --params",
                ))
            })?;

        let value_str = match lookup(key).unwrap() {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            other => other.to_string().trim_matches('"').to_string(),
        };

        validate_identifier(&value_str, param_name)?;

        let placeholder = format!("{{{param_name}}}");
        let (before, after) = rest.split_once(&placeholder).expect("placeholder comes from the template");
        path.push_str(before);
        path.push_str(&value_str);
        rest = after;
        used.push(key.to_string());
    }
    path.push_str(rest);

    Ok((path, used))
}

/// `--params` without the keys that went into the path
fn query_without(params: Option<serde_json::Value>, used: &[String]) -> Option<serde_json::Value> {
    params.map(|mut params| {
        if let Some(obj) = params.as_object_mut() {
            for key in used {
                obj.remove(key);
            }
        }
        params
    })
}

/// The media types the endpoint's request body is documented with.
fn request_media_types(endpoint: &ResolvedEndpoint) -> Vec<String> {
    endpoint
        .operation
        .request_body
        .as_ref()
        .and_then(|b| b.get("content"))
        .and_then(|c| c.as_object())
        .map(|c| c.keys().cloned().collect())
        .unwrap_or_default()
}

/// Whether the request goes as `multipart/form-data`: when it carries files,
/// or when the endpoint takes a form and no JSON at all (its fields then go as
/// text parts). Files on an endpoint that takes no form are refused before
/// anything is sent.
fn sends_multipart(endpoint: &ResolvedEndpoint, has_files: bool, has_body: bool) -> Result<bool, KobanaError> {
    let types = request_media_types(endpoint);
    let takes_form = types.iter().any(|t| t == "multipart/form-data");
    let takes_json = types.is_empty() || types.iter().any(|t| t == "application/json");
    if has_files && !takes_form {
        return Err(KobanaError::Validation(
            tr!(
                pt: "este endpoint não recebe arquivos (--file); veja `kobana schema` para o corpo esperado",
                en: "this endpoint takes no files (--file); see `kobana schema` for the expected body",
                es: "este endpoint no recibe archivos (--file); vea `kobana schema` para el cuerpo esperado",
            )
            .to_string(),
        ));
    }
    Ok(has_files || (takes_form && !takes_json && has_body))
}

/// `--file campo=caminho`: the form field and an existing local file.
fn parse_file_arg(value: &str) -> Result<FilePart, KobanaError> {
    let Some((field, path)) = value.split_once('=') else {
        return Err(KobanaError::Validation(trf!(
            pt: "--file espera campo=caminho (ex.: file=./mensagem.eml), recebeu {value}",
            en: "--file expects field=path (e.g. file=./message.eml), got {value}",
            es: "--file espera campo=ruta (ej.: file=./mensaje.eml), recibió {value}",
        )));
    };
    let field = field.trim();
    if field.is_empty() || field.chars().any(|c| c.is_control() || c == '"') {
        return Err(KobanaError::Validation(trf!(
            pt: "nome de campo inválido em --file: {value}",
            en: "invalid field name in --file: {value}",
            es: "nombre de campo inválido en --file: {value}",
        )));
    }
    let path = std::path::PathBuf::from(path);
    if !path.is_file() {
        let shown = path.display();
        return Err(KobanaError::Validation(trf!(
            pt: "--file: arquivo não encontrado: {shown}",
            en: "--file: file not found: {shown}",
            es: "--file: archivo no encontrado: {shown}",
        )));
    }
    Ok(FilePart { field: field.to_string(), path })
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

    fn resolve(template: &str, params: serde_json::Value) -> Result<(String, Option<serde_json::Value>), KobanaError> {
        let params = Some(params);
        let (path, used) = resolve_path_template(template, &params)?;
        Ok((path, query_without(params, &used)))
    }

    #[test]
    fn path_params_take_their_own_name_first() {
        let (path, query) = resolve(
            "/v2/charge/pix/{pix_uid}/commands/{uid}",
            serde_json::json!({"pix_uid": "P", "uid": "C", "x": 1}),
        )
        .unwrap();
        assert_eq!(path, "/v2/charge/pix/P/commands/C");
        assert_eq!(query, Some(serde_json::json!({"x": 1})));
    }

    #[test]
    fn a_fallback_that_is_a_placeholder_of_the_template_is_not_used() {
        // The command's uid used to fill the Pix's too
        let err = resolve("/v2/charge/pix/{pix_uid}/commands/{uid}", serde_json::json!({"uid": "C"})).unwrap_err();
        assert!(matches!(err, KobanaError::Validation(ref m) if m.contains("pix_uid")), "{err}");

        let err = resolve("/v1/entities/{entity_id}/records/{id}", serde_json::json!({"id": "E"})).unwrap_err();
        assert!(matches!(err, KobanaError::Validation(ref m) if m.contains("entity_id")), "{err}");
    }

    #[test]
    fn the_fallback_key_leaves_the_query() {
        let (path, query) = resolve("/v2/charge/pix/{pix_uid}/cancel", serde_json::json!({"uid": "P1"})).unwrap();
        assert_eq!(path, "/v2/charge/pix/P1/cancel");
        assert_eq!(query, Some(serde_json::json!({})));

        let (path, _) = resolve("/v1/entities/{entity_id}/records", serde_json::json!({"id": "E"})).unwrap();
        assert_eq!(path, "/v1/entities/E/records");
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
            multipart: false,
            files: Vec::new(),
        };

        assert_eq!(
            client.request_headers(&request),
            vec![
                ("x-workspace-id".to_string(), "from-params".to_string()),
                ("X-Other".to_string(), "kept".to_string()),
            ]
        );
    }

    fn post_endpoint(content: serde_json::Value) -> ResolvedEndpoint {
        let spec = serde_json::json!({
            "paths": {"/v1/emails": {"post": {
                "requestBody": {"content": content},
                "responses": {}
            }}}
        });
        let tree = ApiSpec::from_value(&spec).unwrap().build_command_tree("/v1");
        tree.children["emails"].endpoints[0].clone()
    }

    #[test]
    fn files_go_as_multipart_only_where_the_endpoint_takes_a_form() {
        let form_or_eml = post_endpoint(serde_json::json!({"message/rfc822": {}, "multipart/form-data": {}}));
        let json_only = post_endpoint(serde_json::json!({"application/json": {}}));
        let form_only = post_endpoint(serde_json::json!({"multipart/form-data": {}}));
        let both = post_endpoint(serde_json::json!({"application/json": {}, "multipart/form-data": {}}));

        assert!(sends_multipart(&form_or_eml, true, true).unwrap());
        assert!(sends_multipart(&json_only, true, false).is_err());
        assert!(!sends_multipart(&json_only, false, true).unwrap());
        // A form-only endpoint takes --json fields as text parts, files or not
        assert!(sends_multipart(&form_only, false, true).unwrap());
        // Where JSON is accepted too, only files switch to multipart
        assert!(!sends_multipart(&both, false, true).unwrap());
        assert!(sends_multipart(&both, true, true).unwrap());
    }

    #[test]
    fn file_args_name_a_field_and_an_existing_file() {
        let path = std::env::temp_dir().join(format!("kobana-file-arg-{}.eml", std::process::id()));
        std::fs::write(&path, "Subject: x\r\n\r\nbody").unwrap();
        let arg = format!("file={}", path.display());

        let part = parse_file_arg(&arg).unwrap();
        assert_eq!(part.field, "file");
        assert_eq!(part.path, path);
        assert!(parse_file_arg(&path.display().to_string()).is_err(), "no field");
        assert!(parse_file_arg("=x.eml").is_err(), "empty field");
        assert!(parse_file_arg("file=/no/such/file.eml").is_err(), "missing file");
        std::fs::remove_file(path).ok();
    }
}
