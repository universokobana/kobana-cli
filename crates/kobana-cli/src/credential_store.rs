use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use kobana::error::KobanaError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::config;

const KEYRING_SERVICE: &str = "kobana-cli";
const KEYRING_USER: &str = "encryption-key";
const NONCE_SIZE: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_at: Option<i64>,
    pub environment: String,
}

/// Save credentials encrypted to disk
pub fn save_credentials(creds: &StoredCredentials) -> Result<(), KobanaError> {
    let config_dir = config::config_dir();
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| KobanaError::Internal(format!("failed to create config dir: {e}")))?;

    let key = get_or_create_encryption_key()?;
    let plaintext =
        serde_json::to_vec(creds).map_err(|e| KobanaError::Internal(e.to_string()))?;

    let encrypted = encrypt(&key, &plaintext)?;
    let cred_path = credentials_path();

    std::fs::write(&cred_path, &encrypted)
        .map_err(|e| KobanaError::Internal(format!("failed to write credentials: {e}")))?;

    // Restrict file permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&cred_path, perms)
            .map_err(|e| KobanaError::Internal(format!("failed to set permissions: {e}")))?;
    }

    Ok(())
}

/// Load credentials from encrypted file
pub fn load_credentials() -> Result<StoredCredentials, KobanaError> {
    let cred_path = credentials_path();
    if !cred_path.exists() {
        return Err(KobanaError::Auth("No saved credentials found".into()));
    }

    let encrypted = std::fs::read(&cred_path)
        .map_err(|e| KobanaError::Auth(format!("failed to read credentials: {e}")))?;

    let key = get_encryption_key()?;
    let plaintext = decrypt(&key, &encrypted)?;

    serde_json::from_slice(&plaintext)
        .map_err(|e| KobanaError::Auth(format!("corrupted credentials file: {e}")))
}

/// Delete saved credentials
pub fn delete_credentials() -> Result<(), KobanaError> {
    let cred_path = credentials_path();
    if cred_path.exists() {
        // Overwrite with zeros before deleting
        let size = std::fs::metadata(&cred_path)
            .map(|m| m.len() as usize)
            .unwrap_or(0);
        if size > 0 {
            let _ = std::fs::write(&cred_path, vec![0u8; size]);
        }
        std::fs::remove_file(&cred_path)
            .map_err(|e| KobanaError::Internal(format!("failed to delete credentials: {e}")))?;
    }

    // Also try to remove the keyring entry
    let _ = delete_keyring_key();

    Ok(())
}

fn credentials_path() -> PathBuf {
    config::config_dir().join("credentials.enc")
}

fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, KobanaError> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| KobanaError::Internal(format!("encryption init error: {e}")))?;

    let nonce_bytes: [u8; NONCE_SIZE] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| KobanaError::Internal(format!("encryption error: {e}")))?;

    // Prepend nonce to ciphertext
    let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

fn decrypt(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, KobanaError> {
    if data.len() < NONCE_SIZE {
        return Err(KobanaError::Auth("credentials file too short".into()));
    }

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| KobanaError::Internal(format!("decryption init error: {e}")))?;

    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| KobanaError::Auth("failed to decrypt credentials (wrong key or corrupted file)".into()))
}

/// Get or create the encryption key, stored in OS keyring with file fallback
fn get_or_create_encryption_key() -> Result<[u8; 32], KobanaError> {
    // Try OS keyring first
    if let Ok(key) = get_keyring_key() {
        return Ok(key);
    }

    // Try file fallback
    let key_path = config::config_dir().join(".key");
    if key_path.exists() {
        return load_key_from_file(&key_path);
    }

    // Generate new key
    let key: [u8; 32] = rand::random();

    // Try to store in keyring
    if save_keyring_key(&key).is_ok() {
        return Ok(key);
    }

    // Fall back to file
    save_key_to_file(&key_path, &key)?;
    Ok(key)
}

fn get_encryption_key() -> Result<[u8; 32], KobanaError> {
    // Try OS keyring
    if let Ok(key) = get_keyring_key() {
        return Ok(key);
    }

    // Try file fallback
    let key_path = config::config_dir().join(".key");
    if key_path.exists() {
        return load_key_from_file(&key_path);
    }

    Err(KobanaError::Auth(
        "encryption key not found (credentials may be from a different machine)".into(),
    ))
}

fn get_keyring_key() -> Result<[u8; 32], KobanaError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| KobanaError::Internal(format!("keyring error: {e}")))?;

    let secret = entry
        .get_password()
        .map_err(|_| KobanaError::Auth("no key in keyring".into()))?;

    decode_key(&secret)
}

fn save_keyring_key(key: &[u8; 32]) -> Result<(), KobanaError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| KobanaError::Internal(format!("keyring error: {e}")))?;

    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key);
    entry
        .set_password(&encoded)
        .map_err(|e| KobanaError::Internal(format!("failed to save to keyring: {e}")))
}

fn delete_keyring_key() -> Result<(), KobanaError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
        .map_err(|e| KobanaError::Internal(format!("keyring error: {e}")))?;

    entry
        .delete_credential()
        .map_err(|e| KobanaError::Internal(format!("failed to delete keyring entry: {e}")))
}

fn save_key_to_file(path: &PathBuf, key: &[u8; 32]) -> Result<(), KobanaError> {
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key);
    std::fs::write(path, encoded.as_bytes())
        .map_err(|e| KobanaError::Internal(format!("failed to write key file: {e}")))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(path, perms)
            .map_err(|e| KobanaError::Internal(format!("failed to set key permissions: {e}")))?;
    }

    Ok(())
}

fn load_key_from_file(path: &PathBuf) -> Result<[u8; 32], KobanaError> {
    let encoded = std::fs::read_to_string(path)
        .map_err(|e| KobanaError::Auth(format!("failed to read key file: {e}")))?;
    decode_key(encoded.trim())
}

fn decode_key(encoded: &str) -> Result<[u8; 32], KobanaError> {
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| KobanaError::Auth(format!("invalid key encoding: {e}")))?;

    bytes
        .try_into()
        .map_err(|_| KobanaError::Auth("invalid key length".into()))
}
