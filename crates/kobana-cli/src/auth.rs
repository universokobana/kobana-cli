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

/// Resolve the Bearer token for a product.
///
/// Priority:
/// 1. `KOBANA_<PRODUCT>_TOKEN` (and the product's aliases, e.g. `KADU_TOKEN`)
/// 2. `KOBANA_CREDENTIALS_FILE` — a JSON file with `access_token`, when it
///    is this product's and environment's (see [`CredentialsFile::serves`])
/// 3. Saved credentials from `kobana <product> auth login`, refreshed with
///    the refresh token when expired
pub async fn resolve_token(product: &Product, env: &Environment) -> Result<String, KobanaError> {
    if let Some((token, _)) = env_token(product) {
        return Ok(token);
    }

    if let Some(file) = credentials_file_for(product, env)? {
        return Ok(file.access_token);
    }

    let slug = product.slug;
    let env_name = env.as_str();
    if !credential_store::has_credentials(slug, env_name) {
        return Err(KobanaError::Auth(not_configured_for(product)));
    }
    let creds = credential_store::load_credentials(slug, env_name)?;
    if !is_expired(&creds) {
        return Ok(creds.access_token);
    }

    match (&creds.refresh_token, &creds.client_id) {
        (Some(refresh), Some(client_id)) => {
            let response =
                oauth::refresh_token(refresh, client_id, creds.resource.as_deref(), env)
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
            Ok(refreshed.access_token)
        }
        _ => Err(KobanaError::Auth(trf!(
            pt: "as credenciais salvas de {slug} expiraram. Rode 'kobana {slug} auth login' para entrar de novo.",
            en: "saved {slug} credentials expired. Run 'kobana {slug} auth login' to re-authenticate.",
            es: "las credenciales guardadas de {slug} expiraron. Ejecuta 'kobana {slug} auth login' para volver a entrar.",
        ))),
    }
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
    /// Product the token was issued for. Absent in files from before tokens
    /// were per product, which were always banking's.
    #[serde(default)]
    pub product: Option<String>,
    #[serde(default)]
    pub environment: Option<String>,
}

impl CredentialsFile {
    /// Whether the file holds a token for this product and environment. A
    /// token serves one product (its `aud`), so a banking file must not be
    /// sent to billing. A file without `environment` serves any.
    pub fn serves(&self, product: &Product, env: &Environment) -> bool {
        self.product.as_deref().unwrap_or("banking") == product.slug
            && self.environment.as_deref().is_none_or(|e| e == env.as_str())
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
    serde_json::from_value(creds).map_err(|e| {
        KobanaError::Auth(trf!(
            pt: "arquivo de credenciais inválido: {e}",
            en: "invalid credentials file: {e}",
            es: "archivo de credenciales inválido: {e}",
        ))
    })
}

/// Told when a product has no token source set up at all
pub fn not_configured_for(product: &Product) -> String {
    let slug = product.slug;
    let var = product.token_env();
    trf!(
        pt: "Nenhuma autenticação configurada para {slug}. Defina {var} ou rode 'kobana {slug} auth login'.",
        en: "No authentication configured for {slug}. Set {var} or run 'kobana {slug} auth login'.",
        es: "No hay autenticación configurada para {slug}. Define {var} o ejecuta 'kobana {slug} auth login'.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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

        // No environment: any. No product: banking, as files were before 0.10
        let f = file(serde_json::json!({"access_token": "t", "product": "billing"}));
        assert!(f.serves(billing, &prod) && f.serves(billing, &sandbox));
        let f = file(serde_json::json!({"access_token": "t"}));
        assert!(f.serves(banking, &prod));
        assert!(!f.serves(billing, &prod));
    }
}
