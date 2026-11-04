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


/// The config directory, fixed by [`load_dotenv`] before the project's
/// `.env` is read: whatever `dirs` consults to find it (`XDG_CONFIG_HOME` on
/// Linux) must not be able to move the credential store into a checkout.
static CONFIG_DIR: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

/// Resolve the config directory
pub fn config_dir() -> PathBuf {
    CONFIG_DIR.get_or_init(resolve_config_dir).clone()
}

fn resolve_config_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("KOBANA_CONFIG_DIR") {
        PathBuf::from(dir)
    } else {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("kobana")
    }
}

/// Variables a project's `.env` may not set: where the CLI logs in (and
/// refreshes tokens), where it keeps credentials, the credentials themselves,
/// which environment it talks to and through which proxy. A repository
/// someone else wrote could otherwise send the login to its own server, have
/// credentials saved inside it, make the CLI act with its own token or OAuth
/// client, or switch to the plain-HTTP development hosts and route the
/// request — bearer token included — through a proxy it names. Product
/// tokens are matched by [`is_credential_var`]; proxy variables are matched
/// in any case (`http_proxy`, `HTTP_PROXY`), as reqwest reads them.
const PROJECT_DOTENV_BLOCKED: &[&str] = &[
    "KOBANA_ACCOUNT_URL",
    "KOBANA_CONFIG_DIR",
    "KOBANA_CREDENTIALS_FILE",
    "KOBANA_CLIENT_ID",
    "KOBANA_CLIENT_SECRET",
    "KOBANA_ENVIRONMENT",
    // Where the config directory (and so the credential store) is looked up
    "HOME",
    "USERPROFILE",
    "APPDATA",
    "LOCALAPPDATA",
];

const PROXY_VARS: &[&str] = &["HTTP_PROXY", "HTTPS_PROXY", "ALL_PROXY", "NO_PROXY"];

/// Matched without regard to case: Windows looks environment variables up
/// case-insensitively, so `kobana_account_url=` in a `.env` would otherwise
/// be read back as `KOBANA_ACCOUNT_URL` there.
fn is_blocked_var(key: &str) -> bool {
    let key = key.to_ascii_uppercase();
    PROJECT_DOTENV_BLOCKED.contains(&key.as_str())
        || key.starts_with("XDG_")
        || is_credential_var(&key)
        || PROXY_VARS.contains(&key.as_str())
}

/// A product's token variable: `KOBANA_<PRODUCT>_TOKEN` or one of the
/// registered aliases (`KADU_TOKEN`). `key` is upper case.
fn is_credential_var(key: &str) -> bool {
    (key.starts_with("KOBANA_") && key.ends_with("_TOKEN"))
        || crate::product::REGISTRY
            .iter()
            .any(|p| p.token_env_aliases.contains(&key))
}

/// What the project's `.env` did, to say once the language is known
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ProjectDotenv {
    /// Variables it tried to set but may not
    pub ignored: Vec<String>,
    /// Variables it set: the user did not type them, so they are named
    pub applied: Vec<String>,
}

/// Load .env files: the config dir's, then the current directory's (never its
/// parents'). Variables already set win over both, so the user's own file
/// wins over the checked-out one.
pub fn load_dotenv() -> ProjectDotenv {
    // Also fixes `CONFIG_DIR` before the project's `.env` can touch the
    // environment it is resolved from
    let _ = dotenvy::from_path(config_dir().join(".env"));
    apply_project_dotenv(std::path::Path::new(".env"))
}

fn apply_project_dotenv(path: &std::path::Path) -> ProjectDotenv {
    let Ok(entries) = dotenvy::from_path_iter(path) else {
        return ProjectDotenv::default();
    };
    let mut report = ProjectDotenv::default();
    for (key, value) in entries.flatten() {
        if is_blocked_var(&key) {
            report.ignored.push(key);
        } else if std::env::var_os(&key).is_none() {
            std::env::set_var(&key, value);
            report.applied.push(key);
        }
    }
    report
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
                 KOBANA_CLIENT_SECRET=s\nKOBANA_ENVIRONMENT=development\n\
                 http_proxy=http://evil.example:8080\nALL_PROXY=http://evil.example:8080\n\
                 NO_PROXY=\nkobana_account_url=https://evil.example\nkobana_banking_token=t\n\
                 kadu_token=t\nKobana_Environment=development\nXDG_CONFIG_HOME=/tmp/evil\n\
                 HOME=/tmp/evil\n{var}=ok\n"
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
            "KOBANA_ENVIRONMENT",
            "http_proxy",
            "ALL_PROXY",
            "NO_PROXY",
            // Any case: Windows reads `kobana_account_url` as `KOBANA_ACCOUNT_URL`
            "kobana_account_url",
            "kobana_banking_token",
            "kadu_token",
            "Kobana_Environment",
            "XDG_CONFIG_HOME",
            "HOME",
        ];
        let before: Vec<_> = blocked.iter().map(|k| std::env::var(k).ok()).collect();

        let report = apply_project_dotenv(&path);
        assert_eq!(report.ignored, blocked);
        assert_eq!(report.applied, std::slice::from_ref(&var));
        assert_eq!(std::env::var(&var).as_deref(), Ok("ok"));
        let after: Vec<_> = blocked.iter().map(|k| std::env::var(k).ok()).collect();
        assert_eq!(after, before);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn the_config_dir_is_fixed_once_and_stays_put() {
        let first = config_dir();
        // Whatever the environment says afterwards, the store does not move
        std::env::set_var("XDG_CONFIG_HOME", "/tmp/kobana-moved");
        std::env::set_var("KOBANA_CONFIG_DIR", "/tmp/kobana-moved");
        assert_eq!(config_dir(), first);
        std::env::remove_var("XDG_CONFIG_HOME");
        std::env::remove_var("KOBANA_CONFIG_DIR");
    }

    #[test]
    fn a_variable_already_set_is_not_applied_nor_reported() {
        let dir = std::env::temp_dir().join(format!("kobana-dotenv-set-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let var = format!("KOBANA_DOTENV_SET_TEST_{}", std::process::id());
        std::env::set_var(&var, "mine");
        let path = dir.join(".env");
        std::fs::write(&path, format!("{var}=theirs\n")).unwrap();
        let report = apply_project_dotenv(&path);
        assert_eq!(report, ProjectDotenv::default());
        assert_eq!(std::env::var(&var).as_deref(), Ok("mine"));
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
