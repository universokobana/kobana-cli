use kobana::error::KobanaError;

/// Resolve a Bearer token from the available sources (Phase 1: env var only)
///
/// Priority:
/// 1. KOBANA_TOKEN env var
/// 2. (Phase 2: credentials file, client credentials, saved credentials)
pub fn resolve_token() -> Result<String, KobanaError> {
    // Priority 1: KOBANA_TOKEN env var
    if let Ok(token) = std::env::var("KOBANA_TOKEN") {
        if !token.is_empty() {
            return Ok(token);
        }
    }

    Err(KobanaError::Auth(
        "No authentication configured. Set KOBANA_TOKEN or run 'kobana auth login'.".into(),
    ))
}
