use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use kobana::error::KobanaError;
use kobana::{tr, trf};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::config;

const KEYRING_SERVICE: &str = "kobana-cli";
/// The keyring entry every config directory shared up to 0.10.3. Still read,
/// for credentials written before keys were per directory, but never deleted:
/// another directory may depend on it.
const LEGACY_KEYRING_USER: &str = "encryption-key";
const NONCE_SIZE: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_at: Option<i64>,
    pub environment: String,
    /// Product the token was issued for. Absent in the single
    /// `credentials.enc` written up to 0.9.0, which was always banking.
    #[serde(default)]
    pub product: Option<String>,
    /// OAuth client the token was issued to, needed to refresh it
    #[serde(default)]
    pub client_id: Option<String>,
    /// `resource` (the token's `aud`) asked at login, sent again on refresh
    #[serde(default)]
    pub resource: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

/// Save a product's credentials encrypted to disk
pub fn save_credentials(
    product: &str,
    env: &str,
    creds: &StoredCredentials,
) -> Result<(), KobanaError> {
    let config_dir = config::config_dir();
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível criar a pasta de configuração: {e}",
            en: "failed to create config dir: {e}",
            es: "no se pudo crear la carpeta de configuración: {e}",
        )))?;

    let key = get_or_create_encryption_key(&config_dir)?;
    let plaintext =
        serde_json::to_vec(creds).map_err(|e| KobanaError::Internal(e.to_string()))?;

    let encrypted = encrypt(&key, &plaintext)?;
    let cred_path = credentials_path(product, env);

    std::fs::write(&cred_path, &encrypted)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível gravar as credenciais: {e}",
            en: "failed to write credentials: {e}",
            es: "no se pudieron guardar las credenciales: {e}",
        )))?;

    // Restrict file permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&cred_path, perms)
            .map_err(|e| KobanaError::Internal(trf!(
                pt: "não foi possível ajustar as permissões: {e}",
                en: "failed to set permissions: {e}",
                es: "no se pudieron ajustar los permisos: {e}",
            )))?;
    }

    Ok(())
}

/// Load a product's saved credentials. Banking falls back to the single
/// `credentials.enc` of 0.9.0 and earlier when it was saved for `env`.
pub fn load_credentials(product: &str, env: &str) -> Result<StoredCredentials, KobanaError> {
    let path = credentials_path(product, env);
    if path.exists() {
        return load_from(&path);
    }
    if product == "banking" && legacy_credentials_path().exists() {
        let creds = load_from(&legacy_credentials_path())?;
        if creds.environment == env {
            return Ok(creds);
        }
    }
    Err(KobanaError::Auth(tr!(
        pt: "Nenhuma credencial salva",
        en: "No saved credentials found",
        es: "No hay credenciales guardadas",
    ).into()))
}

/// Whether `load_credentials` would find something
pub fn has_credentials(product: &str, env: &str) -> bool {
    credentials_path(product, env).exists()
        || (product == "banking"
            && legacy_credentials_path().exists()
            && load_from(&legacy_credentials_path()).is_ok_and(|c| c.environment == env))
}

fn load_from(cred_path: &PathBuf) -> Result<StoredCredentials, KobanaError> {
    if !cred_path.exists() {
        return Err(KobanaError::Auth(tr!(
            pt: "Nenhuma credencial salva",
            en: "No saved credentials found",
            es: "No hay credenciales guardadas",
        ).into()));
    }

    let encrypted = std::fs::read(cred_path)
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "não foi possível ler as credenciais: {e}",
            en: "failed to read credentials: {e}",
            es: "no se pudieron leer las credenciales: {e}",
        )))?;

    let plaintext = decrypt_with_any_key(&config::config_dir(), &encrypted)?;

    serde_json::from_slice(&plaintext)
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "arquivo de credenciais corrompido: {e}",
            en: "corrupted credentials file: {e}",
            es: "archivo de credenciales dañado: {e}",
        )))
}

/// Delete a product's saved credentials (banking also drops the legacy file),
/// and this directory's encryption key once no credential is left in it.
/// Returns whether anything was removed.
pub fn delete_credentials(product: &str, env: &str) -> Result<bool, KobanaError> {
    let mut removed = wipe(&credentials_path(product, env))?;
    if product == "banking" && legacy_credentials_path().exists() {
        let legacy_env = load_from(&legacy_credentials_path()).map(|c| c.environment);
        if legacy_env.map_or(true, |e| e == env) {
            removed |= wipe(&legacy_credentials_path())?;
        }
    }
    // The key goes with the last credential it encrypts. Only this
    // directory's key: the keyring entry is per directory, and the legacy
    // shared entry may still serve another one.
    if saved_credential_files().is_empty() {
        let dir = config::config_dir();
        let _ = delete_keyring_key(&keyring_user(&dir));
        wipe(&key_file_path(&dir))?;
    }
    Ok(removed)
}

/// Every saved credential file, of any product and environment
fn saved_credential_files() -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(config::config_dir()) else {
        return Vec::new();
    };
    entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("credentials") && n.ends_with(".enc"))
        })
        .collect()
}

/// Overwrite with zeros, then delete. Returns whether the file existed.
fn wipe(path: &PathBuf) -> Result<bool, KobanaError> {
    if !path.exists() {
        return Ok(false);
    }
    let size = std::fs::metadata(path).map(|m| m.len() as usize).unwrap_or(0);
    if size > 0 {
        let _ = std::fs::write(path, vec![0u8; size]);
    }
    std::fs::remove_file(path).map_err(|e| KobanaError::Internal(trf!(
        pt: "não foi possível apagar as credenciais: {e}",
        en: "failed to delete credentials: {e}",
        es: "no se pudieron eliminar las credenciales: {e}",
    )))?;
    Ok(true)
}

/// One file per product and environment: `credentials-billing-sandbox.enc`
pub fn credentials_path(product: &str, env: &str) -> PathBuf {
    config::config_dir().join(format!("credentials-{product}-{env}.enc"))
}

/// The single banking credential written by 0.9.0 and earlier
fn legacy_credentials_path() -> PathBuf {
    config::config_dir().join("credentials.enc")
}

/// Returns the storage backend of the key new credentials of this directory
/// are encrypted with: "file" when a `.key` is there, "keyring" otherwise.
pub fn key_backend() -> &'static str {
    match key_sources(&config::config_dir()).first() {
        Some(KeySource::File(_)) | None => "file",
        Some(KeySource::Keyring(_)) => "keyring",
    }
}

fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, KobanaError> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "erro ao iniciar a criptografia: {e}",
            en: "encryption init error: {e}",
            es: "error al iniciar el cifrado: {e}",
        )))?;

    let nonce_bytes: [u8; NONCE_SIZE] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "erro de criptografia: {e}",
            en: "encryption error: {e}",
            es: "error de cifrado: {e}",
        )))?;

    // Prepend nonce to ciphertext
    let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

fn decrypt(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, KobanaError> {
    if data.len() < NONCE_SIZE {
        return Err(KobanaError::Auth(tr!(
            pt: "arquivo de credenciais curto demais",
            en: "credentials file too short",
            es: "archivo de credenciales demasiado corto",
        ).into()));
    }

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "erro ao iniciar a descriptografia: {e}",
            en: "decryption init error: {e}",
            es: "error al iniciar el descifrado: {e}",
        )))?;

    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| KobanaError::Auth(tr!(
            pt: "não foi possível descriptografar as credenciais (chave errada ou arquivo corrompido)",
            en: "failed to decrypt credentials (wrong key or corrupted file)",
            es: "no se pudieron descifrar las credenciales (clave incorrecta o archivo dañado)",
        ).into()))
}

/// Where an encryption key may live for a config directory
#[derive(Debug, Clone, PartialEq, Eq)]
enum KeySource {
    File(PathBuf),
    Keyring(String),
}

/// `.key` in the config directory
fn key_file_path(dir: &Path) -> PathBuf {
    dir.join(".key")
}

/// The keyring entry of a config directory. The entry is global to the user,
/// so each directory gets its own (`KOBANA_CONFIG_DIR=/tmp/x` must not
/// share, nor delete, the key of the default directory).
fn keyring_user(dir: &Path) -> String {
    use sha2::{Digest, Sha256};
    let dir = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    let digest = Sha256::digest(dir.to_string_lossy().as_bytes());
    let hex: String = digest.iter().take(8).map(|b| format!("{b:02x}")).collect();
    format!("{LEGACY_KEYRING_USER}-{hex}")
}

/// The keys that may have encrypted this directory's credentials, in the
/// order they are tried. A `.key` comes first: a directory that has one
/// encrypted with it, and must stay readable when a keyring entry appears.
fn key_sources(dir: &Path) -> Vec<KeySource> {
    let mut sources = Vec::new();
    let file = key_file_path(dir);
    if file.exists() {
        sources.push(KeySource::File(file));
    }
    for user in [keyring_user(dir), LEGACY_KEYRING_USER.to_string()] {
        if get_keyring_key(&user).is_ok() {
            sources.push(KeySource::Keyring(user));
        }
    }
    sources
}

fn load_key(source: &KeySource) -> Result<[u8; 32], KobanaError> {
    match source {
        KeySource::File(path) => load_key_from_file(path),
        KeySource::Keyring(user) => get_keyring_key(user),
    }
}

/// Decrypt with the first of the directory's keys that opens the data
fn decrypt_with_any_key(dir: &Path, data: &[u8]) -> Result<Vec<u8>, KobanaError> {
    let sources = key_sources(dir);
    if sources.is_empty() {
        return Err(KobanaError::Auth(
            tr!(
                pt: "chave de criptografia não encontrada (as credenciais podem ser de outra máquina)",
                en: "encryption key not found (credentials may be from a different machine)",
                es: "clave de cifrado no encontrada (las credenciales pueden ser de otra máquina)",
            )
            .into(),
        ));
    }
    let mut last_err = None;
    for source in &sources {
        match load_key(source).and_then(|key| decrypt(&key, data)) {
            Ok(plaintext) => return Ok(plaintext),
            Err(e) => last_err = Some(e),
        }
    }
    Err(last_err.expect("at least one key source"))
}

/// The key new credentials are encrypted with: the directory's `.key` or its
/// own keyring entry, or a new one in that entry (a `.key` file when the
/// keyring does not keep it). Never the legacy shared entry, so a directory
/// moves off it the next time it saves.
fn get_or_create_encryption_key(dir: &Path) -> Result<[u8; 32], KobanaError> {
    let own = KeySource::Keyring(keyring_user(dir));
    if let Some(source) = key_sources(dir)
        .into_iter()
        .find(|s| matches!(s, KeySource::File(_)) || *s == own)
    {
        return load_key(&source);
    }

    let key: [u8; 32] = rand::random();

    // Try to store in keyring, and verify by reading back.
    // Some keyring backends silently "succeed" without persisting.
    let user = keyring_user(dir);
    let keyring_ok = save_keyring_key(&user, &key).is_ok() && get_keyring_key(&user).is_ok_and(|k| k == key);

    // Fall back to a file only when the keyring did not keep the key, so the
    // key is never written to disk when the keyring works.
    if !keyring_ok {
        save_key_to_file(&key_file_path(dir), &key)?;
    }

    Ok(key)
}

fn get_keyring_key(user: &str) -> Result<[u8; 32], KobanaError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, user)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "erro no keyring: {e}",
            en: "keyring error: {e}",
            es: "error en el keyring: {e}",
        )))?;

    let secret = entry
        .get_password()
        .map_err(|_| KobanaError::Auth(tr!(
            pt: "nenhuma chave no keyring",
            en: "no key in keyring",
            es: "ninguna clave en el keyring",
        ).into()))?;

    decode_key(&secret)
}

fn save_keyring_key(user: &str, key: &[u8; 32]) -> Result<(), KobanaError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, user)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "erro no keyring: {e}",
            en: "keyring error: {e}",
            es: "error en el keyring: {e}",
        )))?;

    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key);
    entry
        .set_password(&encoded)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível salvar no keyring: {e}",
            en: "failed to save to keyring: {e}",
            es: "no se pudo guardar en el keyring: {e}",
        )))
}

fn delete_keyring_key(user: &str) -> Result<(), KobanaError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, user)
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "erro no keyring: {e}",
            en: "keyring error: {e}",
            es: "error en el keyring: {e}",
        )))?;

    entry
        .delete_credential()
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível apagar a entrada do keyring: {e}",
            en: "failed to delete keyring entry: {e}",
            es: "no se pudo eliminar la entrada del keyring: {e}",
        )))
}

fn save_key_to_file(path: &Path, key: &[u8; 32]) -> Result<(), KobanaError> {
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key);
    std::fs::write(path, encoded.as_bytes())
        .map_err(|e| KobanaError::Internal(trf!(
            pt: "não foi possível gravar o arquivo da chave: {e}",
            en: "failed to write key file: {e}",
            es: "no se pudo guardar el archivo de la clave: {e}",
        )))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(path, perms)
            .map_err(|e| KobanaError::Internal(trf!(
                pt: "não foi possível ajustar as permissões da chave: {e}",
                en: "failed to set key permissions: {e}",
                es: "no se pudieron ajustar los permisos de la clave: {e}",
            )))?;
    }

    Ok(())
}

fn load_key_from_file(path: &Path) -> Result<[u8; 32], KobanaError> {
    let encoded = std::fs::read_to_string(path)
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "não foi possível ler o arquivo da chave: {e}",
            en: "failed to read key file: {e}",
            es: "no se pudo leer el archivo de la clave: {e}",
        )))?;
    decode_key(encoded.trim())
}

fn decode_key(encoded: &str) -> Result<[u8; 32], KobanaError> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| KobanaError::Auth(trf!(
            pt: "codificação da chave inválida: {e}",
            en: "invalid key encoding: {e}",
            es: "codificación de la clave inválida: {e}",
        )))?;

    bytes
        .try_into()
        .map_err(|_| KobanaError::Auth(tr!(
            pt: "tamanho de chave inválido",
            en: "invalid key length",
            es: "longitud de clave inválida",
        ).into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("kobana-cs-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn each_config_directory_has_its_own_keyring_entry() {
        let a = keyring_user(Path::new("/tmp/kobana-a"));
        let b = keyring_user(Path::new("/tmp/kobana-b"));
        assert_ne!(a, b);
        assert_ne!(a, LEGACY_KEYRING_USER);
        assert_eq!(a, keyring_user(Path::new("/tmp/kobana-a")), "stable");
    }

    #[test]
    fn a_key_file_wins_over_the_keyring() {
        let dir = temp_dir("keyfile");
        let key: [u8; 32] = rand::random();
        save_key_to_file(&key_file_path(&dir), &key).unwrap();

        assert_eq!(key_sources(&dir).first(), Some(&KeySource::File(key_file_path(&dir))));
        assert_eq!(get_or_create_encryption_key(&dir).unwrap(), key);

        let data = encrypt(&key, b"secret").unwrap();
        assert_eq!(decrypt_with_any_key(&dir, &data).unwrap(), b"secret");
        std::fs::remove_dir_all(dir).ok();
    }
}
