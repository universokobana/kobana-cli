use kobana::error::KobanaError;
use kobana::{tr, trf};
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
    #[serde(default)]
    assets: Vec<GitHubAsset>,
}

/// A file of a release. GitHub publishes each one's SHA-256 as
/// `digest: "sha256:<hex>"`, which `install.sh` checks too.
#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    #[serde(default)]
    digest: Option<String>,
}

/// The published SHA-256 (hex) of `asset` in a release, when GitHub lists one
#[cfg_attr(not(unix), allow(dead_code))]
fn asset_sha256(release: &GitHubRelease, asset: &str) -> Option<String> {
    release
        .assets
        .iter()
        .find(|a| a.name == asset)
        .and_then(|a| a.digest.as_deref())
        .and_then(|d| d.strip_prefix("sha256:"))
        .map(str::to_ascii_lowercase)
}

#[cfg_attr(not(unix), allow(dead_code))]
fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// The release of a tag (`v0.10.3`), with its assets
#[cfg_attr(not(unix), allow(dead_code))]
async fn fetch_release(tag: &str) -> Result<GitHubRelease, KobanaError> {
    let url = format!("https://api.github.com/repos/universokobana/kobana-cli/releases/tags/{tag}");
    http_client(Duration::from_secs(15))?
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível consultar a versão {tag}: {e}",
            en: "failed to fetch release {tag}: {e}",
            es: "no se pudo consultar la versión {tag}: {e}",
        )))?
        .json()
        .await
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "resposta de versão inválida: {e}",
            en: "invalid release response: {e}",
            es: "respuesta de versión inválida: {e}",
        )))
}

#[derive(Debug, PartialEq)]
pub enum InstallMethod {
    Homebrew,
    Winget,
    Package,
    Cargo,
    Standalone,
    Unknown,
}

impl InstallMethod {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Homebrew => "homebrew",
            Self::Winget => "winget",
            Self::Package => "package",
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
    classify_install_path(&exe.to_string_lossy().to_lowercase())
}

/// `path` must be lowercase.
fn classify_install_path(path: &str) -> InstallMethod {
    if path.contains("cellar/kobana")
        || path.contains("/opt/homebrew/")
        || path.contains("linuxbrew")
    {
        InstallMethod::Homebrew
    } else if path.contains("\\microsoft\\winget\\") || path.contains("/microsoft/winget/") {
        InstallMethod::Winget
    } else if path == "/usr/bin/kobana" {
        // The .deb / .rpm installs here; the package manager owns the file.
        InstallMethod::Package
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
        InstallMethod::Winget => "winget upgrade Kobana.CLI",
        InstallMethod::Package => {
            "sudo apt update && sudo apt install --only-upgrade kobana  # or: sudo dnf upgrade kobana"
        }
        InstallMethod::Cargo => {
            "cargo install --git https://github.com/universokobana/kobana-cli kobana-cli --force"
        }
        InstallMethod::Standalone => "kobana update",
        InstallMethod::Unknown => tr!(
            pt: "Baixe em https://github.com/universokobana/kobana-cli/releases/latest",
            en: "Download from https://github.com/universokobana/kobana-cli/releases/latest",
            es: "Descarga en https://github.com/universokobana/kobana-cli/releases/latest",
        ),
    }
}

/// Build a reqwest client with sensible defaults for GitHub API calls.
fn http_client(timeout: Duration) -> Result<reqwest::Client, KobanaError> {
    reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(timeout)
        .build()
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "erro no cliente HTTP: {e}",
            en: "http client error: {e}",
            es: "error en el cliente HTTP: {e}",
        )))
}

/// Fetch the latest release (tag without leading `v`, html_url) from GitHub.
pub async fn fetch_latest_release(timeout: Duration) -> Result<(String, String), KobanaError> {
    let client = http_client(timeout)?;

    let response = client
        .get(GITHUB_API)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível consultar a última versão: {e}",
            en: "failed to fetch latest release: {e}",
            es: "no se pudo consultar la última versión: {e}",
        )))?;

    let status = response.status();
    if !status.is_success() {
        return Err(KobanaError::Internal(trf!(
            pt: "a API do GitHub respondeu {status}",
            en: "GitHub API returned {status}",
            es: "la API de GitHub respondió {status}",
        )));
    }

    let release: GitHubRelease = response
        .json()
        .await
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "resposta de versão inválida: {e}",
            en: "invalid release response: {e}",
            es: "respuesta de versión inválida: {e}",
        )))?;

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
        let report = serde_json::json!({
            "current": current,
            "latest": latest,
            "update_available": update_available,
            "release_url": release_url,
            "install_method": method.as_str(),
            "install_command": command,
        });
        return crate::formatter::print_json(&report, true, false);
    }

    if !update_available {
        println!(
            "{}",
            trf!(
                pt: "kobana {current} já está na versão mais recente.",
                en: "kobana {current} is already up to date.",
                es: "kobana {current} ya está en la versión más reciente.",
            )
        );
        return Ok(());
    }

    println!(
        "{}",
        trf!(
            pt: "Atualização disponível: kobana {current} → {latest}",
            en: "Update available: kobana {current} → {latest}",
            es: "Actualización disponible: kobana {current} → {latest}",
        )
    );

    if check_only {
        println!("{}", run_message(command));
        return Ok(());
    }

    // Ask for confirmation before running anything destructive.
    let prompt = match method {
        InstallMethod::Homebrew | InstallMethod::Winget | InstallMethod::Cargo => trf!(
            pt: "Rodar `{command}` agora?",
            en: "Run `{command}` now?",
            es: "¿Ejecutar `{command}` ahora?",
        ),
        InstallMethod::Standalone => tr!(
            pt: "Baixar e instalar o novo binário agora?",
            en: "Download and install the new binary now?",
            es: "¿Descargar e instalar el nuevo binario ahora?",
        )
        .to_string(),
        InstallMethod::Package => {
            // Needs root and the distro's own tool: only say what to run.
            println!("{}", run_message(command));
            return Ok(());
        }
        InstallMethod::Unknown => {
            println!(
                "{}",
                trf!(
                    pt: "Forma de instalação não reconhecida. {}",
                    en: "Install method not recognized. {}",
                    es: "Forma de instalación no reconocida. {}",
                    run_message(command)
                )
            );
            return Ok(());
        }
    };

    // Only ask a person at a terminal (and never with KOBANA_PROMPT_DISABLED);
    // otherwise print the command and exit, so scripts don't hang.
    if !crate::ui::can_prompt() {
        println!("{}", run_message(command));
        return Ok(());
    }

    let confirm = crate::ui::confirm(&prompt, true).map_err(|e| {
        KobanaError::Internal(trf!(
            pt: "erro na pergunta: {e}",
            en: "prompt error: {e}",
            es: "error en la pregunta: {e}",
        ))
    })?;

    if !confirm {
        println!(
            "{}",
            tr!(
                pt: "Atualização ignorada.",
                en: "Update skipped.",
                es: "Actualización omitida.",
            )
        );
        return Ok(());
    }

    match method {
        InstallMethod::Homebrew => run_shell_command("brew", &["upgrade", "kobana"]),
        InstallMethod::Winget => run_shell_command("winget", &["upgrade", "Kobana.CLI"]),
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
        InstallMethod::Package | InstallMethod::Unknown => Ok(()),
    }
}

fn run_message(command: &str) -> String {
    trf!(pt: "Rode: {command}", en: "Run: {command}", es: "Ejecuta: {command}")
}

/// Run an external command inline, inheriting stdin/stdout/stderr so the user
/// sees progress from tools like brew and cargo.
fn run_shell_command(program: &str, args: &[&str]) -> Result<(), KobanaError> {
    let status = std::process::Command::new(program)
        .args(args)
        .status()
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível rodar {program}: {e}",
            en: "failed to spawn {program}: {e}",
            es: "no se pudo ejecutar {program}: {e}",
        )))?;

    if !status.success() {
        let command = args.join(" ");
        return Err(KobanaError::Internal(trf!(
            pt: "{program} {command} terminou com status {status}",
            en: "{program} {command} exited with status {status}",
            es: "{program} {command} terminó con estado {status}",
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

    eprintln!(
        "{}",
        trf!(
            pt: "Baixando {download_url}...",
            en: "Downloading {download_url}...",
            es: "Descargando {download_url}...",
        )
    );

    // The SHA-256 GitHub published for the asset, checked before anything
    // replaces the running binary
    let release = fetch_release(&format!("v{version}")).await?;
    let Some(expected_sha) = asset_sha256(&release, &asset) else {
        return Err(KobanaError::Internal(trf!(
            pt: "a v{version} não publica o SHA-256 de {asset}; a atualização foi cancelada",
            en: "v{version} publishes no SHA-256 for {asset}; update cancelled",
            es: "la v{version} no publica el SHA-256 de {asset}; se canceló la actualización",
        )));
    };

    let client = http_client(Duration::from_secs(120))?;
    let bytes = client
        .get(&download_url)
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "o download falhou: {e}",
            en: "download failed: {e}",
            es: "la descarga falló: {e}",
        )))?
        .bytes()
        .await
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível ler o download: {e}",
            en: "read download body: {e}",
            es: "no se pudo leer la descarga: {e}",
        )))?;

    let actual_sha = sha256_hex(&bytes);
    if actual_sha != expected_sha {
        return Err(KobanaError::Internal(trf!(
            pt: "o SHA-256 do download ({actual_sha}) não bate com o publicado ({expected_sha}); a atualização foi cancelada",
            en: "the download's SHA-256 ({actual_sha}) does not match the published one ({expected_sha}); update cancelled",
            es: "el SHA-256 de la descarga ({actual_sha}) no coincide con el publicado ({expected_sha}); se canceló la actualización",
        )));
    }

    // Resolve symlinks: install.sh links ~/.local/bin/kobana to
    // ~/.kobana/bin/kobana, and replacing the link itself would leave the
    // installed binary behind.
    let current_exe = std::env::current_exe()
        .and_then(|p| p.canonicalize())
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível localizar o executável: {e}",
            en: "current exe: {e}",
            es: "no se pudo ubicar el ejecutable: {e}",
        )))?;

    // Write to a temp file next to the current exe (same filesystem for atomic rename).
    let temp_path = current_exe.with_file_name(format!(
        ".kobana.new.{}",
        std::process::id()
    ));
    std::fs::write(&temp_path, &bytes)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível gravar o binário temporário: {e}",
            en: "write temp binary: {e}",
            es: "no se pudo guardar el binario temporal: {e}",
        )))?;

    let perms = std::fs::Permissions::from_mode(0o755);
    std::fs::set_permissions(&temp_path, perms)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível ajustar as permissões do binário: {e}",
            en: "chmod temp binary: {e}",
            es: "no se pudieron ajustar los permisos del binario: {e}",
        )))?;

    std::fs::rename(&temp_path, &current_exe).map_err(|e| {
        let _ = std::fs::remove_file(&temp_path);
        KobanaError::Internal(trf!(
            pt: "não foi possível substituir o binário: {e}",
            en: "atomic rename: {e}",
            es: "no se pudo reemplazar el binario: {e}",
        ))
    })?;

    let done = crate::ui::decoration("✓ ");
    eprintln!(
        "{}",
        trf!(
            pt: "{done}Atualizado para a v{version}",
            en: "{done}Updated to v{version}",
            es: "{done}Actualizado a la v{version}",
        )
    );
    Ok(())
}

#[cfg(not(unix))]
async fn perform_self_update(_version: &str) -> Result<(), KobanaError> {
    Err(KobanaError::Internal(
        tr!(
            pt: "a atualização automática não funciona nesta plataforma — baixe o novo binário na página de releases",
            en: "self-update is not supported on this platform — download the new binary manually from the releases page",
            es: "la actualización automática no funciona en esta plataforma — descarga el nuevo binario en la página de releases",
        )
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
        tr!(
            pt: "não há binário publicado para esta plataforma",
            en: "no release binary available for this platform",
            es: "no hay binario publicado para esta plataforma",
        )
        .into(),
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

/// Whether this run checks for a new version: as in gh, only with a person
/// at a terminal (stdout and stderr), outside CI, and never with
/// `KOBANA_NO_UPDATE_NOTIFIER` set. A script or an agent neither waits on
/// the network for it nor gets the notice mixed into its stderr.
pub fn should_auto_check() -> bool {
    let set = |name: &str| crate::ui::truthy(std::env::var(name).ok().as_deref());
    !set("KOBANA_NO_UPDATE_NOTIFIER")
        && !set("CI")
        && std::io::IsTerminal::is_terminal(&std::io::stdout())
        && std::io::IsTerminal::is_terminal(&std::io::stderr())
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
    crate::ui::warn(&trf!(
        pt: "Nova versão disponível: {current} → {latest}. Rode `kobana update` para instalar.",
        en: "New version available: {current} → {latest}. Run `kobana update` to install it.",
        es: "Nueva versión disponible: {current} → {latest}. Ejecuta `kobana update` para instalarla.",
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_published_digest_of_the_asset_is_found() {
        let release: GitHubRelease = serde_json::from_value(serde_json::json!({
            "tag_name": "v1.0.0",
            "html_url": "https://example",
            "assets": [
                {"name": "kobana-x86_64-unknown-linux-gnu", "digest": "sha256:ABC123"},
                {"name": "kobana-aarch64-apple-darwin"}
            ]
        }))
        .unwrap();
        assert_eq!(asset_sha256(&release, "kobana-x86_64-unknown-linux-gnu").as_deref(), Some("abc123"));
        assert_eq!(asset_sha256(&release, "kobana-aarch64-apple-darwin"), None, "no digest");
        assert_eq!(asset_sha256(&release, "kobana-other"), None, "no asset");
    }

    #[test]
    fn install_method_is_read_from_the_path() {
        let cases = [
            ("/opt/homebrew/cellar/kobana/1.0/bin/kobana", InstallMethod::Homebrew),
            (
                "c:\\users\\a\\appdata\\local\\microsoft\\winget\\packages\\kobana.cli_x\\kobana.exe",
                InstallMethod::Winget,
            ),
            ("/usr/bin/kobana", InstallMethod::Package),
            ("/home/a/.cargo/bin/kobana", InstallMethod::Cargo),
            ("/home/a/.kobana/bin/kobana", InstallMethod::Standalone),
        ];
        for (path, expected) in cases {
            assert_eq!(classify_install_path(path), expected, "{path}");
        }
    }

    #[test]
    fn sha256_is_lowercase_hex() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
