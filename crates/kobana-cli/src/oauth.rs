use kobana::error::KobanaError;
use serde::Deserialize;
use sha2::Digest;
use std::io::{BufRead, Write};
use std::net::TcpListener;

const OAUTH_AUTHORIZE_URL: &str = "https://app.kobana.com.br/oauth/authorize";
const OAUTH_TOKEN_URL_SANDBOX: &str = "https://api-sandbox.kobana.com.br/oauth/token";
const OAUTH_TOKEN_URL_PRODUCTION: &str = "https://api.kobana.com.br/oauth/token";

/// Default public client ID for the Kobana CLI (PKCE, no secret needed)
pub const DEFAULT_CLIENT_ID: &str = "kobana-cli";

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i64>,
}

/// Generate a PKCE code verifier (43-128 chars, URL-safe)
fn generate_code_verifier() -> String {
    use base64::Engine;
    let random_bytes: [u8; 32] = rand::random();
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(random_bytes)
}

/// Generate a PKCE code challenge from the verifier (S256)
fn generate_code_challenge(verifier: &str) -> String {
    use base64::Engine;
    let digest = sha2::Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}

/// Authorization Code flow with PKCE: open browser, listen for callback
pub async fn authorization_code_flow(
    client_id: &str,
    client_secret: Option<&str>,
    production: bool,
) -> Result<TokenResponse, KobanaError> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| KobanaError::Internal(format!("failed to bind callback server: {e}")))?;
    let port = listener.local_addr().unwrap().port();
    let redirect_uri = format!("http://localhost:{port}");

    // Generate PKCE pair
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    let authorize_url = format!(
        "{OAUTH_AUTHORIZE_URL}?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code&code_challenge={code_challenge}&code_challenge_method=S256"
    );

    eprintln!("Opening browser for authorization...");
    eprintln!("If the browser doesn't open, visit: {authorize_url}");

    let _ = open_browser(&authorize_url);

    eprintln!("Waiting for authorization callback on port {port}...");
    let code = wait_for_callback(listener)?;

    let token_url = if production {
        OAUTH_TOKEN_URL_PRODUCTION
    } else {
        OAUTH_TOKEN_URL_SANDBOX
    };

    exchange_code(
        token_url,
        client_id,
        client_secret,
        &code,
        &redirect_uri,
        &code_verifier,
    )
    .await
}

/// Client Credentials flow (requires secret)
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
    client_secret: Option<&str>,
    code: &str,
    redirect_uri: &str,
    code_verifier: &str,
) -> Result<TokenResponse, KobanaError> {
    let client = reqwest::Client::new();

    let mut params = vec![
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("code_verifier", code_verifier),
    ];

    // Only include client_secret if provided (not needed for PKCE public clients)
    let secret_string;
    if let Some(secret) = client_secret {
        secret_string = secret.to_string();
        params.push(("client_secret", &secret_string));
    }

    let response = client
        .post(token_url)
        .form(&params)
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
