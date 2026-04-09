use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Production,
}

impl Environment {
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::Sandbox => "https://api-sandbox.kobana.com.br",
            Self::Production => "https://api.kobana.com.br",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::Sandbox
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

/// Resolve the environment from flags or env var
pub fn resolve_environment(sandbox: bool, production: bool) -> Environment {
    if production {
        return Environment::Production;
    }
    if sandbox {
        return Environment::Sandbox;
    }
    match std::env::var("KOBANA_ENVIRONMENT").as_deref() {
        Ok("production") => Environment::Production,
        _ => Environment::Sandbox,
    }
}
