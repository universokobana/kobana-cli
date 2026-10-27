use crate::config::Environment;
use kobana::error::KobanaError;
use kobana::i18n::{self, Lang};
use kobana::{tr, trf};
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

/// All available Kobana OAuth scopes with descriptions in Portuguese, English
/// and Spanish
pub const ALL_SCOPES: &[(&str, [&str; 3])] = &[
    (
        "admin.subaccounts",
        [
            "Gerenciar subcontas da conta principal",
            "Manage the main account's subaccounts",
            "Gestionar las subcuentas de la cuenta principal",
        ],
    ),
    (
        "admin.users",
        [
            "Gerenciar usuários e permissões",
            "Manage users and permissions",
            "Gestionar usuarios y permisos",
        ],
    ),
    (
        "automation.email_accounts",
        [
            "Gerenciar contas de e-mail para automação",
            "Manage email accounts for automation",
            "Gestionar cuentas de correo para automatización",
        ],
    ),
    (
        "automation.email_deliveries",
        [
            "Visualizar e gerenciar envios de e-mail",
            "View and manage email deliveries",
            "Ver y gestionar envíos de correo",
        ],
    ),
    (
        "automation.sms_accounts",
        [
            "Gerenciar contas de SMS",
            "Manage SMS accounts",
            "Gestionar cuentas de SMS",
        ],
    ),
    (
        "automation.sms_deliveries",
        [
            "Visualizar e gerenciar envios de SMS",
            "View and manage SMS deliveries",
            "Ver y gestionar envíos de SMS",
        ],
    ),
    (
        "automation.webhook_deliveries",
        [
            "Visualizar e gerenciar entregas de webhooks",
            "View and manage webhook deliveries",
            "Ver y gestionar entregas de webhooks",
        ],
    ),
    (
        "automation.webhooks",
        [
            "Gerenciar webhooks e notificações automatizadas",
            "Manage webhooks and automated notifications",
            "Gestionar webhooks y notificaciones automatizadas",
        ],
    ),
    (
        "billing.transactions",
        [
            "Visualizar transações de cobrança e faturamento",
            "View billing and invoicing transactions",
            "Ver transacciones de cobro y facturación",
        ],
    ),
    (
        "charge.automatic_pix.accounts",
        [
            "Gerenciar contas de Pix automático",
            "Manage automatic Pix accounts",
            "Gestionar cuentas de Pix automático",
        ],
    ),
    (
        "charge.automatic_pix.locations",
        [
            "Gerenciar localizações de Pix automático",
            "Manage automatic Pix locations",
            "Gestionar ubicaciones de Pix automático",
        ],
    ),
    (
        "charge.automatic_pix.pix",
        [
            "Gerenciar cobranças Pix automáticas",
            "Manage automatic Pix charges",
            "Gestionar cobros Pix automáticos",
        ],
    ),
    (
        "charge.automatic_pix.recurrences",
        [
            "Gerenciar recorrências de Pix automático",
            "Manage automatic Pix recurrences",
            "Gestionar recurrencias de Pix automático",
        ],
    ),
    (
        "charge.automatic_pix.requests",
        [
            "Gerenciar solicitações de Pix automático",
            "Manage automatic Pix requests",
            "Gestionar solicitudes de Pix automático",
        ],
    ),
    (
        "charge.bank_billet_accounts",
        [
            "Gerenciar contas de boletos bancários",
            "Manage bank slip accounts",
            "Gestionar cuentas de boletos bancarios",
        ],
    ),
    (
        "charge.bank_billet_payments",
        [
            "Visualizar e registrar pagamentos de boletos",
            "View and record bank slip payments",
            "Ver y registrar pagos de boletos",
        ],
    ),
    (
        "charge.bank_billet_registrations",
        [
            "Gerenciar registros de boletos nos bancos",
            "Manage bank slip registrations with the banks",
            "Gestionar registros de boletos en los bancos",
        ],
    ),
    (
        "charge.bank_billets",
        [
            "Gerenciar boletos bancários",
            "Manage bank slips",
            "Gestionar boletos bancarios",
        ],
    ),
    (
        "charge.customer_subscriptions",
        [
            "Gerenciar assinaturas de clientes",
            "Manage customer subscriptions",
            "Gestionar suscripciones de clientes",
        ],
    ),
    (
        "charge.installments",
        [
            "Gerenciar parcelamentos",
            "Manage installment plans",
            "Gestionar pagos en cuotas",
        ],
    ),
    (
        "charge.payments",
        [
            "Visualizar e gerenciar pagamentos recebidos",
            "View and manage received payments",
            "Ver y gestionar pagos recibidos",
        ],
    ),
    (
        "charge.pix",
        [
            "Gerenciar cobranças Pix",
            "Manage Pix charges",
            "Gestionar cobros Pix",
        ],
    ),
    (
        "charge.pix_accounts",
        [
            "Gerenciar contas Pix",
            "Manage Pix accounts",
            "Gestionar cuentas Pix",
        ],
    ),
    (
        "core.providers",
        [
            "Gerenciar provedores do sistema (bancos e integrações)",
            "Manage system providers (banks and integrations)",
            "Gestionar proveedores del sistema (bancos e integraciones)",
        ],
    ),
    (
        "crm.customers",
        [
            "Gerenciar clientes e informações comerciais",
            "Manage customers and business information",
            "Gestionar clientes e información comercial",
        ],
    ),
    (
        "crm.people",
        [
            "Gerenciar pessoas e contatos",
            "Manage people and contacts",
            "Gestionar personas y contactos",
        ],
    ),
    (
        "data.bank_billet_queries",
        [
            "Consultar informações de boletos bancários",
            "Look up bank slip information",
            "Consultar información de boletos bancarios",
        ],
    ),
    (
        "financial.accounts",
        [
            "Gerenciar contas financeiras",
            "Manage financial accounts",
            "Gestionar cuentas financieras",
        ],
    ),
    (
        "financial.balances",
        [
            "Visualizar saldos e movimentações financeiras",
            "View balances and financial activity",
            "Ver saldos y movimientos financieros",
        ],
    ),
    (
        "financial.providers",
        [
            "Gerenciar provedores financeiros e integrações bancárias",
            "Manage financial providers and bank integrations",
            "Gestionar proveedores financieros e integraciones bancarias",
        ],
    ),
    (
        "financial.statement_transactions",
        [
            "Visualizar transações e extratos financeiros",
            "View financial transactions and statements",
            "Ver transacciones y extractos financieros",
        ],
    ),
    (
        "integration.certificates",
        [
            "Gerenciar certificados digitais para integrações",
            "Manage digital certificates for integrations",
            "Gestionar certificados digitales para integraciones",
        ],
    ),
    (
        "integration.connections",
        [
            "Gerenciar conexões com bancos e provedores",
            "Manage connections with banks and providers",
            "Gestionar conexiones con bancos y proveedores",
        ],
    ),
    (
        "integration.discharges",
        [
            "Gerenciar arquivos de retorno bancário (baixas)",
            "Manage bank return files (settlements)",
            "Gestionar archivos de retorno bancario (bajas)",
        ],
    ),
    (
        "integration.edi_boxes",
        [
            "Gerenciar caixas postais EDI para troca de arquivos",
            "Manage EDI mailboxes for file exchange",
            "Gestionar buzones EDI para intercambio de archivos",
        ],
    ),
    (
        "integration.remittances",
        [
            "Gerenciar arquivos de remessa bancária",
            "Manage bank remittance files",
            "Gestionar archivos de remesa bancaria",
        ],
    ),
    (
        "login",
        [
            "Autenticar com o usuário",
            "Authenticate as the user",
            "Autenticarse con el usuario",
        ],
    ),
    (
        "mailbox.entries",
        [
            "Gerenciar caixas postais para recebimento de arquivos",
            "Manage mailboxes for receiving files",
            "Gestionar buzones para recibir archivos",
        ],
    ),
    (
        "mailbox.files",
        [
            "Visualizar e gerenciar arquivos nas caixas postais",
            "View and manage files in the mailboxes",
            "Ver y gestionar archivos en los buzones",
        ],
    ),
    (
        "partner.bank_contracts",
        [
            "Gerenciar contratos bancários com parceiros",
            "Manage bank contracts with partners",
            "Gestionar contratos bancarios con socios",
        ],
    ),
    (
        "payment.bank_billets",
        [
            "Efetuar pagamentos de boletos bancários",
            "Pay bank slips",
            "Pagar boletos bancarios",
        ],
    ),
    (
        "payment.batches",
        [
            "Gerenciar lotes de pagamentos",
            "Manage payment batches",
            "Gestionar lotes de pagos",
        ],
    ),
    (
        "payment.darfs",
        [
            "Efetuar pagamentos de DARFs (tributos federais)",
            "Pay DARFs (federal taxes)",
            "Pagar DARFs (impuestos federales)",
        ],
    ),
    (
        "payment.dda",
        [
            "Gerenciar DDA (Débito Direto Autorizado)",
            "Manage DDA (Authorized Direct Debit)",
            "Gestionar DDA (Débito Directo Autorizado)",
        ],
    ),
    (
        "payment.dda.bank_billets",
        [
            "Visualizar e gerenciar boletos DDA disponíveis para pagamento",
            "View and manage DDA bank slips available for payment",
            "Ver y gestionar boletos DDA disponibles para pago",
        ],
    ),
    (
        "payment.dda_accounts",
        [
            "Gerenciar contas DDA (Débito Direto Autorizado)",
            "Manage DDA (Authorized Direct Debit) accounts",
            "Gestionar cuentas DDA (Débito Directo Autorizado)",
        ],
    ),
    (
        "payment.payments",
        [
            "Gerenciar todos os tipos de pagamentos através da API",
            "Manage every kind of payment through the API",
            "Gestionar todos los tipos de pagos a través de la API",
        ],
    ),
    (
        "payment.pix",
        [
            "Efetuar pagamentos via Pix",
            "Pay via Pix",
            "Pagar vía Pix",
        ],
    ),
    (
        "payment.taxes",
        [
            "Efetuar pagamentos de impostos e taxas",
            "Pay taxes and fees",
            "Pagar impuestos y tasas",
        ],
    ),
    (
        "payment.utilities",
        [
            "Efetuar pagamentos de contas de consumo (água, luz, etc)",
            "Pay utility bills (water, power, etc.)",
            "Pagar facturas de servicios (agua, luz, etc.)",
        ],
    ),
    (
        "security.access_tokens",
        [
            "Gerenciar tokens de acesso e autenticação",
            "Manage access and authentication tokens",
            "Gestionar tokens de acceso y autenticación",
        ],
    ),
    (
        "system.events",
        [
            "Visualizar eventos e logs do sistema",
            "View system events and logs",
            "Ver eventos y logs del sistema",
        ],
    ),
    (
        "system.imports",
        [
            "Gerenciar importações de dados",
            "Manage data imports",
            "Gestionar importaciones de datos",
        ],
    ),
    (
        "system.reports",
        [
            "Gerar e visualizar relatórios do sistema",
            "Generate and view system reports",
            "Generar y ver informes del sistema",
        ],
    ),
    (
        "transfer.batches",
        [
            "Gerenciar lotes de transferências",
            "Manage transfer batches",
            "Gestionar lotes de transferencias",
        ],
    ),
    (
        "transfer.internal",
        [
            "Realizar transferências internas entre contas",
            "Make internal transfers between accounts",
            "Realizar transferencias internas entre cuentas",
        ],
    ),
    (
        "transfer.pix",
        [
            "Realizar transferências via Pix",
            "Make transfers via Pix",
            "Realizar transferencias vía Pix",
        ],
    ),
    (
        "transfer.ted",
        [
            "Realizar transferências TED (Transferência Eletrônica Disponível)",
            "Make TED transfers (Electronic Available Transfer)",
            "Realizar transferencias TED (Transferencia Electrónica Disponible)",
        ],
    ),
    (
        "transfer.transfers",
        [
            "Gerenciar todos os tipos de transferências (Pix, TED, Internas) via endpoint unificado",
            "Manage every kind of transfer (Pix, TED, internal) via the unified endpoint",
            "Gestionar todos los tipos de transferencias (Pix, TED, internas) vía endpoint unificado",
        ],
    ),
];

/// Top-level resource groups with human-readable descriptions in Portuguese,
/// English and Spanish. Used for the first step of the interactive scope
/// selector.
pub const TOP_LEVEL_GROUPS: &[(&str, [&str; 3])] = &[
    (
        "admin",
        [
            "Administração (subcontas, usuários)",
            "Administration (subaccounts, users)",
            "Administración (subcuentas, usuarios)",
        ],
    ),
    (
        "automation",
        [
            "Automação (e-mail, SMS, webhooks)",
            "Automation (email, SMS, webhooks)",
            "Automatización (correo, SMS, webhooks)",
        ],
    ),
    (
        "billing",
        [
            "Cobrança e faturamento",
            "Charging and invoicing",
            "Cobro y facturación",
        ],
    ),
    (
        "charge",
        [
            "Cobranças (boletos, Pix, parcelamentos)",
            "Charges (bank slips, Pix, installments)",
            "Cobros (boletos, Pix, cuotas)",
        ],
    ),
    (
        "core",
        [
            "Sistema central (provedores)",
            "Core system (providers)",
            "Sistema central (proveedores)",
        ],
    ),
    (
        "crm",
        [
            "CRM (clientes, pessoas)",
            "CRM (customers, people)",
            "CRM (clientes, personas)",
        ],
    ),
    (
        "data",
        [
            "Consultas de dados",
            "Data lookups",
            "Consultas de datos",
        ],
    ),
    (
        "financial",
        [
            "Financeiro (contas, saldos, extratos)",
            "Financial (accounts, balances, statements)",
            "Financiero (cuentas, saldos, extractos)",
        ],
    ),
    (
        "integration",
        [
            "Integrações (certificados, conexões, arquivos)",
            "Integrations (certificates, connections, files)",
            "Integraciones (certificados, conexiones, archivos)",
        ],
    ),
    (
        "login",
        [
            "Autenticar com o usuário",
            "Authenticate as the user",
            "Autenticarse con el usuario",
        ],
    ),
    (
        "mailbox",
        [
            "Caixas postais (EDI, arquivos)",
            "Mailboxes (EDI, files)",
            "Buzones (EDI, archivos)",
        ],
    ),
    (
        "partner",
        [
            "Parceiros (contratos bancários)",
            "Partners (bank contracts)",
            "Socios (contratos bancarios)",
        ],
    ),
    (
        "payment",
        [
            "Pagamentos (boletos, Pix, tributos, contas)",
            "Payments (bank slips, Pix, taxes, bills)",
            "Pagos (boletos, Pix, impuestos, facturas)",
        ],
    ),
    (
        "security",
        [
            "Segurança (tokens de acesso)",
            "Security (access tokens)",
            "Seguridad (tokens de acceso)",
        ],
    ),
    (
        "system",
        [
            "Sistema (eventos, imports, relatórios)",
            "System (events, imports, reports)",
            "Sistema (eventos, importaciones, informes)",
        ],
    ),
    (
        "transfer",
        [
            "Transferências (Pix, TED, internas)",
            "Transfers (Pix, TED, internal)",
            "Transferencias (Pix, TED, internas)",
        ],
    ),
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
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível abrir o servidor de retorno: {e}",
            en: "failed to bind callback server: {e}",
            es: "no se pudo abrir el servidor de retorno: {e}",
        )))?;
    let port = listener.local_addr().unwrap().port();
    let redirect_uri = format!("http://127.0.0.1:{port}");

    // Generate PKCE pair
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);

    let authorize_base = oauth_authorize_url(env);

    let authorize_url = format!(
        "{authorize_base}?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code&code_challenge={code_challenge}&code_challenge_method=S256&scope={scopes}"
    );

    eprintln!(
        "{}",
        tr!(
            pt: "Abrindo o navegador para autorizar...",
            en: "Opening browser for authorization...",
            es: "Abriendo el navegador para autorizar...",
        )
    );
    eprintln!(
        "{}",
        trf!(
            pt: "Se o navegador não abrir, acesse: {authorize_url}",
            en: "If the browser doesn't open, visit: {authorize_url}",
            es: "Si el navegador no se abre, visita: {authorize_url}",
        )
    );

    let _ = open_browser(&authorize_url);

    eprintln!(
        "{}",
        trf!(
            pt: "Aguardando o retorno da autorização na porta {port}...",
            en: "Waiting for authorization callback on port {port}...",
            es: "Esperando el retorno de la autorización en el puerto {port}...",
        )
    );
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
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "falha ao pedir o token: {e}",
            en: "token request failed: {e}",
            es: "falló la solicitud del token: {e}",
        )))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(KobanaError::Auth(trf!(
            pt: "falha ao pedir o token ({status}): {body}",
            en: "token request failed ({status}): {body}",
            es: "falló la solicitud del token ({status}): {body}",
        )));
    }

    serde_json::from_str(&body)
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "resposta de token inválida: {e}",
            en: "invalid token response: {e}",
            es: "respuesta de token inválida: {e}",
        )))
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
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "falha ao renovar o token: {e}",
            en: "refresh request failed: {e}",
            es: "falló la renovación del token: {e}",
        )))?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(KobanaError::Auth(trf!(
            pt: "falha ao renovar o token ({status}): {body}",
            en: "refresh failed ({status}): {body}",
            es: "falló la renovación del token ({status}): {body}",
        )));
    }

    serde_json::from_str(&body)
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "resposta de renovação inválida: {e}",
            en: "invalid refresh response: {e}",
            es: "respuesta de renovación inválida: {e}",
        )))
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
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível criar o cliente HTTP: {e}",
            en: "failed to build client: {e}",
            es: "no se pudo crear el cliente HTTP: {e}",
        )))?;

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
        eprintln!("{}", tr!(pt: "Corpo (form):", en: "Body (form):", es: "Cuerpo (form):"));
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
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "falha ao trocar o código pelo token: {e}",
            en: "token exchange failed: {e}",
            es: "falló el canje del código por el token: {e}",
        )))?;

    let status = response.status();
    let version = response.version();
    let headers = response.headers().clone();
    let body = response.text().await.unwrap_or_default();

    if verbose {
        eprintln!(
            "--- {} {status} ({version:?}) ---",
            tr!(pt: "Resposta:", en: "Response:", es: "Respuesta:")
        );
        for (name, value) in headers.iter() {
            eprintln!("{name}: {}", value.to_str().unwrap_or("<binary>"));
        }
        eprintln!();
        let size = body.len();
        eprintln!(
            "{}",
            trf!(
                pt: "Corpo ({size} bytes): {body}",
                en: "Body ({size} bytes): {body}",
                es: "Cuerpo ({size} bytes): {body}",
            )
        );
        eprintln!("---\n");
    }

    if !status.is_success() {
        return Err(KobanaError::Auth(trf!(
            pt: "falha ao trocar o código pelo token ({status}): {body}",
            en: "token exchange failed ({status}): {body}",
            es: "falló el canje del código por el token ({status}): {body}",
        )));
    }

    serde_json::from_str(&body)
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "resposta de token inválida: {e}",
            en: "invalid token response: {e}",
            es: "respuesta de token inválida: {e}",
        )))
}

fn wait_for_callback(listener: TcpListener) -> Result<String, KobanaError> {
    listener
        .set_nonblocking(false)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "falha ao configurar o servidor de retorno: {e}",
            en: "failed to set blocking: {e}",
            es: "falló la configuración del servidor de retorno: {e}",
        )))?;

    let (stream, _) = listener
        .accept()
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "falha ao receber o retorno: {e}",
            en: "failed to accept callback: {e}",
            es: "falló la recepción del retorno: {e}",
        )))?;

    let mut reader = std::io::BufReader::new(&stream);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "falha ao ler o retorno: {e}",
            en: "failed to read callback: {e}",
            es: "falló la lectura del retorno: {e}",
        )))?;

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
        let title = tr!(
            pt: "Autorização concedida",
            en: "Authorization granted",
            es: "Autorización concedida",
        );
        write_html(
            &stream,
            200,
            title,
            &format!(
                "<h2 style=\"color:#10b981;\">✓ {title}</h2>\
                 <p><strong>{}</strong></p>\
                 <p>{}</p>",
                tr!(
                    pt: "Você foi autenticado com sucesso.",
                    en: "You have been authenticated.",
                    es: "Te autenticaste correctamente.",
                ),
                close_tab()
            ),
        );
        Ok(code.clone())
    } else if let Some(error) = query.get("error") {
        let description = query
            .get("error_description")
            .cloned()
            .unwrap_or_else(|| error.clone());
        let title = tr!(
            pt: "Autorização negada",
            en: "Authorization denied",
            es: "Autorización denegada",
        );
        write_html(
            &stream,
            400,
            title,
            &format!(
                "<h2 style=\"color:#ef4444;\">✗ {title}</h2>\
                 <p><strong>{}</strong></p>\
                 <p>{}</p>\
                 <p>{}</p>",
                html_escape(error),
                html_escape(&description),
                close_tab()
            ),
        );
        Err(KobanaError::Auth(trf!(
            pt: "autorização negada: {error}: {description}",
            en: "authorization denied: {error}: {description}",
            es: "autorización denegada: {error}: {description}",
        )))
    } else {
        let title = tr!(
            pt: "Retorno inválido",
            en: "Invalid callback",
            es: "Retorno inválido",
        );
        write_html(
            &stream,
            400,
            title,
            &format!(
                "<h2 style=\"color:#ef4444;\">✗ {title}</h2><p>{}</p>",
                tr!(
                    pt: "A requisição não contém um código de autorização nem um erro.",
                    en: "The request carries neither an authorization code nor an error.",
                    es: "La solicitud no contiene un código de autorización ni un error.",
                )
            ),
        );
        Err(KobanaError::Auth(
            tr!(
                pt: "o retorno não trouxe um código de autorização",
                en: "no authorization code in callback",
                es: "el retorno no trajo un código de autorización",
            )
            .into(),
        ))
    };

    result
}

fn close_tab() -> &'static str {
    tr!(
        pt: "Pode fechar esta aba e voltar ao terminal.",
        en: "You can close this tab and go back to the terminal.",
        es: "Puedes cerrar esta pestaña y volver a la terminal.",
    )
}

fn write_html(stream: &std::net::TcpStream, status: u16, title: &str, body: &str) {
    let html_lang = match i18n::lang() {
        Lang::Pt => "pt-BR",
        Lang::En => "en",
        Lang::Es => "es",
    };
    let status_line = match status {
        200 => "200 OK",
        400 => "400 Bad Request",
        _ => "200 OK",
    };
    let html = format!(
        "<!DOCTYPE html><html lang=\"{html_lang}\"><head>\
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
            .map_err(|e| KobanaError::Internal(trf!(
                pt: "não foi possível abrir o navegador: {e}",
                en: "failed to open browser: {e}",
                es: "no se pudo abrir el navegador: {e}",
            )))?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| KobanaError::Internal(trf!(
                pt: "não foi possível abrir o navegador: {e}",
                en: "failed to open browser: {e}",
                es: "no se pudo abrir el navegador: {e}",
            )))?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn()
            .map_err(|e| KobanaError::Internal(trf!(
                pt: "não foi possível abrir o navegador: {e}",
                en: "failed to open browser: {e}",
                es: "no se pudo abrir el navegador: {e}",
            )))?;
    }
    Ok(())
}
