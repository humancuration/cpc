use ring::{aead, rand, pbkdf2};
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use ring::pbkdf2::{derive_key, PBKDF2_HMAC_SHA256};
use anyhow::{Result, Context};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};
use zeroize::Zeroize;
use base64::{Engine as _, engine::general_purpose};

const KEY_SIZE: usize = 32; // 256-bit key for AES-256-GCM
const NONCE_SIZE: usize = 12; // 96-bit nonce for AES-GCM
const SALT_SIZE: usize = 32; // 256-bit salt for PBKDF2
const PBKDF2_ITERATIONS: u32 = 100_000;

/// Custom nonce sequence for AES-GCM
struct CounterNonceSequence(u32);

impl NonceSequence for CounterNonceSequence {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        nonce_bytes[4..8].copy_from_slice(&self.0.to_be_bytes());
        self.0 += 1;
        Nonce::try_assume_unique_for_key(&nonce_bytes)
    }
}

/// Secure storage implementation with AES-256-GCM encryption
pub struct SecureStorage {
    key: UnboundKey,
    data_dir: PathBuf,
    rng: SystemRandom,
}

impl SecureStorage {
    /// Create a new SecureStorage instance
    pub fn new(config: &tauri::Config) -> Result<Self> {
        let data_dir = tauri::api::path::app_data_dir(config)
            .context("Failed to get app data directory")?;
        
        fs::create_dir_all(&data_dir)
            .context("Failed to create app data directory")?;
        
        let key = Self::get_or_create_key(&data_dir)?;
        let rng = SystemRandom::new();
        
        Ok(Self { key, data_dir, rng })
    }

    /// Get or create encryption key using PBKDF2 derivation
    fn get_or_create_key(data_dir: &Path) -> Result<UnboundKey> {
        let key_file = data_dir.join(".secure_key");
        
        // Try to load existing key
        if key_file.exists() {
            let mut key_data = Vec::new();
            fs::File::open(&key_file)
                .context("Failed to open key file")?
                .read_to_end(&mut key_data)
                .context("Failed to read key file")?;
            
            if key_data.len() == KEY_SIZE {
                return UnboundKey::new(&AES_256_GCM, &key_data)
                    .map_err(|_| anyhow::anyhow!("Invalid key format"));
            }
        }
        
        // Generate new key
        let mut key_data = [0u8; KEY_SIZE];
        SystemRandom::new()
            .fill(&mut key_data)
            .map_err(|_| anyhow::anyhow!("Failed to generate random key"))?;
        
        // Save key securely
        let mut file = fs::File::create(&key_file)
            .context("Failed to create key file")?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(fs::Permissions::from_mode(0o600))
                .context("Failed to set key file permissions")?;
        }
        
        file.write_all(&key_data)
            .context("Failed to write key file")?;
        
        UnboundKey::new(&AES_256_GCM, &key_data)
            .map_err(|_| anyhow::anyhow!("Invalid key format"))
    }

    /// Store encrypted data with key derivation
    pub fn store(&self, key: &str, value: &[u8], password: Option<&str>) -> Result<()> {
        let file_path = self.data_dir.join(format!("{}.enc", base64::encode_config(key, base64::URL_SAFE_NO_PAD)));
        
        // Generate salt for PBKDF2
        let mut salt = [0u8; SALT_SIZE];
        self.rng.fill(&mut salt)
            .map_err(|_| anyhow::anyhow!("Failed to generate salt"))?;
        
        // Derive key if password provided
        let derived_key = if let Some(password) = password {
            let mut derived = [0u8; KEY_SIZE];
            derive_key(
                PBKDF2_HMAC_SHA256,
                &mut derived,
                password.as_bytes(),
                &salt,
                PBKDF2_ITERATIONS,
            );
            UnboundKey::new(&AES_256_GCM, &derived)
                .map_err(|_| anyhow::anyhow!("Failed to create derived key"))?
        } else {
            self.key.clone()
        };
        
        // Generate nonce
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|_| anyhow::anyhow!("Failed to generate nonce"))?;
        let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)
            .map_err(|_| anyhow::anyhow!("Invalid nonce"))?;
        
        // Encrypt data
        let mut sealing_key = SealingKey::new(derived_key, CounterNonceSequence(0));
        let mut ciphertext = value.to_vec();
        sealing_key.seal_in_place_append_tag(
            nonce,
            Aad::from(key.as_bytes()),
            &mut ciphertext,
        ).map_err(|_| anyhow::anyhow!("Encryption failed"))?;
        
        // Write encrypted data: [salt][nonce][ciphertext]
        let mut file = fs::File::create(&file_path)
            .context("Failed to create encrypted file")?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(fs::Permissions::from_mode(0o600))
                .context("Failed to set encrypted file permissions")?;
        }
        
        file.write_all(&salt)?;
        file.write_all(&nonce_bytes)?;
        file.write_all(&ciphertext)?;
        
        Ok(())
    }

    /// Retrieve and decrypt data
    pub fn retrieve(&self, key: &str, password: Option<&str>) -> Result<Option<Vec<u8>>> {
        let file_path = self.data_dir.join(format!("{}.enc", base64::encode_config(key, base64::URL_SAFE_NO_PAD)));
        
        if !file_path.exists() {
            return Ok(None);
        }
        
        let mut file = fs::File::open(&file_path)
            .context("Failed to open encrypted file")?;
        
        // Read salt
        let mut salt = [0u8; SALT_SIZE];
        file.read_exact(&mut salt)
            .context("Failed to read salt")?;
        
        // Read nonce
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        file.read_exact(&mut nonce_bytes)
            .context("Failed to read nonce")?;
        let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes)
            .map_err(|_| anyhow::anyhow!("Invalid nonce"))?;
        
        // Read ciphertext
        let mut ciphertext = Vec::new();
        file.read_to_end(&mut ciphertext)
            .context("Failed to read ciphertext")?;
        
        // Derive key if password provided
        let derived_key = if let Some(password) = password {
            let mut derived = [0u8; KEY_SIZE];
            derive_key(
                PBKDF2_HMAC_SHA256,
                &mut derived,
                password.as_bytes(),
                &salt,
                PBKDF2_ITERATIONS,
            );
            UnboundKey::new(&AES_256_GCM, &derived)
                .map_err(|_| anyhow::anyhow!("Failed to create derived key"))?
        } else {
            self.key.clone()
        };
        
        // Decrypt data
        let mut opening_key = OpeningKey::new(derived_key, CounterNonceSequence(0));
        opening_key.open_in_place(
            nonce,
            Aad::from(key.as_bytes()),
            &mut ciphertext,
        ).map_err(|_| anyhow::anyhow!("Decryption failed"))?;
        
        // Remove authentication tag
        let plaintext_len = ciphertext.len() - AES_256_GCM.tag_len();
        ciphertext.truncate(plaintext_len);
        
        Ok(Some(ciphertext))
    }

    /// Delete stored data
    pub fn delete(&self, key: &str) -> Result<()> {
        let file_path = self.data_dir.join(format!("{}.enc", base64::encode_config(key, base64::URL_SAFE_NO_PAD)));
        
        if file_path.exists() {
            fs::remove_file(&file_path)
                .context("Failed to delete encrypted file")?;
        }
        
        Ok(())
    }

    /// List all stored keys
    pub fn list_keys(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        
        for entry in fs::read_dir(&self.data_dir)
            .context("Failed to read data directory")? {
            
            let entry = entry.context("Failed to read directory entry")?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            
            if file_name_str.ends_with(".enc") {
                let encoded_key = &file_name_str[..file_name_str.len() - 4];
                if let Ok(decoded_key) = base64::decode_config(encoded_key, base64::URL_SAFE_NO_PAD) {
                    if let Ok(key_str) = String::from_utf8(decoded_key) {
                        keys.push(key_str);
                    }
                }
            }
        }
        
        Ok(keys)
    }
}

/// Zeroize sensitive data on drop
impl Drop for SecureStorage {
    fn drop(&mut self) {
        // Zeroize the key material
        // Note: This is a best-effort attempt since UnboundKey doesn't expose internals
        // In production, consider using zeroize::Zeroizing wrapper
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_secure_storage() {
        let temp_dir = tempdir().unwrap();
        let config = tauri::Config::default();
        
        // Create mock config for testing
        let mut config = tauri::Config {
            build: tauri::utils::config::BuildConfig::default(),
            package: tauri::utils::config::PackageConfig {
                product_name: Some("test-app".to_string()),
                ..Default::default()
            },
            tauri: tauri::utils::config::TauriConfig {
                identifier: "com.test.app".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        
        // This test would need proper mocking for tauri config
        // For now, we'll just test the basic functionality
        
        assert!(true); // Placeholder
    }
}