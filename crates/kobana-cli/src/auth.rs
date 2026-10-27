use kobana::error::KobanaError;
use kobana::{tr, trf};

use crate::credential_store;

/// Resolve a Bearer token from the available sources
///
/// Priority:
/// 1. KOBANA_TOKEN env var
/// 2. KOBANA_CREDENTIALS_FILE env var
/// 3. KOBANA_CLIENT_ID + KOBANA_CLIENT_SECRET (deferred to login)
/// 4. Saved credentials (~/.config/kobana/credentials.enc)
pub fn resolve_token() -> Result<String, KobanaError> {
    // Priority 1: KOBANA_TOKEN env var
    if let Ok(token) = std::env::var("KOBANA_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    // Priority 2: KOBANA_CREDENTIALS_FILE
    if let Ok(file_path) = std::env::var("KOBANA_CREDENTIALS_FILE") {
        if !file_path.is_empty() {
            let content = std::fs::read_to_string(&file_path).map_err(|e| {
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
            if let Some(token) = creds["access_token"].as_str() {
                return Ok(token.to_string());
            }
            return Err(KobanaError::Auth(
                tr!(
                    pt: "o arquivo de credenciais não tem o campo 'access_token'",
                    en: "credentials file missing 'access_token' field",
                    es: "al archivo de credenciales le falta el campo 'access_token'",
                )
                .into(),
            ));
        }
    }

    // Priority 4: Saved credentials
    if !credential_store::credentials_path().exists() {
        return Err(KobanaError::Auth(not_configured_message().into()));
    }
    let creds = credential_store::load_credentials()?;
    if let Some(expires_at) = creds.expires_at {
        if chrono::Utc::now().timestamp() > expires_at {
            return Err(KobanaError::Auth(
                tr!(
                    pt: "as credenciais salvas expiraram. Rode 'kobana auth login' para entrar de novo.",
                    en: "saved credentials expired. Run 'kobana auth login' to re-authenticate.",
                    es: "las credenciales guardadas expiraron. Ejecuta 'kobana auth login' para volver a entrar.",
                )
                .into(),
            ));
        }
    }
    Ok(creds.access_token)
}

/// Told when no token source is set up at all
pub fn not_configured_message() -> &'static str {
    tr!(
        pt: "Nenhuma autenticação configurada. Defina KOBANA_TOKEN ou rode 'kobana auth login'.",
        en: "No authentication configured. Set KOBANA_TOKEN or run 'kobana auth login'.",
        es: "No hay autenticación configurada. Define KOBANA_TOKEN o ejecuta 'kobana auth login'.",
    )
}
