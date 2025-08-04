// Storage: encrypted project save/load and SQLx metadata stubs.
// Uses RustCrypto (AES-GCM + SHA-256) for encryption and SQLx for metadata.
// Note: DB connection and migrations are left for the runner/integration layer.

use anyhow::Result;
use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead}};
use sha2::{Sha256, Digest};
use rand::RngCore;
use serde::{Serialize, Deserialize};
use tracing::info;

#[derive(Serialize, Deserialize)]
pub struct ProjectFile {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    // Add more fields (timeline, clips, plugins, automation, etc.)
}

fn key_from_passphrase(passphrase: &str) -> [u8; 32] {
    // Derive 256-bit key via SHA-256 (placeholder; replace with proper KDF if needed)
    let mut hasher = Sha256::new();
    hasher.update(passphrase.as_bytes());
    let out = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&out[..32]);
    key
}

pub fn encrypt_project(bytes: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    let key = key_from_passphrase(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key)?;
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);
    let mut out = Vec::from(nonce.as_slice());
    let ct = cipher.encrypt(nonce, bytes)?;
    out.extend_from_slice(&ct);
    Ok(out)
}

pub fn decrypt_project(bytes: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    if bytes.len() < 12 { anyhow::bail!("ciphertext too short"); }
    let (nonce_bytes, ct) = bytes.split_at(12);
    let key = key_from_passphrase(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key)?;
    let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
    let pt = cipher.decrypt(nonce, ct)?;
    Ok(pt)
}

// Convenience helpers to save/load JSON-serialized project files.
pub fn save_project_to_path(project: &ProjectFile, passphrase: &str, path: &std::path::Path) -> Result<()> {
    let json = serde_json::to_vec(project)?;
    let enc = encrypt_project(&json, passphrase)?;
    std::fs::write(path, enc)?;
    info!("Project saved: {}", path.display());
    Ok(())
}

pub fn load_project_from_path(passphrase: &str, path: &std::path::Path) -> Result<ProjectFile> {
    let enc = std::fs::read(path)?;
    let dec = decrypt_project(&enc, passphrase)?;
    let proj: ProjectFile = serde_json::from_slice(&dec)?;
    Ok(proj)
}

// --- SQLx metadata (stubs) ---

// Example schema (to be applied via migrations):
// CREATE TABLE project_meta (
//   id UUID PRIMARY KEY,
//   name TEXT NOT NULL,
//   width INT NOT NULL,
//   height INT NOT NULL,
//   fps REAL NOT NULL,
//   updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
// );

#[allow(unused_variables)]
pub async fn upsert_project_metadata(pool: &sqlx::PgPool, id: uuid::Uuid, proj: &ProjectFile) -> Result<()> {
    // Note: enable sqlx offline feature and run `sqlx prepare` in CI for query checking.
    let _ = sqlx::query!(
        r#"
        INSERT INTO project_meta (id, name, width, height, fps, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ON CONFLICT (id) DO UPDATE
        SET name = EXCLUDED.name,
            width = EXCLUDED.width,
            height = EXCLUDED.height,
            fps = EXCLUDED.fps,
            updated_at = NOW()
        "#,
        id,
        proj.name,
        proj.width as i32,
        proj.height as i32,
        proj.fps
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[allow(unused_variables)]
pub async fn get_project_metadata(pool: &sqlx::PgPool, id: uuid::Uuid) -> Result<Option<ProjectFile>> {
    let rec = sqlx::query!(
        r#"
        SELECT name, width, height, fps
        FROM project_meta
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(r) = rec {
        Ok(Some(ProjectFile {
            name: r.name,
            width: r.width as u32,
            height: r.height as u32,
            fps: r.fps,
        }))
    } else {
        Ok(None)
    }
}