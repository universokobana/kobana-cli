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

/// Variables a project's `.env` may not set: where the CLI logs in (and
/// refreshes tokens), where it keeps credentials, and the credentials
/// themselves. A repository someone else wrote could otherwise send the login
/// to its own server, have credentials saved inside it, or make the CLI act
/// with its own token or OAuth client. Product tokens are matched by
/// [`is_credential_var`].
const PROJECT_DOTENV_BLOCKED: &[&str] = &[
    "KOBANA_ACCOUNT_URL",
    "KOBANA_CONFIG_DIR",
    "KOBANA_CREDENTIALS_FILE",
    "KOBANA_CLIENT_ID",
    "KOBANA_CLIENT_SECRET",
];

/// A product's token variable: `KOBANA_<PRODUCT>_TOKEN` or one of the
/// registered aliases (`KADU_TOKEN`)
fn is_credential_var(key: &str) -> bool {
    (key.starts_with("KOBANA_") && key.ends_with("_TOKEN"))
        || crate::product::REGISTRY
            .iter()
            .any(|p| p.token_env_aliases.contains(&key))
}

/// Load .env files: the current directory's (never its parents') and the
/// config dir's. Variables already set win over both. Returns the variables
/// the project's `.env` set but were ignored, to warn about once the
/// language is known.
pub fn load_dotenv() -> Vec<String> {
    let ignored = apply_project_dotenv(std::path::Path::new(".env"));
    let _ = dotenvy::from_path(config_dir().join(".env"));
    ignored
}

fn apply_project_dotenv(path: &std::path::Path) -> Vec<String> {
    let Ok(entries) = dotenvy::from_path_iter(path) else {
        return Vec::new();
    };
    let mut ignored = Vec::new();
    for (key, value) in entries.flatten() {
        if PROJECT_DOTENV_BLOCKED.contains(&key.as_str()) || is_credential_var(&key) {
            ignored.push(key);
        } else if std::env::var_os(&key).is_none() {
            std::env::set_var(key, value);
        }
    }
    ignored
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
    fn a_project_dotenv_cannot_move_the_login_or_the_credentials() {
        let dir = std::env::temp_dir().join(format!("kobana-dotenv-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let var = format!("KOBANA_DOTENV_TEST_{}", std::process::id());
        let path = dir.join(".env");
        std::fs::write(
            &path,
            format!(
                "KOBANA_ACCOUNT_URL=https://evil.example\nKOBANA_CONFIG_DIR=/tmp/evil\n\
                 KOBANA_BANKING_TOKEN=t\nKADU_TOKEN=t\nKOBANA_CREDENTIALS_FILE=/tmp/c.json\n\
                 KOBANA_CLIENT_SECRET=s\n{var}=ok\n"
            ),
        )
        .unwrap();
        let blocked = [
            "KOBANA_ACCOUNT_URL",
            "KOBANA_CONFIG_DIR",
            "KOBANA_BANKING_TOKEN",
            "KADU_TOKEN",
            "KOBANA_CREDENTIALS_FILE",
            "KOBANA_CLIENT_SECRET",
        ];
        let before: Vec<_> = blocked.iter().map(|k| std::env::var(k).ok()).collect();

        let ignored = apply_project_dotenv(&path);
        assert_eq!(ignored, blocked);
        assert_eq!(std::env::var(&var).as_deref(), Ok("ok"));
        let after: Vec<_> = blocked.iter().map(|k| std::env::var(k).ok()).collect();
        assert_eq!(after, before);
        std::fs::remove_dir_all(dir).ok();
    }

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
