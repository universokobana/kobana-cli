use crate::config::Environment;
use kobana::error::KobanaError;
use serde::Deserialize;
use sha2::Digest;
use std::io::{BufRead, Write};
use std::net::TcpListener;

fn oauth_authorize_url(env: &Environment) -> &'static str {
    match env {
        Environment::Sandbox => "https://app-sandbox.kobana.com.br/oauth/authorize",
        Environment::Production => "https://app.kobana.com.br/oauth/authorize",
        Environment::Development => "http://localhost:5005/oauth/authorize",
    }
}

fn oauth_token_url(env: &Environment) -> &'static str {
    match env {
        Environment::Sandbox => "https://app-sandbox.kobana.com.br/oauth/token",
        Environment::Production => "https://app.kobana.com.br/oauth/token",
        Environment::Development => "http://localhost:5005/oauth/token",
    }
}

/// Default public client ID for the Kobana CLI (PKCE, no secret needed)
pub const DEFAULT_CLIENT_ID: &str = "kobana-cli";

/// All available Kobana OAuth scopes
pub const ALL_SCOPES: &[&str] = &[
    "admin.subaccounts",
    "admin.users",
    "automation.email_accounts",
    "automation.email_deliveries",
    "automation.sms_accounts",
    "automation.sms_deliveries",
    "automation.webhook_deliveries",
    "automation.webhooks",
    "billing.transactions",
    "charge.automatic_pix.accounts",
    "charge.automatic_pix.locations",
    "charge.automatic_pix.pix",
    "charge.automatic_pix.recurrences",
    "charge.automatic_pix.requests",
    "charge.bank_billet_accounts",
    "charge.bank_billet_payments",
    "charge.bank_billet_registrations",
    "charge.bank_billets",
    "charge.customer_subscriptions",
    "charge.installments",
    "charge.payments",
    "charge.pix",
    "charge.pix_accounts",
    "core.providers",
    "crm.customers",
    "crm.people",
    "data.bank_billet_queries",
    "financial.accounts",
    "financial.balances",
    "financial.providers",
    "financial.statement_transactions",
    "integration.certificates",
    "integration.connections",
    "integration.discharges",
    "integration.edi_boxes",
    "integration.remittances",
    "login",
    "mailbox.entries",
    "mailbox.files",
    "partner.bank_contracts",
    "payment.bank_billets",
    "payment.batches",
    "payment.darfs",
    "payment.dda",
    "payment.dda.bank_billets",
    "payment.dda_accounts",
    "payment.payments",
    "payment.pix",
    "payment.taxes",
    "payment.utilities",
    "security.access_tokens",
    "system.events",
    "system.imports",
    "system.reports",
    "transfer.batches",
    "transfer.internal",
    "transfer.pix",
    "transfer.ted",
    "transfer.transfers",
];

/// Default scopes requested by the CLI (all scopes)
pub fn default_scopes() -> String {
    ALL_SCOPES.join("+")
}

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
    scopes: &str,
    env: &Environment,
    verbose: bool,
) -> Result<TokenResponse, KobanaError> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| KobanaError::Internal(format!("failed to bind callback server: {e}")))?;
    let port = listener.local_addr().unwrap().port();
    let redirect_uri = format!("http://127.0.0.1:{port}");

    // Generate PKCE pair
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    let authorize_base = oauth_authorize_url(env);

    let authorize_url = format!(
        "{authorize_base}?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code&code_challenge={code_challenge}&code_challenge_method=S256&scope={scopes}"
    );

    eprintln!("Opening browser for authorization...");
    eprintln!("If the browser doesn't open, visit: {authorize_url}");

    let _ = open_browser(&authorize_url);

    eprintln!("Waiting for authorization callback on port {port}...");
    let code = wait_for_callback(listener)?;

    let token_url = oauth_token_url(env);

    exchange_code(
        token_url,
        client_id,
        client_secret,
        &code,
        &redirect_uri,
        &code_verifier,
        verbose,
    )
    .await
}

/// Client Credentials flow (requires secret)
pub async fn client_credentials_flow(
    client_id: &str,
    client_secret: &str,
    env: &Environment,
) -> Result<TokenResponse, KobanaError> {
    let token_url = oauth_token_url(env);

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
    env: &Environment,
) -> Result<TokenResponse, KobanaError> {
    let token_url = oauth_token_url(env);

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
    verbose: bool,
) -> Result<TokenResponse, KobanaError> {
    let user_agent = format!("kobana-cli/{}", env!("CARGO_PKG_VERSION"));
    let client = reqwest::Client::builder()
        .user_agent(&user_agent)
        .build()
        .map_err(|e| KobanaError::Internal(format!("failed to build client: {e}")))?;

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

    if verbose {
        eprintln!("\n--- POST {token_url} ---");
        eprintln!("User-Agent: {user_agent}");
        eprintln!("Content-Type: application/x-www-form-urlencoded");
        eprintln!("Accept: application/json");
        eprintln!("Body (form):");
        for (k, v) in &params {
            let display = if *k == "code" || *k == "code_verifier" || *k == "client_secret" {
                format!("{}...", &v[..v.len().min(8)])
            } else {
                v.to_string()
            };
            eprintln!("  {k}={display}");
        }
        eprintln!("---");
    }

    let response = client
        .post(token_url)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .map_err(|e| KobanaError::Auth(format!("token exchange failed: {e}")))?;

    let status = response.status();
    let version = response.version();
    let headers = response.headers().clone();
    let body = response.text().await.unwrap_or_default();

    if verbose {
        eprintln!("--- Response: {status} ({version:?}) ---");
        for (name, value) in headers.iter() {
            eprintln!("{name}: {}", value.to_str().unwrap_or("<binary>"));
        }
        eprintln!();
        eprintln!("Body ({} bytes): {body}", body.len());
        eprintln!("---\n");
    }

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
