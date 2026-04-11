use std::path::PathBuf;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Environment {
    #[default]
    Production,
    Sandbox,
    Development,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::Sandbox => "https://api-sandbox.kobana.com.br",
            Self::Production => "https://api.kobana.com.br",
            Self::Development => "http://localhost:5005/api",
        }
    }

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
/// Defaults to Production when neither is set.
pub fn resolve_environment(env_flag: Option<&str>) -> Environment {
    let value = env_flag
        .map(|s| s.to_string())
        .or_else(|| std::env::var("KOBANA_ENVIRONMENT").ok());

    match value.as_deref() {
        Some("sandbox") => Environment::Sandbox,
        Some("development") => Environment::Development,
        Some("production") | None => Environment::Production,
        Some(_) => Environment::Production,
    }
}
