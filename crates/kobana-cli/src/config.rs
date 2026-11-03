use kobana::error::KobanaError;
use kobana::trf;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Environment {
    #[default]
    Production,
    Sandbox,
    Development,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sandbox => "sandbox",
            Self::Production => "production",
            Self::Development => "development",
        }
    }
}


/// Resolve the config directory
pub fn config_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("KOBANA_CONFIG_DIR") {
        PathBuf::from(dir)
    } else {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("kobana")
    }
}

/// Load .env files (project-local and config dir)
pub fn load_dotenv() {
    // Load project-local .env first
    let _ = dotenvy::dotenv();
    // Then config dir .env
    let config = config_dir();
    let _ = dotenvy::from_path(config.join(".env"));
}

/// Resolve the environment from the `--env` flag or KOBANA_ENVIRONMENT.
/// Defaults to Production when neither is set. An unknown value is an error:
/// a typo (`sandbx`) must not send tests and logins to production.
pub fn resolve_environment(env_flag: Option<&str>) -> Result<Environment, KobanaError> {
    let from_var = std::env::var("KOBANA_ENVIRONMENT").ok();
    parse_environment(env_flag, from_var.as_deref())
}

fn parse_environment(env_flag: Option<&str>, env_var: Option<&str>) -> Result<Environment, KobanaError> {
    match env_flag.or(env_var) {
        Some("sandbox") => Ok(Environment::Sandbox),
        Some("development") => Ok(Environment::Development),
        Some("production") | None => Ok(Environment::Production),
        Some(other) => Err(KobanaError::Validation(trf!(
            pt: "KOBANA_ENVIRONMENT inválido: '{other}' (use production, sandbox ou development)",
            en: "invalid KOBANA_ENVIRONMENT: '{other}' (use production, sandbox or development)",
            es: "KOBANA_ENVIRONMENT inválido: '{other}' (use production, sandbox o development)",
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_environments_are_refused() {
        assert_eq!(parse_environment(None, None).unwrap(), Environment::Production);
        assert_eq!(parse_environment(None, Some("sandbox")).unwrap(), Environment::Sandbox);
        assert_eq!(parse_environment(Some("development"), Some("sandbox")).unwrap(), Environment::Development);
        assert!(matches!(parse_environment(None, Some("sandbx")), Err(KobanaError::Validation(_))));
        // The flag wins, so a bad variable does not matter when it is given
        assert_eq!(parse_environment(Some("sandbox"), Some("sandbx")).unwrap(), Environment::Sandbox);
    }
}
