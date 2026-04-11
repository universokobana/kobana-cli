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

/// All available Kobana OAuth scopes with descriptions (pt-BR)
pub const ALL_SCOPES: &[(&str, &str)] = &[
    ("admin.subaccounts", "Gerenciar subcontas da conta principal"),
    ("admin.users", "Gerenciar usuários e permissões"),
    ("automation.email_accounts", "Gerenciar contas de e-mail para automação"),
    ("automation.email_deliveries", "Visualizar e gerenciar envios de e-mail"),
    ("automation.sms_accounts", "Gerenciar contas de SMS"),
    ("automation.sms_deliveries", "Visualizar e gerenciar envios de SMS"),
    ("automation.webhook_deliveries", "Visualizar e gerenciar entregas de webhooks"),
    ("automation.webhooks", "Gerenciar webhooks e notificações automatizadas"),
    ("billing.transactions", "Visualizar transações de cobrança e faturamento"),
    ("charge.automatic_pix.accounts", "Gerenciar contas de Pix automático"),
    ("charge.automatic_pix.locations", "Gerenciar localizações de Pix automático"),
    ("charge.automatic_pix.pix", "Gerenciar cobranças Pix automáticas"),
    ("charge.automatic_pix.recurrences", "Gerenciar recorrências de Pix automático"),
    ("charge.automatic_pix.requests", "Gerenciar solicitações de Pix automático"),
    ("charge.bank_billet_accounts", "Gerenciar contas de boletos bancários"),
    ("charge.bank_billet_payments", "Visualizar e registrar pagamentos de boletos"),
    ("charge.bank_billet_registrations", "Gerenciar registros de boletos nos bancos"),
    ("charge.bank_billets", "Gerenciar boletos bancários"),
    ("charge.customer_subscriptions", "Gerenciar assinaturas de clientes"),
    ("charge.installments", "Gerenciar parcelamentos"),
    ("charge.payments", "Visualizar e gerenciar pagamentos recebidos"),
    ("charge.pix", "Gerenciar cobranças Pix"),
    ("charge.pix_accounts", "Gerenciar contas Pix"),
    ("core.providers", "Gerenciar provedores do sistema (bancos e integrações)"),
    ("crm.customers", "Gerenciar clientes e informações comerciais"),
    ("crm.people", "Gerenciar pessoas e contatos"),
    ("data.bank_billet_queries", "Consultar informações de boletos bancários"),
    ("financial.accounts", "Gerenciar contas financeiras"),
    ("financial.balances", "Visualizar saldos e movimentações financeiras"),
    ("financial.providers", "Gerenciar provedores financeiros e integrações bancárias"),
    ("financial.statement_transactions", "Visualizar transações e extratos financeiros"),
    ("integration.certificates", "Gerenciar certificados digitais para integrações"),
    ("integration.connections", "Gerenciar conexões com bancos e provedores"),
    ("integration.discharges", "Gerenciar arquivos de retorno bancário (baixas)"),
    ("integration.edi_boxes", "Gerenciar caixas postais EDI para troca de arquivos"),
    ("integration.remittances", "Gerenciar arquivos de remessa bancária"),
    ("login", "Autenticar com o usuário"),
    ("mailbox.entries", "Gerenciar caixas postais para recebimento de arquivos"),
    ("mailbox.files", "Visualizar e gerenciar arquivos nas caixas postais"),
    ("partner.bank_contracts", "Gerenciar contratos bancários com parceiros"),
    ("payment.bank_billets", "Efetuar pagamentos de boletos bancários"),
    ("payment.batches", "Gerenciar lotes de pagamentos"),
    ("payment.darfs", "Efetuar pagamentos de DARFs (tributos federais)"),
    ("payment.dda", "Gerenciar DDA (Débito Direto Autorizado)"),
    ("payment.dda.bank_billets", "Visualizar e gerenciar boletos DDA disponíveis para pagamento"),
    ("payment.dda_accounts", "Gerenciar contas DDA (Débito Direto Autorizado)"),
    ("payment.payments", "Gerenciar todos os tipos de pagamentos através da API"),
    ("payment.pix", "Efetuar pagamentos via Pix"),
    ("payment.taxes", "Efetuar pagamentos de impostos e taxas"),
    ("payment.utilities", "Efetuar pagamentos de contas de consumo (água, luz, etc)"),
    ("security.access_tokens", "Gerenciar tokens de acesso e autenticação"),
    ("system.events", "Visualizar eventos e logs do sistema"),
    ("system.imports", "Gerenciar importações de dados"),
    ("system.reports", "Gerar e visualizar relatórios do sistema"),
    ("transfer.batches", "Gerenciar lotes de transferências"),
    ("transfer.internal", "Realizar transferências internas entre contas"),
    ("transfer.pix", "Realizar transferências via Pix"),
    ("transfer.ted", "Realizar transferências TED (Transferência Eletrônica Disponível)"),
    ("transfer.transfers", "Gerenciar todos os tipos de transferências (Pix, TED, Internas) via endpoint unificado"),
];

/// Top-level resource groups with human-readable descriptions (pt-BR).
/// Used for the first step of the interactive scope selector.
pub const TOP_LEVEL_GROUPS: &[(&str, &str)] = &[
    ("admin", "Administração (subcontas, usuários)"),
    ("automation", "Automação (e-mail, SMS, webhooks)"),
    ("billing", "Cobrança e faturamento"),
    ("charge", "Cobranças (boletos, Pix, parcelamentos)"),
    ("core", "Sistema central (provedores)"),
    ("crm", "CRM (clientes, pessoas)"),
    ("data", "Consultas de dados"),
    ("financial", "Financeiro (contas, saldos, extratos)"),
    ("integration", "Integrações (certificados, conexões, arquivos)"),
    ("login", "Autenticar com o usuário"),
    ("mailbox", "Caixas postais (EDI, arquivos)"),
    ("partner", "Parceiros (contratos bancários)"),
    ("payment", "Pagamentos (boletos, Pix, tributos, contas)"),
    ("security", "Segurança (tokens de acesso)"),
    ("system", "Sistema (eventos, imports, relatórios)"),
    ("transfer", "Transferências (Pix, TED, internas)"),
];

/// Default scopes requested by the CLI (all scopes)
pub fn default_scopes() -> String {
    ALL_SCOPES
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join("+")
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

    // Parse the query string from: GET /?code=xxx HTTP/1.1 or /?error=xxx
    let query: std::collections::HashMap<String, String> = request_line
        .split_whitespace()
        .nth(1)
        .and_then(|uri| uri.split_once('?').map(|(_, q)| q.to_string()))
        .map(|q| {
            q.split('&')
                .filter_map(|param| {
                    let (key, value) = param.split_once('=')?;
                    Some((
                        url_decode(key),
                        url_decode(value),
                    ))
                })
                .collect()
        })
        .unwrap_or_default();

    // Always write an HTML response before returning, so the browser tab
    // shows a meaningful message and the TCP connection is closed cleanly.
    let result = if let Some(code) = query.get("code") {
        write_html(
            &stream,
            200,
            "Autorização concedida",
            "<h2 style=\"color:#10b981;\">✓ Autorização concedida</h2>\
             <p><strong>Você foi autenticado com sucesso.</strong></p>\
             <p>Pode fechar esta aba e voltar ao terminal.</p>",
        );
        Ok(code.clone())
    } else if let Some(error) = query.get("error") {
        let description = query
            .get("error_description")
            .cloned()
            .unwrap_or_else(|| error.clone());
        write_html(
            &stream,
            400,
            "Autorização negada",
            &format!(
                "<h2 style=\"color:#ef4444;\">✗ Autorização negada</h2>\
                 <p><strong>{}</strong></p>\
                 <p>{}</p>\
                 <p>Pode fechar esta aba e voltar ao terminal.</p>",
                html_escape(error),
                html_escape(&description)
            ),
        );
        Err(KobanaError::Auth(format!(
            "authorization denied: {error}: {description}"
        )))
    } else {
        write_html(
            &stream,
            400,
            "Callback inválido",
            "<h2 style=\"color:#ef4444;\">✗ Callback inválido</h2>\
             <p>A requisição não contém um código de autorização nem um erro.</p>",
        );
        Err(KobanaError::Auth(
            "no authorization code in callback".into(),
        ))
    };

    result
}

fn write_html(stream: &std::net::TcpStream, status: u16, title: &str, body: &str) {
    let status_line = match status {
        200 => "200 OK",
        400 => "400 Bad Request",
        _ => "200 OK",
    };
    let html = format!(
        "<!DOCTYPE html><html lang=\"pt-BR\"><head>\
         <meta charset=\"utf-8\"><title>{title}</title>\
         <style>body{{font-family:-apple-system,system-ui,sans-serif;\
         max-width:480px;margin:80px auto;padding:24px;text-align:center;\
         color:#1f2937;}}h2{{margin:0 0 16px;}}p{{margin:8px 0;line-height:1.5;}}</style>\
         </head><body>{body}</body></html>"
    );
    let response = format!(
        "HTTP/1.1 {status_line}\r\nContent-Type: text/html; charset=utf-8\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{html}",
        html.len()
    );
    let mut writer = std::io::BufWriter::new(stream);
    let _ = writer.write_all(response.as_bytes());
    let _ = writer.flush();
}

fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hex = &s[i + 1..i + 3];
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    out.push(byte);
                } else {
                    out.push(b'%');
                }
                i += 3;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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
