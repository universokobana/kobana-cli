use kobana::error::KobanaError;

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
                KobanaError::Auth(format!("failed to read credentials file '{file_path}': {e}"))
            })?;
            let creds: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
                KobanaError::Auth(format!("invalid credentials file: {e}"))
            })?;
            if let Some(token) = creds["access_token"].as_str() {
                return Ok(token.to_string());
            }
            return Err(KobanaError::Auth(
                "credentials file missing 'access_token' field".into(),
            ));
        }
    }

    // Priority 4: Saved credentials
    match credential_store::load_credentials() {
        Ok(creds) => {
            // Check expiration
            if let Some(expires_at) = creds.expires_at {
                if chrono::Utc::now().timestamp() > expires_at {
                    return Err(KobanaError::Auth(
                        "saved credentials expired. Run 'kobana auth login' to re-authenticate.".into(),
                    ));
                }
            }
            Ok(creds.access_token)
        }
        Err(KobanaError::Auth(msg)) if msg == "No saved credentials found" => {
            Err(KobanaError::Auth(
                "No authentication configured. Set KOBANA_TOKEN or run 'kobana auth login'.".into(),
            ))
        }
        Err(e) => Err(e),
    }
}
