use kobana::error::KobanaError;
use serde::Deserialize;
use std::io::{BufRead, Write};
use std::net::TcpListener;

const OAUTH_AUTHORIZE_URL: &str = "https://app.kobana.com.br/oauth/authorize";
const OAUTH_TOKEN_URL_SANDBOX: &str = "https://api-sandbox.kobana.com.br/oauth/token";
const OAUTH_TOKEN_URL_PRODUCTION: &str = "https://api.kobana.com.br/oauth/token";

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i64>,
}

/// Authorization Code flow: open browser, listen for callback
pub async fn authorization_code_flow(
    client_id: &str,
    client_secret: &str,
    production: bool,
) -> Result<TokenResponse, KobanaError> {
    // Bind to a random port for the callback
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| KobanaError::Internal(format!("failed to bind callback server: {e}")))?;
    let port = listener.local_addr().unwrap().port();
    let redirect_uri = format!("http://localhost:{port}");

    let authorize_url = format!(
        "{OAUTH_AUTHORIZE_URL}?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code"
    );

    eprintln!("Opening browser for authorization...");
    eprintln!("If the browser doesn't open, visit: {authorize_url}");

    // Try to open browser
    let _ = open_browser(&authorize_url);

    // Wait for the callback
    eprintln!("Waiting for authorization callback on port {port}...");
    let code = wait_for_callback(listener)?;

    // Exchange code for token
    let token_url = if production {
        OAUTH_TOKEN_URL_PRODUCTION
    } else {
        OAUTH_TOKEN_URL_SANDBOX
    };

    exchange_code(token_url, client_id, client_secret, &code, &redirect_uri).await
}

/// Client Credentials flow
pub async fn client_credentials_flow(
    client_id: &str,
    client_secret: &str,
    production: bool,
) -> Result<TokenResponse, KobanaError> {
    let token_url = if production {
        OAUTH_TOKEN_URL_PRODUCTION
    } else {
        OAUTH_TOKEN_URL_SANDBOX
    };

    let client = reqwest::Client::new();
    let response = client
        .post(token_url)
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send()
        .await
        .map_err(|e| KobanaError::Auth(format!("token request failed: {e}")))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(KobanaError::Auth(format!(
            "token request failed ({status}): {body}"
        )));
    }

    serde_json::from_str(&body)
        .map_err(|e| KobanaError::Auth(format!("invalid token response: {e}")))
}

/// Refresh an access token
#[allow(dead_code)]
pub async fn refresh_token(
    refresh_token: &str,
    production: bool,
) -> Result<TokenResponse, KobanaError> {
    let token_url = if production {
        OAUTH_TOKEN_URL_PRODUCTION
    } else {
        OAUTH_TOKEN_URL_SANDBOX
    };

    let client = reqwest::Client::new();
    let response = client
        .post(token_url)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await
        .map_err(|e| KobanaError::Auth(format!("refresh request failed: {e}")))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(KobanaError::Auth(format!(
            "refresh failed ({status}): {body}"
        )));
    }

    serde_json::from_str(&body)
        .map_err(|e| KobanaError::Auth(format!("invalid refresh response: {e}")))
}

async fn exchange_code(
    token_url: &str,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<TokenResponse, KobanaError> {
    let client = reqwest::Client::new();
    let response = client
        .post(token_url)
        .form(&[
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("code", code),
            ("redirect_uri", redirect_uri),
        ])
        .send()
        .await
        .map_err(|e| KobanaError::Auth(format!("token exchange failed: {e}")))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(KobanaError::Auth(format!(
            "token exchange failed ({status}): {body}"
        )));
    }

    serde_json::from_str(&body)
        .map_err(|e| KobanaError::Auth(format!("invalid token response: {e}")))
}

fn wait_for_callback(listener: TcpListener) -> Result<String, KobanaError> {
    // Set a timeout for waiting
    listener
        .set_nonblocking(false)
        .map_err(|e| KobanaError::Internal(format!("failed to set blocking: {e}")))?;

    let (stream, _) = listener
        .accept()
        .map_err(|e| KobanaError::Auth(format!("failed to accept callback: {e}")))?;

    let mut reader = std::io::BufReader::new(&stream);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|e| KobanaError::Auth(format!("failed to read callback: {e}")))?;

    // Parse the code from the request: GET /?code=xxx HTTP/1.1
    let code = request_line
        .split_whitespace()
        .nth(1) // URI part
        .and_then(|uri| {
            uri.split('?')
                .nth(1)
                .and_then(|query| {
                    query.split('&').find_map(|param| {
                        let (key, value) = param.split_once('=')?;
                        if key == "code" {
                            Some(value.to_string())
                        } else {
                            None
                        }
                    })
                })
        })
        .ok_or_else(|| KobanaError::Auth("no authorization code in callback".into()))?;

    // Send a success response
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body><h2>Autorizado!</h2><p>Pode fechar esta aba e voltar ao terminal.</p></body></html>";
    let mut writer = std::io::BufWriter::new(&stream);
    let _ = writer.write_all(response.as_bytes());
    let _ = writer.flush();

    Ok(code)
}

fn open_browser(url: &str) -> Result<(), KobanaError> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| KobanaError::Internal(format!("failed to open browser: {e}")))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| KobanaError::Internal(format!("failed to open browser: {e}")))?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn()
            .map_err(|e| KobanaError::Internal(format!("failed to open browser: {e}")))?;
    }
    Ok(())
}
