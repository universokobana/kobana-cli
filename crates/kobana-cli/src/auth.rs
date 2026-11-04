use kobana::error::KobanaError;
use kobana::{tr, trf};

use crate::config::Environment;
use crate::credential_store::{self, StoredCredentials};
use crate::oauth;
use crate::product::Product;

/// Seconds before `expires_at` at which a saved token counts as expired, so
/// it is not refreshed a moment too late for the request that uses it
const EXPIRY_SKEW: i64 = 30;

/// Where a product's token came from
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSource {
    /// `KOBANA_<PRODUCT>_TOKEN` (or one of the product's aliases)
    ProductEnv(String),
    /// `KOBANA_CREDENTIALS_FILE`
    CredentialsFile(String),
    /// `kobana <product> auth login`
    Saved,
}

impl TokenSource {
    pub fn label(&self) -> String {
        match self {
            Self::ProductEnv(var) => var.clone(),
            Self::CredentialsFile(_) => "KOBANA_CREDENTIALS_FILE".into(),
            Self::Saved => "saved_credentials".into(),
        }
    }
}

/// The env vars read as a product's own token, in priority order
pub fn product_token_vars(product: &Product) -> Vec<String> {
    let mut vars = vec![product.token_env()];
    vars.extend(product.token_env_aliases.iter().map(|v| v.to_string()));
    vars
}

fn non_empty_env(var: &str) -> Option<String> {
    std::env::var(var).ok().filter(|v| !v.is_empty())
}

/// A token set in the environment for this product, without touching saved
/// credentials
pub fn env_token(product: &Product) -> Option<(String, TokenSource)> {
    for var in product_token_vars(product) {
        if let Some(token) = non_empty_env(&var) {
            return Some((token, TokenSource::ProductEnv(var)));
        }
    }
    None
}

/// A product's token and what is known about it, from wherever it came
#[derive(Debug, Clone)]
pub struct ResolvedToken {
    pub access_token: String,
    pub source: TokenSource,
    pub token_type: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    pub environment: Option<String>,
    pub resource: Option<String>,
}

/// Resolve a product's token and what is known about it.
///
/// Priority:
/// 1. `KOBANA_<PRODUCT>_TOKEN` (and the product's aliases, e.g. `KADU_TOKEN`)
/// 2. `KOBANA_CREDENTIALS_FILE` — a JSON file with `access_token`, when it
///    is this product's and environment's (see [`CredentialsFile::serves`])
/// 3. Saved credentials from `kobana <product> auth login`, refreshed with
///    the refresh token when expired
pub async fn resolve(product: &Product, env: &Environment) -> Result<ResolvedToken, KobanaError> {
    if let Some((token, source)) = env_token(product) {
        return Ok(ResolvedToken {
            access_token: token,
            source,
            // The CLI sends it as a Bearer token
            token_type: "Bearer".into(),
            refresh_token: None,
            expires_at: None,
            environment: None,
            resource: None,
        });
    }

    if let Some(file) = credentials_file_for(product, env)? {
        return Ok(ResolvedToken {
            access_token: file.access_token,
            source: TokenSource::CredentialsFile(credentials_file_path().unwrap_or_default()),
            token_type: file.token_type.unwrap_or_else(|| "Bearer".into()),
            refresh_token: file.refresh_token,
            expires_at: file.expires_at,
            environment: file.environment,
            resource: file.resource,
        });
    }

    let creds = saved_credentials(product, env).await?;
    Ok(ResolvedToken {
        access_token: creds.access_token,
        source: TokenSource::Saved,
        token_type: creds.token_type,
        refresh_token: creds.refresh_token,
        expires_at: creds.expires_at,
        environment: Some(creds.environment),
        resource: creds.resource,
    })
}

/// Refuse to send a token that is not bound to an environment over plain
/// HTTP unless `--env` was typed. `KOBANA_<PRODUCT>_TOKEN` is the same
/// variable whatever the environment, so a production token would otherwise
/// go in the clear — and through any proxy the environment names — as soon
/// as something switched the CLI to a development host.
pub fn refuse_unbound_token_in_the_clear(
    token: &ResolvedToken,
    base_url: &str,
    explicit_env: bool,
) -> Result<(), KobanaError> {
    let in_the_clear = base_url.starts_with("http://");
    if !in_the_clear || explicit_env || token.environment.is_some() {
        return Ok(());
    }
    let source = match &token.source {
        TokenSource::ProductEnv(var) => var.clone(),
        TokenSource::CredentialsFile(path) => format!("KOBANA_CREDENTIALS_FILE ({path})"),
        TokenSource::Saved => return Ok(()),
    };
    Err(KobanaError::Auth(trf!(
        pt: "o token de {source} não é de um ambiente e {base_url} não usa HTTPS; para mandá-lo mesmo assim, passe --env development na linha de comando",
        en: "the token from {source} is not bound to an environment and {base_url} is not HTTPS; to send it anyway, pass --env development on the command line",
        es: "el token de {source} no es de un entorno y {base_url} no usa HTTPS; para enviarlo de todos modos, pasa --env development en la línea de comandos",
    )))
}

/// The product's saved credentials for `env`, refreshed when expired
async fn saved_credentials(product: &Product, env: &Environment) -> Result<StoredCredentials, KobanaError> {
    let slug = product.slug;
    let env_name = env.as_str();
    if !credential_store::has_credentials(slug, env_name) {
        return Err(not_configured_for(product, env));
    }
    let creds = credential_store::load_credentials(slug, env_name)?;
    if !is_expired(&creds) {
        return Ok(creds);
    }

    // Two commands running at once with the expired token would spend the
    // same refresh token; with rotation or reuse detection the second fails
    // and the whole family may be revoked. One refreshes at a time, and the
    // ones that waited read what it saved.
    let _lock = credential_store::lock_refresh(slug, env_name)?;
    let creds = credential_store::load_credentials(slug, env_name)?;
    if !is_expired(&creds) {
        return Ok(creds);
    }

    match (&creds.refresh_token, &creds.client_id) {
        (Some(refresh), Some(client_id)) => {
            let response =
                oauth::refresh_token(refresh, client_id, creds.resource.as_deref(), &issuer_of(&creds, env))
                    .await
                    .map_err(|e| {
                        KobanaError::Auth(trf!(
                            pt: "as credenciais de {slug} expiraram e não foi possível renová-las ({e}). Rode 'kobana {slug} auth login'.",
                            en: "the {slug} credentials expired and could not be refreshed ({e}). Run 'kobana {slug} auth login'.",
                            es: "las credenciales de {slug} expiraron y no se pudieron renovar ({e}). Ejecuta 'kobana {slug} auth login'.",
                        ))
                    })?;
            let refreshed = StoredCredentials {
                access_token: response.access_token,
                // Rotation: keep the old refresh token only when no new one came back
                refresh_token: response.refresh_token.or(creds.refresh_token.clone()),
                token_type: response.token_type.unwrap_or(creds.token_type.clone()),
                expires_at: response
                    .expires_in
                    .map(|secs| chrono::Utc::now().timestamp() + secs),
                product: Some(slug.to_string()),
                ..creds
            };
            credential_store::save_credentials(slug, env_name, &refreshed)?;
            Ok(refreshed)
        }
        _ => Err(KobanaError::Auth(trf!(
            pt: "as credenciais salvas de {slug} expiraram. Rode 'kobana {slug} auth login' para entrar de novo.",
            en: "saved {slug} credentials expired. Run 'kobana {slug} auth login' to re-authenticate.",
            es: "las credenciales guardadas de {slug} expiraron. Ejecuta 'kobana {slug} auth login' para volver a entrar.",
        ))),
    }
}

/// Where a saved token is refreshed: the server that issued it, or for
/// credentials saved before the issuer was kept, the environment's default
/// Conta Kobana. Never `KOBANA_ACCOUNT_URL`, which a `.env` could point
/// anywhere after the login.
fn issuer_of(creds: &StoredCredentials, env: &Environment) -> String {
    creds
        .issuer
        .clone()
        .unwrap_or_else(|| oauth::default_authorization_server(env))
}

pub fn is_expired(creds: &StoredCredentials) -> bool {
    creds
        .expires_at
        .is_some_and(|exp| chrono::Utc::now().timestamp() > exp - EXPIRY_SKEW)
}

/// The JSON file `KOBANA_CREDENTIALS_FILE` points to — the shape
/// `kobana <product> auth export` prints
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CredentialsFile {
    pub access_token: String,
    #[serde(default)]
    pub token_type: Option<String>,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_at: Option<i64>,
    #[serde(default)]
    pub resource: Option<String>,
    /// Product the token was issued for. Required: a token serves one
    /// product, and a file that does not say which is refused.
    pub product: String,
    #[serde(default)]
    pub environment: Option<String>,
}

impl CredentialsFile {
    /// Whether the file holds a token for this product and environment. A
    /// token serves one product (its `aud`), so a banking file must not be
    /// sent to billing. A file without `environment` is taken as production,
    /// the default: it must not follow the CLI to a development host.
    pub fn serves(&self, product: &Product, env: &Environment) -> bool {
        let file_env = self.environment.as_deref().unwrap_or("production");
        self.product == product.slug && file_env == env.as_str()
    }
}

/// `KOBANA_CREDENTIALS_FILE`, when it is set, as its path
pub fn credentials_file_path() -> Option<String> {
    non_empty_env("KOBANA_CREDENTIALS_FILE")
}

/// The `KOBANA_CREDENTIALS_FILE` credentials, when they serve this product
/// and environment. A file that cannot be read is an error, whatever it was
/// meant for.
pub fn credentials_file_for(product: &Product, env: &Environment) -> Result<Option<CredentialsFile>, KobanaError> {
    let Some(path) = credentials_file_path() else {
        return Ok(None);
    };
    let file = read_credentials_file(&path)?;
    Ok(file.serves(product, env).then_some(file))
}

pub fn read_credentials_file(file_path: &str) -> Result<CredentialsFile, KobanaError> {
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        KobanaError::Auth(trf!(
            pt: "não foi possível ler o arquivo de credenciais '{file_path}': {e}",
            en: "failed to read credentials file '{file_path}': {e}",
            es: "no se pudo leer el archivo de credenciales '{file_path}': {e}",
        ))
    })?;
    let creds: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
        KobanaError::Auth(trf!(
            pt: "arquivo de credenciais inválido: {e}",
            en: "invalid credentials file: {e}",
            es: "archivo de credenciales inválido: {e}",
        ))
    })?;
    if !creds["access_token"].is_string() {
        return Err(KobanaError::Auth(
            tr!(
                pt: "o arquivo de credenciais não tem o campo 'access_token'",
                en: "credentials file missing 'access_token' field",
                es: "al archivo de credenciales le falta el campo 'access_token'",
            )
            .into(),
        ));
    }
    if !creds["product"].is_string() {
        return Err(KobanaError::Auth(trf!(
            pt: "o arquivo de credenciais '{file_path}' não tem o campo 'product' (o produto do token, ex.: \"banking\"); gere-o com `kobana <produto> auth export`",
            en: "credentials file '{file_path}' missing 'product' field (the token's product, e.g. \"banking\"); create it with `kobana <product> auth export`",
            es: "al archivo de credenciales '{file_path}' le falta el campo 'product' (el producto del token, ej.: \"banking\"); créalo con `kobana <producto> auth export`",
        )));
    }
    serde_json::from_value(creds).map_err(|e| {
        KobanaError::Auth(trf!(
            pt: "arquivo de credenciais inválido: {e}",
            en: "invalid credentials file: {e}",
            es: "archivo de credenciales inválido: {e}",
        ))
    })
}

/// Told when a product has no token source set up at all. The hint matches
/// what the environment offers: no login where the Conta Kobana has no
/// resource for the product, and the environment's own error where the
/// product does not exist in it at all.
pub fn not_configured_for(product: &Product, env: &Environment) -> KobanaError {
    if let Err(e) = product.base_url(env) {
        return e;
    }
    let slug = product.slug;
    let var = product.token_env();
    if product.oauth_resource(env).is_err() {
        let env = env.as_str();
        return KobanaError::Auth(trf!(
            pt: "Nenhuma autenticação configurada para {slug} em {env}, que não tem login pela Conta Kobana. Defina {var}.",
            en: "No authentication configured for {slug} in {env}, which has no Conta Kobana login. Set {var}.",
            es: "No hay autenticación configurada para {slug} en {env}, que no tiene inicio de sesión con la Conta Kobana. Define {var}.",
        ));
    }
    KobanaError::Auth(trf!(
        pt: "Nenhuma autenticação configurada para {slug}. Defina {var} ou rode 'kobana {slug} auth login'.",
        en: "No authentication configured for {slug}. Set {var} or run 'kobana {slug} auth login'.",
        es: "No hay autenticación configurada para {slug}. Define {var} o ejecuta 'kobana {slug} auth login'.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_is_refreshed_where_it_was_issued() {
        let mut creds = StoredCredentials {
            access_token: "a".into(),
            refresh_token: Some("r".into()),
            token_type: "Bearer".into(),
            expires_at: None,
            environment: "sandbox".into(),
            product: Some("billing".into()),
            client_id: Some("kobana-cli".into()),
            resource: None,
            scopes: Vec::new(),
            issuer: Some("https://account.sandbox.kobana.com.br".into()),
        };
        assert_eq!(issuer_of(&creds, &Environment::Sandbox), "https://account.sandbox.kobana.com.br");
        // Saved before the issuer was kept: the environment's Conta Kobana,
        // whatever KOBANA_ACCOUNT_URL says
        creds.issuer = None;
        assert_eq!(issuer_of(&creds, &Environment::Production), "https://account.kobana.com.br");
    }

    #[test]
    fn the_hint_matches_what_the_environment_offers() {
        let kadu = crate::product::by_slug("kadu").unwrap();
        let billing = crate::product::by_slug("billing").unwrap();

        // Kadu has no sandbox at all: say so, do not send to `auth login`
        let err = not_configured_for(kadu, &Environment::Sandbox);
        assert!(matches!(err, KobanaError::Validation(_)), "{err}");
        assert!(!err.to_string().contains("auth login"), "{err}");

        let err = not_configured_for(billing, &Environment::Production);
        assert!(matches!(err, KobanaError::Auth(_)));
        assert!(err.to_string().contains("kobana billing auth login"), "{err}");

        // A host without a Conta Kobana resource: the token variable only
        for product in crate::product::REGISTRY {
            for env in [Environment::Production, Environment::Sandbox, Environment::Development] {
                if product.base_url(&env).is_ok() && product.oauth_resource(&env).is_err() {
                    let err = not_configured_for(product, &env);
                    assert!(!err.to_string().contains("auth login"), "{} {}: {err}", product.slug, env.as_str());
                }
            }
        }
    }

    fn file(json: serde_json::Value) -> CredentialsFile {
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn a_credentials_file_serves_only_its_product_and_environment() {
        let banking = crate::product::by_slug("banking").unwrap();
        let billing = crate::product::by_slug("billing").unwrap();
        let prod = Environment::Production;
        let sandbox = Environment::Sandbox;

        let f = file(serde_json::json!({"access_token": "t", "product": "billing", "environment": "sandbox"}));
        assert!(f.serves(billing, &sandbox));
        assert!(!f.serves(billing, &prod));
        assert!(!f.serves(banking, &sandbox));

        // No environment: production only, never a development host
        let f = file(serde_json::json!({"access_token": "t", "product": "billing"}));
        assert!(f.serves(billing, &prod));
        assert!(!f.serves(billing, &sandbox));
        assert!(!f.serves(billing, &Environment::Development));
    }

    #[test]
    fn an_unbound_token_goes_in_the_clear_only_with_an_explicit_env() {
        let token = |source, environment| ResolvedToken {
            access_token: "t".into(),
            source,
            token_type: "Bearer".into(),
            refresh_token: None,
            expires_at: None,
            environment,
            resource: None,
        };
        let env_token = token(TokenSource::ProductEnv("KOBANA_BANKING_TOKEN".into()), None);
        assert!(refuse_unbound_token_in_the_clear(&env_token, "https://api.kobana.com.br", false).is_ok());
        assert!(matches!(
            refuse_unbound_token_in_the_clear(&env_token, "http://localhost:5005/api", false),
            Err(KobanaError::Auth(_))
        ));
        assert!(refuse_unbound_token_in_the_clear(&env_token, "http://localhost:5005/api", true).is_ok());
        let file_token = token(TokenSource::CredentialsFile("/tmp/c.json".into()), None);
        assert!(refuse_unbound_token_in_the_clear(&file_token, "http://localhost:5005/api", false).is_err());
        // Bound to an environment or saved by a login: the environment was chosen on purpose
        let bound = token(TokenSource::CredentialsFile("/tmp/c.json".into()), Some("development".into()));
        assert!(refuse_unbound_token_in_the_clear(&bound, "http://localhost:5005/api", false).is_ok());
        let saved = token(TokenSource::Saved, Some("development".into()));
        assert!(refuse_unbound_token_in_the_clear(&saved, "http://localhost:5005/api", false).is_ok());
    }

    #[test]
    fn a_credentials_file_without_product_is_refused() {
        let path = std::env::temp_dir().join(format!("kobana-creds-{}.json", std::process::id()));
        std::fs::write(&path, r#"{"access_token": "t"}"#).unwrap();
        let err = read_credentials_file(&path.display().to_string()).unwrap_err();
        assert!(matches!(err, KobanaError::Auth(ref m) if m.contains("product")), "{err}");

        std::fs::write(&path, r#"{"access_token": "t", "product": "banking"}"#).unwrap();
        assert_eq!(read_credentials_file(&path.display().to_string()).unwrap().product, "banking");
        std::fs::remove_file(path).ok();
    }
}
