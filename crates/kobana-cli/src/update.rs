use kobana::error::KobanaError;
use serde::Deserialize;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config;

const GITHUB_API: &str =
    "https://api.github.com/repos/universokobana/kobana-cli/releases/latest";
const CHECK_INTERVAL_SECS: u64 = 86_400; // 24 hours
const AUTO_CHECK_TIMEOUT_SECS: u64 = 3;
const USER_AGENT: &str = concat!("kobana-cli/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
}

#[derive(Debug, PartialEq)]
pub enum InstallMethod {
    Homebrew,
    Cargo,
    Standalone,
    Unknown,
}

impl InstallMethod {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Homebrew => "homebrew",
            Self::Cargo => "cargo",
            Self::Standalone => "standalone",
            Self::Unknown => "unknown",
        }
    }
}

/// Detect how the binary was installed, based on its path.
pub fn detect_install_method() -> InstallMethod {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => return InstallMethod::Unknown,
    };
    let path = exe.to_string_lossy().to_lowercase();

    if path.contains("cellar/kobana")
        || path.contains("/opt/homebrew/")
        || path.contains("linuxbrew")
    {
        InstallMethod::Homebrew
    } else if path.contains(".cargo/bin")
        || path.contains("/target/debug/")
        || path.contains("/target/release/")
    {
        InstallMethod::Cargo
    } else {
        InstallMethod::Standalone
    }
}

fn install_command(method: &InstallMethod) -> &'static str {
    match method {
        InstallMethod::Homebrew => "brew upgrade kobana",
        InstallMethod::Cargo => {
            "cargo install --git https://github.com/universokobana/kobana-cli kobana-cli --force"
        }
        InstallMethod::Standalone => "kobana update",
        InstallMethod::Unknown => {
            "Download from https://github.com/universokobana/kobana-cli/releases/latest"
        }
    }
}

/// Build a reqwest client with sensible defaults for GitHub API calls.
fn http_client(timeout: Duration) -> Result<reqwest::Client, KobanaError> {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(timeout)
        .build()
        .map_err(|e| KobanaError::Internal(format!("http client error: {e}")))
}

/// Fetch the latest release (tag without leading `v`, html_url) from GitHub.
pub async fn fetch_latest_release(timeout: Duration) -> Result<(String, String), KobanaError> {
    let client = http_client(timeout)?;

    let response = client
        .get(GITHUB_API)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| KobanaError::Internal(format!("failed to fetch latest release: {e}")))?;

    let status = response.status();
    if !status.is_success() {
        return Err(KobanaError::Internal(format!(
            "GitHub API returned {status}"
        )));
    }

    let release: GitHubRelease = response
        .json()
        .await
        .map_err(|e| KobanaError::Internal(format!("invalid release response: {e}")))?;

    let tag = release
        .tag_name
        .strip_prefix('v')
        .unwrap_or(&release.tag_name)
        .to_string();
    Ok((tag, release.html_url))
}

/// Compare two dotted-numeric versions (e.g. "0.3.0" vs "0.3.1").
pub fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    fn parts(v: &str) -> Vec<u32> {
        v.split('.').filter_map(|p| p.parse().ok()).collect()
    }
    parts(a).cmp(&parts(b))
}

/// Handle the `kobana update` command.
pub async fn handle_update(check_only: bool, as_json: bool) -> Result<(), KobanaError> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let (latest, release_url) = fetch_latest_release(Duration::from_secs(15)).await?;
    let update_available = compare_versions(&latest, &current) == std::cmp::Ordering::Greater;
    let method = detect_install_method();
    let command = install_command(&method);

    // Update the cached timestamp so auto-check doesn't spam the user.
    update_last_check(&latest);

    if as_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "current": current,
                "latest": latest,
                "update_available": update_available,
                "release_url": release_url,
                "install_method": method.as_str(),
                "install_command": command,
            }))
            .unwrap_or_default()
        );
        return Ok(());
    }

    if !update_available {
        println!("kobana {current} is already up to date.");
        return Ok(());
    }

    println!("Update available: kobana {current} → {latest}");

    if check_only {
        println!("Run: {command}");
        return Ok(());
    }

    // Ask for confirmation before running anything destructive.
    let prompt = match method {
        InstallMethod::Homebrew | InstallMethod::Cargo => {
            format!("Run `{command}` now?")
        }
        InstallMethod::Standalone => "Download and install the new binary now?".to_string(),
        InstallMethod::Unknown => {
            println!("Install method not recognized. Run: {command}");
            return Ok(());
        }
    };

    // Only prompt when stdin is a terminal; otherwise fall back to printing
    // the command and exiting, so scripts don't hang.
    if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        println!("Run: {command}");
        return Ok(());
    }

    let confirm = inquire::Confirm::new(&prompt)
        .with_default(true)
        .prompt()
        .map_err(|e| KobanaError::Internal(format!("prompt error: {e}")))?;

    if !confirm {
        println!("Update skipped.");
        return Ok(());
    }

    match method {
        InstallMethod::Homebrew => run_shell_command("brew", &["upgrade", "kobana"]),
        InstallMethod::Cargo => run_shell_command(
            "cargo",
            &[
                "install",
                "--git",
                "https://github.com/universokobana/kobana-cli",
                "kobana-cli",
                "--force",
            ],
        ),
        InstallMethod::Standalone => perform_self_update(&latest).await,
        InstallMethod::Unknown => Ok(()),
    }
}

/// Run an external command inline, inheriting stdin/stdout/stderr so the user
/// sees progress from tools like brew and cargo.
fn run_shell_command(program: &str, args: &[&str]) -> Result<(), KobanaError> {
    let status = std::process::Command::new(program)
        .args(args)
        .status()
        .map_err(|e| KobanaError::Internal(format!("failed to spawn {program}: {e}")))?;

    if !status.success() {
        return Err(KobanaError::Internal(format!(
            "{program} {} exited with status {status}",
            args.join(" ")
        )));
    }
    Ok(())
}

/// Download the release asset for the current platform and replace the
/// running executable atomically. Unix only (macOS + Linux).
#[cfg(unix)]
async fn perform_self_update(version: &str) -> Result<(), KobanaError> {
    use std::os::unix::fs::PermissionsExt;

    let target = current_target()?;
    let asset = format!("kobana-{target}");
    let download_url = format!(
        "https://github.com/universokobana/kobana-cli/releases/download/v{version}/{asset}"
    );

    eprintln!("Downloading {download_url}...");

    let client = http_client(Duration::from_secs(120))?;
    let bytes = client
        .get(&download_url)
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| KobanaError::Internal(format!("download failed: {e}")))?
        .bytes()
        .await
        .map_err(|e| KobanaError::Internal(format!("read download body: {e}")))?;

    let current_exe = std::env::current_exe()
        .map_err(|e| KobanaError::Internal(format!("current exe: {e}")))?;

    // Write to a temp file next to the current exe (same filesystem for atomic rename).
    let temp_path = current_exe.with_file_name(format!(
        ".kobana.new.{}",
        std::process::id()
    ));
    std::fs::write(&temp_path, &bytes)
        .map_err(|e| KobanaError::Internal(format!("write temp binary: {e}")))?;

    let perms = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(&temp_path, perms)
        .map_err(|e| KobanaError::Internal(format!("chmod temp binary: {e}")))?;

    std::fs::rename(&temp_path, &current_exe).map_err(|e| {
        let _ = std::fs::remove_file(&temp_path);
        KobanaError::Internal(format!("atomic rename: {e}"))
    })?;

    eprintln!("✓ Updated to v{version}");
    Ok(())
}

#[cfg(not(unix))]
async fn perform_self_update(_version: &str) -> Result<(), KobanaError> {
    Err(KobanaError::Internal(
        "self-update is not supported on this platform — download the new binary manually from the releases page"
            .into(),
    ))
}

/// Map (target_os, target_arch) to the release asset suffix.
#[allow(clippy::needless_return, unreachable_code)]
fn current_target() -> Result<&'static str, KobanaError> {
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return Ok("darwin-amd64");
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return Ok("darwin-arm64");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return Ok("linux-amd64");
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return Ok("linux-arm64");
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    return Ok("windows-amd64.exe");
    Err(KobanaError::Internal(
        "no release binary available for this platform".into(),
    ))
}

// --- Auto-check cache ---

fn cache_path() -> PathBuf {
    config::config_dir().join(".update_check")
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

struct CachedCheck {
    timestamp: u64,
    latest: String,
}

fn read_cache() -> Option<CachedCheck> {
    let content = std::fs::read_to_string(cache_path()).ok()?;
    let mut timestamp = 0u64;
    let mut latest = String::new();
    for line in content.lines() {
        if let Some((k, v)) = line.split_once('=') {
            match k {
                "timestamp" => timestamp = v.parse().unwrap_or(0),
                "latest" => latest = v.to_string(),
                _ => {}
            }
        }
    }
    if latest.is_empty() {
        None
    } else {
        Some(CachedCheck { timestamp, latest })
    }
}

fn update_last_check(latest: &str) {
    let path = cache_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let content = format!("timestamp={}\nlatest={}\n", now_secs(), latest);
    let _ = std::fs::write(&path, content);
}

/// Silently check for a new version at most once per day. Prints a short
/// notice to stderr if an update is available. Never fails — any network
/// error is swallowed so normal commands are not affected.
pub async fn auto_check() {
    let now = now_secs();

    // If we have a recent cached check, reuse it.
    if let Some(cache) = read_cache() {
        if now.saturating_sub(cache.timestamp) < CHECK_INTERVAL_SECS {
            notify_if_outdated(&cache.latest);
            return;
        }
    }

    // Otherwise do a short network check with a tight timeout.
    let fetch = fetch_latest_release(Duration::from_secs(AUTO_CHECK_TIMEOUT_SECS));
    let result = tokio::time::timeout(
        Duration::from_secs(AUTO_CHECK_TIMEOUT_SECS + 1),
        fetch,
    )
    .await;

    if let Ok(Ok((latest, _url))) = result {
        update_last_check(&latest);
        notify_if_outdated(&latest);
    }
}

fn notify_if_outdated(latest: &str) {
    let current = env!("CARGO_PKG_VERSION");
    if compare_versions(latest, current) != std::cmp::Ordering::Greater {
        return;
    }
    let msg = format!(
        "⚠ New version available: {current} → {latest}. Run `kobana update` for installation instructions."
    );
    if std::io::IsTerminal::is_terminal(&std::io::stderr()) {
        eprintln!("\x1b[33m{msg}\x1b[0m");
    } else {
        eprintln!("{msg}");
    }
}
