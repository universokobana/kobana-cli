use crate::config::Environment;
use kobana::error::KobanaError;
use kobana::i18n::{self, Lang};
use kobana::{tr, trf};
use serde::Deserialize;
use sha2::Digest;
use std::io::{BufRead, Write};
use std::net::TcpListener;

/// The Conta Kobana — the authorization server that issues the tokens of
/// every product. Production uses the apex, other environments enter as a
/// segment (`account.sandbox.kobana.com.br`); development is the local
/// kobana-app. `KOBANA_ACCOUNT_URL` overrides it (e.g. dev behind a tunnel).
pub fn authorization_server(env: &Environment) -> String {
    if let Ok(url) = std::env::var("KOBANA_ACCOUNT_URL") {
        if !url.is_empty() {
            return url.trim_end_matches('/').to_string();
        }
    }
    default_authorization_server(env)
}

/// The Conta Kobana of an environment, ignoring `KOBANA_ACCOUNT_URL`
pub fn default_authorization_server(env: &Environment) -> String {
    match env {
        Environment::Production => "https://account.kobana.com.br".into(),
        Environment::Sandbox => "https://account.sandbox.kobana.com.br".into(),
        Environment::Development => "http://localhost:5005".into(),
    }
}

fn oauth_authorize_url(env: &Environment) -> String {
    format!("{}/oauth/authorize", authorization_server(env))
}

fn oauth_token_url(env: &Environment) -> String {
    format!("{}/oauth/token", authorization_server(env))
}

/// Access levels: global verbs, valid for every resource of the token. A
/// resource scope alone only reads; `write`, `delete`, `import` and `export`
/// each unlock one kind of operation and do not imply one another.
pub const ACCESS_LEVELS: &[(&str, [&str; 3])] = &[
    ("read", ["Leitura", "Read", "Lectura"]),
    ("write", ["Criação e alteração", "Create and update", "Creación y modificación"]),
    ("delete", ["Exclusão", "Delete", "Eliminación"]),
    ("import", ["Importação em volume", "Bulk import", "Importación masiva"]),
    ("export", ["Exportação em volume", "Bulk export", "Exportación masiva"]),
];

/// The resource scopes a product publishes, and where they came from
#[derive(Debug)]
pub struct ScopeCatalog {
    /// `{product}/{resource}` scopes, sorted
    pub resources: Vec<String>,
    /// The document they were read from
    pub source: String,
}

/// RFC 9728 metadata URL for a resource: the well-known path goes between
/// the host and the resource's own path (`http://h/mcp` →
/// `http://h/.well-known/oauth-protected-resource/mcp`)
pub fn protected_resource_metadata_url(resource: &str) -> Result<String, KobanaError> {
    let url = reqwest::Url::parse(resource)
        .map_err(|e| KobanaError::Internal(format!("{resource}: {e}")))?;
    let path = url.path().trim_end_matches('/');
    let mut base = url.clone();
    base.set_path("");
    base.set_query(None);
    Ok(format!(
        "{}/.well-known/oauth-protected-resource{path}",
        base.as_str().trim_end_matches('/')
    ))
}

/// The `{product}/…` scopes of a metadata document, without the catch-all
/// `{product}/all`, which the apps do not advertise either
pub fn product_scopes(metadata: &serde_json::Value, product: &str) -> Vec<String> {
    let prefix = format!("{product}/");
    let mut scopes: Vec<String> = metadata["scopes_supported"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|s| s.as_str())
        .filter(|s| s.starts_with(&prefix) && *s != format!("{product}/all"))
        .map(String::from)
        .collect();
    scopes.sort();
    scopes.dedup();
    scopes
}

/// The `User-Agent` of every request the CLI sends to the Conta Kobana
fn user_agent() -> String {
    format!("kobana-cli/{}", env!("CARGO_PKG_VERSION"))
}

/// The HTTP client of every OAuth request (discovery, token, refresh):
/// identified, with a timeout, and asking for JSON
fn http_client() -> Result<reqwest::Client, KobanaError> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"));
    reqwest::Client::builder()
        .user_agent(user_agent())
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível criar o cliente HTTP: {e}",
            en: "failed to build client: {e}",
            es: "no se pudo crear el cliente HTTP: {e}",
        )))
}

async fn fetch_json(url: &str) -> Result<serde_json::Value, String> {
    let client = http_client().map_err(|e| e.to_string())?;
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = response.status();
    if !status.is_success() {
        return Err(status.to_string());
    }
    response.json().await.map_err(|e| e.to_string())
}

/// The scopes a product publishes in its `/.well-known/oauth-protected-resource`.
///
/// When the app does not publish the document (or it lists none of its own
/// scopes), falls back to the authorization server's catalogue filtered by
/// the product prefix — the same registry the app's document is built from.
pub async fn discover_scopes(
    product: &str,
    mcp_resource: &str,
    env: &Environment,
) -> Result<ScopeCatalog, KobanaError> {
    let metadata_url = protected_resource_metadata_url(mcp_resource)?;
    let app_error = match fetch_json(&metadata_url).await {
        Ok(doc) => {
            let resources = product_scopes(&doc, product);
            if !resources.is_empty() {
                return Ok(ScopeCatalog { resources, source: metadata_url });
            }
            tr!(pt: "nenhum escopo do produto", en: "no product scopes", es: "ningún alcance del producto").to_string()
        }
        Err(e) => e,
    };

    let as_url = format!(
        "{}/.well-known/oauth-authorization-server",
        authorization_server(env)
    );
    match fetch_json(&as_url).await {
        Ok(doc) => {
            let resources = product_scopes(&doc, product);
            if resources.is_empty() {
                return Err(KobanaError::Auth(trf!(
                    pt: "a Conta Kobana não publica escopos de {product} ({metadata_url}: {app_error})",
                    en: "the Conta Kobana publishes no {product} scopes ({metadata_url}: {app_error})",
                    es: "la Conta Kobana no publica alcances de {product} ({metadata_url}: {app_error})",
                )));
            }
            Ok(ScopeCatalog { resources, source: as_url })
        }
        Err(as_error) => Err(KobanaError::Auth(trf!(
            pt: "não foi possível obter os escopos de {product}: {metadata_url}: {app_error}; {as_url}: {as_error}",
            en: "could not fetch the {product} scopes: {metadata_url}: {app_error}; {as_url}: {as_error}",
            es: "no se pudieron obtener los alcances de {product}: {metadata_url}: {app_error}; {as_url}: {as_error}",
        ))),
    }
}

/// Default public client ID for the Kobana CLI (PKCE, no secret needed)
pub const DEFAULT_CLIENT_ID: &str = "kobana-cli";

/// Labels for the banking resource scopes, in Portuguese, English and
/// Spanish. Only labels: the scopes offered at login come from the app's
/// published metadata, and one missing here is shown without a description.
pub const BANKING_SCOPE_LABELS: &[(&str, [&str; 3])] = &[
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

/// Labels for the banking resource groups (the segment before the first
/// `.`), used on the first screen of the interactive scope selector.
pub const BANKING_GROUP_LABELS: &[(&str, [&str; 3])] = &[
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

/// Authorization Code flow with PKCE: open browser, listen for callback.
/// The CLI is a public client: no secret, which it could not keep, and which
/// a refresh would need again.
pub async fn authorization_code_flow(
    client_id: &str,
    scopes: &str,
    resource: &str,
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

    let state = generate_code_verifier();
    let authorize_url = reqwest::Url::parse_with_params(
        &oauth_authorize_url(env),
        &[
            ("client_id", client_id),
            ("redirect_uri", redirect_uri.as_str()),
            ("response_type", "code"),
            ("code_challenge", code_challenge.as_str()),
            ("code_challenge_method", "S256"),
            ("scope", scopes),
            ("resource", resource),
            ("state", state.as_str()),
        ],
    )
    .map_err(|e| KobanaError::Internal(e.to_string()))?
    .to_string();

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
    let code = wait_for_callback(listener, &state)?;

    let token_url = oauth_token_url(env);

    exchange_code(
        &token_url,
        client_id,
        &code,
        &redirect_uri,
        &code_verifier,
        resource,
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

    let client = http_client()?;
    let response = client
        .post(&token_url)
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

/// Refresh an access token at `issuer`, the authorization server that
/// issued it — never wherever `KOBANA_ACCOUNT_URL` points now, so the refresh
/// token cannot be sent to another server. A public client identifies itself
/// with its `client_id`, and `resource` keeps the new token bound to the same
/// `aud`.
pub async fn refresh_token(
    refresh_token: &str,
    client_id: &str,
    resource: Option<&str>,
    issuer: &str,
) -> Result<TokenResponse, KobanaError> {
    let token_url = format!("{}/oauth/token", issuer.trim_end_matches('/'));

    let mut form = vec![
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
    ];
    if let Some(resource) = resource {
        form.push(("resource", resource));
    }

    let client = http_client()?;
    let response = client
        .post(&token_url)
        .form(&form)
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
    code: &str,
    redirect_uri: &str,
    code_verifier: &str,
    resource: &str,
    verbose: bool,
) -> Result<TokenResponse, KobanaError> {
    let user_agent = user_agent();
    let client = http_client()?;

    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("code_verifier", code_verifier),
        ("resource", resource),
    ];

    if verbose {
        eprintln!("\n--- POST {token_url} ---");
        eprintln!("User-Agent: {user_agent}");
        eprintln!("Content-Type: application/x-www-form-urlencoded");
        eprintln!("Accept: application/json");
        eprintln!("{}", tr!(pt: "Corpo (form):", en: "Body (form):", es: "Cuerpo (form):"));
        for (k, v) in &params {
            let display = if *k == "code" || *k == "code_verifier" {
                redact(v)
            } else {
                v.to_string()
            };
            eprintln!("  {k}={display}");
        }
        eprintln!("---");
    }

    let response = client
        .post(token_url)
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
            eprintln!("{name}: {}", header_for_log(name.as_str(), value.to_str().unwrap_or("<binary>")));
        }
        eprintln!();
        let size = body.len();
        let shown = redact_token_body(&body);
        eprintln!(
            "{}",
            trf!(
                pt: "Corpo ({size} bytes): {shown}",
                en: "Body ({size} bytes): {shown}",
                es: "Cuerpo ({size} bytes): {shown}",
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

/// Headers whose value is a credential: a session cookie of the Conta Kobana
/// is as good as a login
const SECRET_HEADERS: &[&str] = &["set-cookie", "cookie", "authorization", "proxy-authorization"];

/// A header value as `--verbose` prints it: a credential is reduced to its
/// cookie name (or omitted), the rest is shown as is. `--verbose` output ends
/// up in CI logs.
pub(crate) fn header_for_log(name: &str, value: &str) -> String {
    if !SECRET_HEADERS.iter().any(|h| name.eq_ignore_ascii_case(h)) {
        return value.to_string();
    }
    let hidden = tr!(pt: "<omitido>", en: "<redacted>", es: "<omitido>");
    match value.split_once('=') {
        // `Set-Cookie: _session=abc; path=/` → `_session=<redacted>`
        Some((cookie, _)) if name.to_ascii_lowercase().contains("cookie") => format!("{}={hidden}", cookie.trim()),
        _ => hidden.to_string(),
    }
}

/// The first characters of a secret, enough to tell two apart in a log
fn redact(value: &str) -> String {
    let prefix: String = value.chars().take(8).collect();
    format!("{prefix}...")
}

/// A token response as `--verbose` prints it: the tokens cut to a prefix, as
/// the request's `code`, `code_verifier` and `client_secret` are. `--verbose`
/// output ends up in CI logs.
fn redact_token_body(body: &str) -> String {
    const SECRETS: [&str; 3] = ["access_token", "refresh_token", "id_token"];
    match serde_json::from_str::<serde_json::Value>(body) {
        Ok(serde_json::Value::Object(mut obj)) => {
            for key in SECRETS {
                if let Some(serde_json::Value::String(token)) = obj.get(key) {
                    let shown = redact(token);
                    obj.insert(key.to_string(), serde_json::Value::String(shown));
                }
            }
            serde_json::Value::Object(obj).to_string()
        }
        Ok(other) => other.to_string(),
        // Not JSON: nothing structured to keep, and it may still hold a token
        Err(_) if SECRETS.iter().any(|k| body.contains(k)) => {
            tr!(pt: "<omitido>", en: "<redacted>", es: "<omitido>").to_string()
        }
        Err(_) => body.to_string(),
    }
}

fn wait_for_callback(listener: TcpListener, state: &str) -> Result<String, KobanaError> {
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
    let state_matches = query.get("state").map(|s| s.as_str()) == Some(state);
    let result = if query.contains_key("code") && !state_matches {
        write_page(
            &stream,
            &CallbackPage::failure(
                tr!(pt: "Retorno inválido", en: "Invalid callback", es: "Retorno inválido"),
                tr!(
                    pt: "Este retorno não pertence ao login em andamento no terminal. Rode o login de novo.",
                    en: "This callback does not belong to the login running in the terminal. Run the login again.",
                    es: "Este retorno no corresponde al inicio de sesión en curso en la terminal. Vuelve a iniciar sesión.",
                ),
                Some(("state", tr!(pt: "não confere", en: "does not match", es: "no coincide"))),
            ),
        );
        Err(KobanaError::Auth(
            tr!(
                pt: "o retorno da autorização não trouxe o `state` deste login",
                en: "the authorization callback does not carry this login's `state`",
                es: "el retorno de la autorización no trajo el `state` de este inicio de sesión",
            )
            .into(),
        ))
    } else if let Some(code) = query.get("code") {
        write_page(&stream, &CallbackPage::success());
        Ok(code.clone())
    } else if let Some(error) = query.get("error") {
        let description = query
            .get("error_description")
            .cloned()
            .unwrap_or_else(|| error.clone());
        write_page(
            &stream,
            &CallbackPage::failure(
                tr!(pt: "Autorização negada", en: "Authorization denied", es: "Autorización denegada"),
                tr!(
                    pt: "A Conta Kobana não liberou o acesso. Volte ao terminal e tente entrar de novo.",
                    en: "Conta Kobana did not grant access. Go back to the terminal and try to log in again.",
                    es: "Conta Kobana no concedió el acceso. Vuelve a la terminal e intenta iniciar sesión de nuevo.",
                ),
                Some((error, &description)),
            ),
        );
        Err(KobanaError::Auth(trf!(
            pt: "autorização negada: {error}: {description}",
            en: "authorization denied: {error}: {description}",
            es: "autorización denegada: {error}: {description}",
        )))
    } else {
        write_page(
            &stream,
            &CallbackPage::failure(
                tr!(pt: "Retorno inválido", en: "Invalid callback", es: "Retorno inválido"),
                tr!(
                    pt: "A requisição não contém um código de autorização nem um erro.",
                    en: "The request carries neither an authorization code nor an error.",
                    es: "La solicitud no contiene un código de autorización ni un error.",
                ),
                None,
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

const CALLBACK_TEMPLATE: &str = include_str!("../assets/callback.html");
const STORK_PNG: &[u8] = include_bytes!("../assets/cegonha.png");
const LOGO_PNG: &[u8] = include_bytes!("../assets/kobana-logo-negativo.png");
/// The Kobana "K" symbol in lime, percent-encoded for a `data:` URI
const FAVICON_SVG: &str = "%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 512 512'%3E%3Crect width='512' height='512' rx='96' fill='%23000'/%3E%3Cpath fill='%23D3FD54' d='M91 256.5V336h62.4l39.6-23.5c21.8-12.9 40.3-23.7 41.2-24 .9-.4 17.7 8.9 42.3 23.5l40.8 24 51.7-.2 51.7-.3-65-38.5c-35.7-21.2-65.6-39-66.4-39.6-1.2-.8 12.4-9.3 65.1-40.5l66.5-39.4-51.6-.3-51.6-.2-81.7 48c-45 26.4-82 48-82.4 48-.3 0-.6-21.6-.6-48v-48H91v79.5z'/%3E%3C/svg%3E";

/// What the browser tab shows when the authorization server redirects back
struct CallbackPage<'a> {
    status: u16,
    /// Brand colour of the headline: `lime` for success, `purple` otherwise
    accent: &'static str,
    headline: &'static str,
    title: &'static str,
    message: &'static str,
    /// A label and a value shown verbatim (e.g. an OAuth error and its description)
    detail: Option<(&'a str, &'a str)>,
}

impl<'a> CallbackPage<'a> {
    fn success() -> Self {
        Self {
            status: 200,
            accent: "lime",
            headline: tr!(pt: "Pronto!", en: "All set!", es: "¡Listo!"),
            title: tr!(
                pt: "Autorização concedida",
                en: "Authorization granted",
                es: "Autorización concedida",
            ),
            message: tr!(
                pt: "Você foi autenticado com sucesso. A cegonha já levou suas credenciais para o terminal.",
                en: "You have been authenticated. The stork has already delivered your credentials to the terminal.",
                es: "Te autenticaste correctamente. La cigüeña ya llevó tus credenciales a la terminal.",
            ),
            detail: None,
        }
    }

    fn failure(title: &'static str, message: &'static str, detail: Option<(&'a str, &'a str)>) -> Self {
        Self {
            status: 400,
            accent: "purple",
            headline: tr!(pt: "Ops!", en: "Oops!", es: "¡Ups!"),
            title,
            message,
            detail,
        }
    }

    fn render(&self) -> String {
        use base64::Engine;
        let b64 = |bytes: &[u8]| base64::engine::general_purpose::STANDARD.encode(bytes);
        let lang = match i18n::lang() {
            Lang::Pt => "pt-BR",
            Lang::En => "en",
            Lang::Es => "es",
        };
        let detail = self
            .detail
            .map(|(label, value)| {
                format!(
                    "<p class=\"detail\"><b>{}</b><br>{}</p>",
                    html_escape(label),
                    html_escape(value)
                )
            })
            .unwrap_or_default();
        CALLBACK_TEMPLATE
            .replace("{{lang}}", lang)
            .replace("{{accent}}", self.accent)
            .replace("{{headline}}", &html_escape(self.headline))
            .replace("{{title}}", &html_escape(self.title))
            .replace("{{message}}", &html_escape(self.message))
            .replace("{{next_step}}", &html_escape(close_tab()))
            .replace("{{detail}}", &detail)
            .replace("{{favicon}}", FAVICON_SVG)
            .replace("{{stork}}", &b64(STORK_PNG))
            .replace("{{logo}}", &b64(LOGO_PNG))
    }
}

fn close_tab() -> &'static str {
    tr!(
        pt: "Pode fechar esta aba e voltar ao terminal.",
        en: "You can close this tab and go back to the terminal.",
        es: "Puedes cerrar esta pestaña y volver a la terminal.",
    )
}

fn write_page(stream: &std::net::TcpStream, page: &CallbackPage) {
    let status_line = match page.status {
        400 => "400 Bad Request",
        _ => "200 OK",
    };
    let html = page.render();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbose_headers_hide_credentials() {
        assert_eq!(header_for_log("Set-Cookie", "_session=abc123; path=/; HttpOnly"), format!("_session={}", tr!(pt: "<omitido>", en: "<redacted>", es: "<omitido>")));
        assert!(!header_for_log("authorization", "Bearer xyz").contains("xyz"));
        assert_eq!(header_for_log("content-type", "application/json"), "application/json");
    }

    #[test]
    fn verbose_token_bodies_hide_the_tokens() {
        let body = r#"{"access_token":"AAAAAAAAsecret-access","refresh_token":"RRRRRRRRsecret-refresh","id_token":"IIIIIIIIsecret-id","token_type":"Bearer","expires_in":3600}"#;
        let shown = redact_token_body(body);
        for secret in ["secret-access", "secret-refresh", "secret-id"] {
            assert!(!shown.contains(secret), "{shown}");
        }
        assert!(shown.contains("AAAAAAAA...") && shown.contains("\"expires_in\":3600"), "{shown}");

        assert_eq!(redact_token_body(r#"{"error":"invalid_grant"}"#), r#"{"error":"invalid_grant"}"#);
        assert!(!redact_token_body("access_token=xyz&x=1").contains("xyz"));
    }

    #[test]
    fn callback_pages_fill_every_placeholder() {
        let detail = ("access_denied", "<script>alert(1)</script>");
        for page in [
            CallbackPage::success(),
            CallbackPage::failure("t", "m", Some(detail)),
            CallbackPage::failure("t", "m", None),
        ] {
            let html = page.render();
            assert!(!html.contains("{{"), "unfilled placeholder");
            assert!(!html.contains("<script>"), "detail must be escaped");
            assert!(html.contains("data:image/png;base64,iVBOR"));
        }
        assert!(CallbackPage::success().render().contains("--accent: var(--lime)"));
        assert!(CallbackPage::failure("t", "m", None).render().contains("--accent: var(--purple)"));
    }

    #[test]
    fn metadata_url_goes_between_host_and_path() {
        assert_eq!(
            protected_resource_metadata_url("https://mcp.billing.kobana.com.br").unwrap(),
            "https://mcp.billing.kobana.com.br/.well-known/oauth-protected-resource"
        );
        assert_eq!(
            protected_resource_metadata_url("http://localhost:5005/mcp").unwrap(),
            "http://localhost:5005/.well-known/oauth-protected-resource/mcp"
        );
    }

    #[test]
    fn product_scopes_keep_only_the_product_prefix() {
        let doc = serde_json::json!({
            "scopes_supported": ["billing/plans", "read", "write", "billing/all", "billing/customers", "finance/accounts"]
        });
        assert_eq!(product_scopes(&doc, "billing"), vec!["billing/customers", "billing/plans"]);
        assert!(product_scopes(&serde_json::json!({}), "billing").is_empty());
    }

    #[test]
    fn account_host_per_environment() {
        if std::env::var("KOBANA_ACCOUNT_URL").is_ok() {
            return;
        }
        assert_eq!(authorization_server(&Environment::Production), "https://account.kobana.com.br");
        assert_eq!(authorization_server(&Environment::Sandbox), "https://account.sandbox.kobana.com.br");
    }

    #[test]
    fn the_five_access_levels() {
        let names: Vec<&str> = ACCESS_LEVELS.iter().map(|(n, _)| *n).collect();
        assert_eq!(names, ["read", "write", "delete", "import", "export"]);
    }
}
