use crate::error::KobanaError;
use crate::spec::HttpMethod;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::time::Duration;

const USER_AGENT_VALUE: &str = "kobana-cli/0.1.0";

#[derive(Debug, Clone)]
pub struct KobanaClient {
    inner: reqwest::Client,
    base_url: String,
    token: String,
}

#[derive(Debug, Clone)]
pub struct ApiRequest {
    pub method: HttpMethod,
    pub path: String,
    pub query_params: Option<serde_json::Value>,
    pub body: Option<serde_json::Value>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ApiResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: serde_json::Value,
}

impl KobanaClient {
    pub fn new(base_url: &str, token: &str) -> Result<Self, KobanaError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| KobanaError::Internal(format!("failed to create HTTP client: {e}")))?;

        Ok(Self {
            inner: client,
            base_url: base_url.trim_end_matches('/').to_string(),
            token: token.to_string(),
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
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
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, USER_AGENT_VALUE);

        // Add idempotency key for mutations
        if let Some(key) = &req.idempotency_key {
            builder = builder.header("X-Idempotency-Key", key.as_str());
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

        // Add request body
        if let Some(body) = &req.body {
            builder = builder.json(body);
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
            let message = body["error"]
                .as_str()
                .or_else(|| body["errors"][0]["message"].as_str())
                .or_else(|| body["message"].as_str())
                .unwrap_or("unknown error")
                .to_string();

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
