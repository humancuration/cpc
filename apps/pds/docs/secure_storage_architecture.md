# Secure Storage Architecture for Desktop App

## Overview
This document describes the secure storage architecture for the desktop app, providing equivalent functionality to Android's SecureStorage implementation while being cross-platform.

## Components

### 1. SecureStorage Rust Module
```rust
use ring::{aead, rand};
use anyhow::{Result, Context};
use std::fs;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;

const KEY_SIZE: usize = 32; // 256-bit key for AES-256-GCM

pub struct SecureStorage {
    key: aead::UnboundKey,
    data_dir: PathBuf,
}

impl SecureStorage {
    pub fn new(config: &tauri::Config) -> Result<Self> {
        let data_dir = app_data_dir(config).context("Failed to get app data dir")?;
        fs::create_dir_all(&data_dir)?;
        
        let key = Self::get_or_create_key()?;
        Ok(Self { key, data_dir })
    }

    fn get_or_create_key() -> Result<aead::UnboundKey> {
        // Implementation to retrieve or generate encryption key
    }

    pub fn store(&self, key: &str, value: &[u8]) -> Result<()> {
        // Encryption implementation using AES-256-GCM
    }

    pub fn retrieve(&self, key: &str) -> Result<Option<Vec<u8>>> {
        // Decryption implementation
    }
}
```

### 2. Tauri Commands
```rust
#[tauri::command]
fn secure_store(key: String, value: String) -> Result<(), String> {
    let storage = SecureStorage::new(&app.config())
        .map_err(|e| format!("Storage init failed: {}", e))?;
    storage.store(&key, value.as_bytes())
        .map_err(|e| format!("Store failed: {}", e))
}

#[tauri::command]
fn secure_retrieve(key: String) -> Result<Option<String>, String> {
    let storage = SecureStorage::new(&app.config())
        .map_err(|e| format!("Storage init failed: {}", e))?;
    storage.retrieve(&key)
        .map(|opt| opt.map(|bytes| String::from_utf8_lossy(&bytes).into_owned()))
        .map_err(|e| format!("Retrieve failed: {}", e))
}
```

### 3. Frontend Store
```javascript
import { invoke } from '@tauri-apps/api/tauri'

export async function secureStore(key, value) {
    return invoke('secure_store', { key, value });
}

export async function secureRetrieve(key) {
    return invoke('secure_retrieve', { key });
}
```

## Security Considerations

1. **Key Management**:
   - Use PBKDF2 with 100,000 iterations to derive keys from a master password
   - Store derived keys in Tauri's secure storage location with appropriate file permissions

2. **Encryption**:
   - AES-256-GCM algorithm for authenticated encryption
   - 12-byte random nonce for each encryption operation
   - Store nonce alongside ciphertext (12 bytes + ciphertext)

3. **Memory Safety**:
   - Zeroize sensitive data in memory after use
   - Use Rust's ownership system to prevent dangling references

4. **Platform-specific Protections**:
   - On Windows: Use encrypted filesystem if available
   - On macOS: Leverage keychain for master key storage
   - On Linux: Use secret service API via libsecret

## File Structure
```
apps/
├── pds/
│   ├── src/
│   │   └── secure_storage.rs       # SecureStorage implementation
│   ├── src-tauri/
│   │   ├── src/
│   │   │   └── lib.rs              # Tauri commands
│   │   └── Cargo.toml              # Add ring and anyhow dependencies
│   └── frontend/
│       └── src/
│           └── stores/
│               └── secureStorage.js # Frontend store
└── docs/
    └── secure_storage_architecture.md
```

## Dependencies
```toml
# apps/pds/src-tauri/Cargo.toml
[dependencies]
ring = "0.17"        # MIT license
anyhow = "1.0"       # MIT/Apache-2.0
```

## Integration Points
1. StorageConfigPanel.svelte will use secureStorage.js for sensitive configuration data
2. Existing storage metrics will be extended to include secure storage usage statistics