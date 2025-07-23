use ring::{aead, rand, pbkdf2};
use anyhow::{Result, Context};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use std::num::NonZeroU32;

const KEY_SIZE: usize = 32; // 256-bit key for AES-256-GCM
const SALT: &[u8] = b"cpc_secure_storage_salt_2025"; // Fixed salt for key derivation
const PBKDF2_ITERATIONS: u32 = 100_000;

pub struct SecureStorage {
    key: aead::LessSafeKey,
    data_dir: PathBuf,
}

impl SecureStorage {
    pub fn new(config: &tauri::Config) -> Result<Self> {
        let data_dir = app_data_dir(config).context("Failed to get app data dir")?;
        fs::create_dir_all(&data_dir)?;
        
        let key = Self::get_or_create_key()?;
        Ok(Self { key, data_dir })
    }

    fn get_or_create_key() -> Result<aead::LessSafeKey> {
        // For now, derive key from a fixed password - in production, this should be user-provided
        let mut key_bytes = [0u8; KEY_SIZE];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
            SALT,
            b"cpc_master_key_2025", // TODO: Replace with user-provided password
            &mut key_bytes
        );
        
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
            .map_err(|_| anyhow::anyhow!("Key creation failed"))?;
        
        Ok(aead::LessSafeKey::new(unbound_key))
    }

    pub fn store(&self, key: &str, value: &[u8]) -> Result<()> {
        // Generate random nonce
        let nonce_bytes = rand::generate(&rand::SystemRandom::new())
            .map_err(|_| anyhow::anyhow!("Failed to generate nonce"))?
            .expose();
        
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        
        // Prepare buffer with space for nonce and tag
        let mut buffer = Vec::with_capacity(value.len() + aead::AES_256_GCM.tag_len());
        buffer.extend_from_slice(value);
        
        // Encrypt in place
        let tag = self.key.seal_in_place_append_tag(
            nonce,
            aead::Aad::empty(),
            &mut buffer
        ).map_err(|_| anyhow::anyhow!("Encryption failed"))?;
        
        // Combine nonce + encrypted data + tag
        let mut final_data = Vec::with_capacity(12 + buffer.len());
        final_data.extend_from_slice(&nonce_bytes);
        final_data.extend_from_slice(&buffer);
        
        // Write to file
        fs::write(self.data_dir.join(key), final_data)?;
        
        Ok(())
    }

    pub fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let path = self.data_dir.join(key);
        if !path.exists() {
            return Ok(None);
        }
        
        let full_data = fs::read(&path)?;
        if full_data.len() < 12 + aead::AES_256_GCM.tag_len() {
            return Err(anyhow::anyhow!("Invalid encrypted data format"));
        }
        
        // Split nonce and encrypted data
        let (nonce_bytes, encrypted_data) = full_data.split_at(12);
        let nonce = aead::Nonce::assume_unique_for_key(
            nonce_bytes.try_into().unwrap()
        );
        
        // Decrypt
        let mut buffer = encrypted_data.to_vec();
        let decrypted = self.key.open_in_place(
            nonce,
            aead::Aad::empty(),
            &mut buffer
        ).map_err(|_| anyhow::anyhow!("Decryption failed"))?;
        
        Ok(Some(decrypted.to_vec()))
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        let path = self.data_dir.join(key);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn list_keys(&self) -> Result<Vec<String>> {
        let mut keys = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.data_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    keys.push(name.to_string());
                }
            }
        }
        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tauri::Config;

    #[test]
    fn test_store_and_retrieve() {
        let config = Config::default();
        let storage = SecureStorage::new(&config).unwrap();
        
        let test_key = "test_key";
        let test_value = b"test_value";
        
        storage.store(test_key, test_value).unwrap();
        let retrieved = storage.retrieve(test_key).unwrap();
        
        assert_eq!(retrieved, Some(test_value.to_vec()));
        
        // Clean up
        storage.delete(test_key).unwrap();
    }

    #[test]
    fn test_nonexistent_key() {
        let config = Config::default();
        let storage = SecureStorage::new(&config).unwrap();
        
        let retrieved = storage.retrieve("nonexistent_key").unwrap();
        assert_eq!(retrieved, None);
    }
}