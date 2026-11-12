use crate::error::KobanaError;
use crate::i18n::{self, Lang};
use crate::spec::HttpMethod;
use crate::{tr, trf};
use reqwest::header::{HeaderMap, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::path::{Path, PathBuf};
use std::time::Duration;

const USER_AGENT_VALUE: &str = concat!("kobana-cli/", env!("CARGO_PKG_VERSION"));

/// The channel the APIs stamp on what a request writes (`created_via`,
/// `updated_via`). Finance, billing and dashboards read it; the others ignore
/// it. `--params` may still set its own (`other` with `X-Request-Source-Other`).
pub const REQUEST_SOURCE: (&str, &str) = ("X-Request-Source", "cli");

fn accept_language() -> &'static str {
    match i18n::lang() {
        Lang::Pt => "pt-BR,pt;q=0.9",
        Lang::En => "en",
        Lang::Es => "es",
    }
}

#[derive(Debug, Clone)]
pub struct KobanaClient {
    inner: reqwest::Client,
    base_url: String,
    token: String,
    /// Headers sent on every request unless the request sets the same name
    default_headers: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub method: HttpMethod,
    pub path: String,
    pub query_params: Option<serde_json::Value>,
    pub body: Option<serde_json::Value>,
    pub idempotency_key: Option<String>,
    /// Header parameters declared by the endpoint (e.g. `X-Workspace-Id`)
    pub headers: Vec<(String, String)>,
    /// Send `body` as `multipart/form-data` instead of JSON: its top-level
    /// fields become text parts, next to `files`
    pub multipart: bool,
    /// Files of a multipart request, one part each
    pub files: Vec<FilePart>,
}

/// One file of a multipart request: the form field and the local path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePart {
    pub field: String,
    pub path: PathBuf,
}

/// The media type a file part is sent with, from its extension. The APIs read
/// the content, not the name, except for a raw e-mail (`message/rfc822`).
pub fn file_media_type(path: &Path) -> &'static str {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase());
    match ext.as_deref() {
        Some("eml") => "message/rfc822",
        Some("pdf") => "application/pdf",
        Some("xml") => "application/xml",
        Some("csv") => "text/csv",
        Some("txt") => "text/plain",
        Some("json") => "application/json",
        Some("ofx") => "application/x-ofx",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("pem") | Some("crt") | Some("cer") | Some("key") => "application/x-pem-file",
        Some("pfx") | Some("p12") => "application/x-pkcs12",
        _ => "application/octet-stream",
    }
}

/// The text parts of a multipart body: each top-level field of the JSON
/// object, strings as they are and anything else as its JSON text. `null` is
/// left out, as an absent field.
pub fn multipart_text_fields(body: Option<&serde_json::Value>) -> Result<Vec<(String, String)>, KobanaError> {
    let Some(body) = body else {
        return Ok(Vec::new());
    };
    let Some(obj) = body.as_object() else {
        return Err(KobanaError::Validation(
            tr!(
                pt: "com --file, o --json precisa ser um objeto (cada campo vira uma parte do formulário)",
                en: "with --file, --json must be an object (each field becomes a form part)",
                es: "con --file, el --json debe ser un objeto (cada campo se vuelve una parte del formulario)",
            )
            .to_string(),
        ));
    };
    Ok(obj
        .iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(k, v)| {
            let text = match v {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            (k.clone(), text)
        })
        .collect())
}

async fn multipart_form(req: &ApiRequest) -> Result<reqwest::multipart::Form, KobanaError> {
    let mut form = reqwest::multipart::Form::new();
    for (name, value) in multipart_text_fields(req.body.as_ref())? {
        form = form.text(name, value);
    }
    for file in &req.files {
        let bytes = tokio::fs::read(&file.path).await.map_err(|e| {
            let path = file.path.display();
            KobanaError::Validation(trf!(
                pt: "não foi possível ler {path}: {e}",
                en: "could not read {path}: {e}",
                es: "no se pudo leer {path}: {e}",
            ))
        })?;
        let file_name = file
            .path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "file".to_string());
        let part = reqwest::multipart::Part::bytes(bytes)
            .file_name(file_name)
            .mime_str(file_media_type(&file.path))
            .map_err(|e| KobanaError::Validation(e.to_string()))?;
        form = form.part(file.field.clone(), part);
    }
    Ok(form)
}

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: serde_json::Value,
}

impl KobanaClient {
    pub fn new(base_url: &str, token: &str) -> Result<Self, KobanaError> {
        Self::build(base_url, token, None)
    }

    /// Client that presents a certificate during the TLS handshake.
    ///
    /// `pem` must hold the certificate chain and its private key. For a
    /// product whose edge terminates mTLS (`Product::client_cert_env`); none
    /// does today — inbox did until its API dropped mTLS on 2026-10-31.
    pub fn with_client_cert(base_url: &str, token: &str, pem: &[u8]) -> Result<Self, KobanaError> {
        let identity = reqwest::Identity::from_pem(pem).map_err(|e| {
            KobanaError::Auth(trf!(
                pt: "certificado de cliente inválido: {e}. Era esperado um PEM com a cadeia do certificado e a chave privada.",
                en: "invalid client certificate: {e}. Expected PEM with the certificate chain and its private key.",
                es: "certificado de cliente inválido: {e}. Se esperaba un PEM con la cadena del certificado y la clave privada.",
            ))
        })?;
        Self::build(base_url, token, Some(identity))
    }

    fn build(
        base_url: &str,
        token: &str,
        identity: Option<reqwest::Identity>,
    ) -> Result<Self, KobanaError> {
        let mut builder = reqwest::Client::builder().timeout(Duration::from_secs(30));

        if let Some(identity) = identity {
            builder = builder.identity(identity);
        }

        let client = builder
            .build()
            .map_err(|e| KobanaError::Internal(trf!(
                pt: "não foi possível criar o cliente HTTP: {e}",
                en: "failed to create HTTP client: {e}",
                es: "no se pudo crear el cliente HTTP: {e}",
            )))?;

        Ok(Self {
            inner: client,
            base_url: base_url.trim_end_matches('/').to_string(),
            token: token.to_string(),
            default_headers: Vec::new(),
        })
    }

    /// Send these headers on every request that does not set them itself.
    pub fn with_default_headers(mut self, headers: Vec<(String, String)>) -> Self {
        self.default_headers = headers;
        self
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// The headers a request will carry besides auth, content type and user
    /// agent: its own header parameters, then any default it does not override,
    /// then [`REQUEST_SOURCE`].
    pub fn request_headers(&self, req: &ApiRequest) -> Vec<(String, String)> {
        let mut headers = req.headers.clone();
        let source = (REQUEST_SOURCE.0.to_string(), REQUEST_SOURCE.1.to_string());
        for (name, value) in self.default_headers.iter().chain(std::iter::once(&source)) {
            if !headers.iter().any(|(n, _)| n.eq_ignore_ascii_case(name)) {
                headers.push((name.clone(), value.clone()));
            }
        }
        headers
    }

    pub async fn execute(&self, req: &ApiRequest) -> Result<ApiResponse, KobanaError> {
        let url = format!("{}{}", self.base_url, req.path);

        tracing::debug!(method = %req.method, url = %url, "executing API request");

        let mut builder = match req.method {
            HttpMethod::Get => self.inner.get(&url),
            HttpMethod::Post => self.inner.post(&url),
            HttpMethod::Put => self.inner.put(&url),
            HttpMethod::Patch => self.inner.patch(&url),
            HttpMethod::Delete => self.inner.delete(&url),
        };

        builder = builder
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, USER_AGENT_VALUE)
            // APIs that localize their messages answer in the user's language
            .header(ACCEPT_LANGUAGE, accept_language());

        // Add idempotency key for mutations
        if let Some(key) = &req.idempotency_key {
            builder = builder.header("X-Idempotency-Key", key.as_str());
        }

        for (name, value) in self.request_headers(req) {
            builder = builder.header(name, value);
        }

        // Add query parameters
        if let Some(params) = &req.query_params {
            if let Some(obj) = params.as_object() {
                let pairs: Vec<(String, String)> = obj
                    .iter()
                    .map(|(k, v)| {
                        let val = match v {
                            serde_json::Value::String(s) => s.clone(),
                            other => other.to_string(),
                        };
                        (k.clone(), val)
                    })
                    .collect();
                builder = builder.query(&pairs);
            }
        }

        // Add request body. A multipart form sets its own content type (with
        // the boundary), so the JSON one goes only on a JSON request.
        if req.multipart {
            builder = builder.multipart(multipart_form(req).await?);
        } else {
            builder = builder.header(CONTENT_TYPE, "application/json");
            if let Some(body) = &req.body {
                builder = builder.json(body);
            }
        }

        let response = builder.send().await?;
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body_text = response.text().await?;

        let body: serde_json::Value = if body_text.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_str(&body_text).unwrap_or(serde_json::Value::String(body_text))
        };

        if status >= 400 {
            let message = error_message(&body);

            return Err(KobanaError::Api {
                status,
                message,
                body: Some(body),
            });
        }

        Ok(ApiResponse {
            status,
            headers,
            body,
        })
    }
}

/// The human message in an error body. The products answer in different
/// shapes: `{"error": "..."}`, `{"error": {"code", "message"}}` (kadu),
/// `{"errors": [{"message"}]}` and `{"message": "..."}`.
fn error_message(body: &serde_json::Value) -> String {
    body["error"]
        .as_str()
        .or_else(|| body["error"]["message"].as_str())
        .or_else(|| body["errors"][0]["message"].as_str())
        .or_else(|| body["message"].as_str())
        .unwrap_or(tr!(pt: "erro desconhecido", en: "unknown error", es: "error desconocido"))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_message_reads_every_product_shape() {
        use serde_json::json;
        assert_eq!(error_message(&json!({"error": "invalid_token"})), "invalid_token");
        assert_eq!(
            error_message(&json!({"error": {"code": "AS_UNAVAILABLE", "message": "Retry shortly."}})),
            "Retry shortly."
        );
        assert_eq!(error_message(&json!({"errors": [{"message": "bad"}]})), "bad");
        assert_eq!(error_message(&json!({"message": "nope"})), "nope");
    }

    #[test]
    fn multipart_text_fields_keep_strings_and_encode_the_rest() {
        let body = serde_json::json!({
            "inbox_id": "abc",
            "tags": ["a", "b"],
            "custom_data": {"k": 1},
            "draft": false,
            "skipped": null
        });
        let mut fields = multipart_text_fields(Some(&body)).unwrap();
        fields.sort();
        assert_eq!(
            fields,
            vec![
                ("custom_data".to_string(), r#"{"k":1}"#.to_string()),
                ("draft".to_string(), "false".to_string()),
                ("inbox_id".to_string(), "abc".to_string()),
                ("tags".to_string(), r#"["a","b"]"#.to_string()),
            ]
        );
        assert!(multipart_text_fields(None).unwrap().is_empty());
        assert!(multipart_text_fields(Some(&serde_json::json!([1]))).is_err());
    }

    #[test]
    fn a_raw_email_is_sent_as_message_rfc822() {
        assert_eq!(file_media_type(Path::new("msg.EML")), "message/rfc822");
        assert_eq!(file_media_type(Path::new("nota.pdf")), "application/pdf");
        assert_eq!(file_media_type(Path::new("cert.pfx")), "application/x-pkcs12");
        assert_eq!(file_media_type(Path::new("sem-extensao")), "application/octet-stream");
    }
}
