//! File service for versioned files backed by object storage (local FS stub)
//!
//! Responsibilities:
//! - Create new file versions (store bytes in object storage, persist metadata in Postgres repo)
//! - Get a specific version's content
//!
//! For now, object storage is implemented via a simple local-filesystem adapter trait.

use async_trait::async_trait;
use chrono::Utc;
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

use crate::domain::models::FileVersion;

#[derive(thiserror::Error, Debug)]
pub enum FileServiceError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("repository error: {0}")]
    Repository(String),
    #[error("storage error: {0}")]
    Storage(String),
    #[error("validation error: {0}")]
    Validation(String),
}

/// Repository for file metadata (versions)
#[async_trait]
pub trait FileRepository: Send + Sync {
    async fn get_latest_version(&self, file_id: Uuid) -> Result<Option<i32>, String>;
    async fn create_version(&self, version: &FileVersion) -> Result<(), String>;
    async fn get_version_meta(&self, file_id: Uuid, version: i32) -> Result<FileVersion, String>;
}

/// Object storage abstraction (local FS for now)
#[async_trait]
pub trait ObjectStorage: Send + Sync {
    async fn put(&self, key: &str, bytes: &[u8]) -> Result<(), String>;
    async fn get(&self, key: &str) -> Result<Vec<u8>, String>;
}

/// Simple local filesystem storage: stores under a base directory
pub struct LocalFsStorage {
    base: PathBuf,
}

impl LocalFsStorage {
    pub fn new<P: Into<PathBuf>>(base: P) -> Self {
        Self { base: base.into() }
    }

    fn path_for(&self, key: &str) -> PathBuf {
        self.base.join(key)
    }
}

#[async_trait]
impl ObjectStorage for LocalFsStorage {
    async fn put(&self, key: &str, bytes: &[u8]) -> Result<(), String> {
        let path = self.path_for(key);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        tokio::fs::write(&path, bytes).await.map_err(|e| e.to_string())
    }

    async fn get(&self, key: &str) -> Result<Vec<u8>, String> {
        let path = self.path_for(key);
        tokio::fs::read(&path).await.map_err(|e| e.to_string())
    }
}

#[async_trait]
pub trait FileService: Send + Sync {
    async fn create_version(
        &self,
        file_id: Uuid,
        content: Vec<u8>,
        user_id: Uuid,
    ) -> Result<FileVersion, FileServiceError>;

    async fn get_version(
        &self,
        file_id: Uuid,
        version: i32,
    ) -> Result<(FileVersion, Vec<u8>), FileServiceError>;
}

pub struct FileServiceImpl<R: FileRepository, S: ObjectStorage> {
    repo: Arc<R>,
    storage: Arc<S>,
}

impl<R: FileRepository, S: ObjectStorage> FileServiceImpl<R, S> {
    pub fn new(repo: Arc<R>, storage: Arc<S>) -> Self {
        Self { repo, storage }
    }

    fn storage_key(file_id: Uuid, version: i32) -> String {
        format!("{}/{}", file_id, version)
    }
}

#[async_trait]
impl<R: FileRepository, S: ObjectStorage> FileService for FileServiceImpl<R, S> {
    async fn create_version(
        &self,
        file_id: Uuid,
        content: Vec<u8>,
        user_id: Uuid,
    ) -> Result<FileVersion, FileServiceError> {
        if content.is_empty() {
            return Err(FileServiceError::Validation("content cannot be empty".into()));
        }

        // determine next version
        let next_version = match self.repo.get_latest_version(file_id).await {
            Ok(Some(latest)) => latest + 1,
            Ok(None) => 1,
            Err(e) => return Err(FileServiceError::Repository(e)),
        };

        // store in object storage
        let key = Self::storage_key(file_id, next_version);
        self.storage
            .put(&key, &content)
            .await
            .map_err(FileServiceError::Storage)?;

        // persist metadata
        let meta = FileVersion {
            id: Uuid::new_v4(),
            file_id,
            version: next_version,
            created_by: user_id,
            created_at: Utc::now(),
        };
        self.repo
            .create_version(&meta)
            .await
            .map_err(FileServiceError::Repository)?;

        Ok(meta)
    }

    async fn get_version(
        &self,
        file_id: Uuid,
        version: i32,
    ) -> Result<(FileVersion, Vec<u8>), FileServiceError> {
        let meta = self
            .repo
            .get_version_meta(file_id, version)
            .await
            .map_err(|e| {
                if e.to_lowercase().contains("not found") {
                    FileServiceError::NotFound(format!("{}#{}", file_id, version))
                } else {
                    FileServiceError::Repository(e)
                }
            })?;
        let key = Self::storage_key(file_id, version);
        let bytes = self.storage.get(&key).await.map_err(|e| {
            if e.to_lowercase().contains("not found") {
                FileServiceError::NotFound(key)
            } else {
                FileServiceError::Storage(e)
            }
        })?;
        Ok((meta, bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct InMemFileRepo {
        latest: HashMap<Uuid, i32>,
        metas: HashMap<(Uuid, i32), FileVersion>,
    }

    #[async_trait]
    impl FileRepository for InMemFileRepo {
        async fn get_latest_version(&self, file_id: Uuid) -> Result<Option<i32>, String> {
            Ok(self.latest.get(&file_id).cloned())
        }
        async fn create_version(&self, version: &FileVersion) -> Result<(), String> {
            Ok(())
        }
        async fn get_version_meta(&self, file_id: Uuid, version: i32) -> Result<FileVersion, String> {
            self.metas
                .get(&(file_id, version))
                .cloned()
                .ok_or("not found".into())
        }
    }

    #[tokio::test]
    async fn create_and_get_version_roundtrip() {
        // prepare fake repo and fs storage under temp dir
        let tmp = tempfile::tempdir().unwrap();
        let storage = Arc::new(LocalFsStorage::new(tmp.path()));
        let repo = Arc::new(InMemFileRepo {
            latest: HashMap::new(),
            metas: HashMap::new(),
        });

        let svc = FileServiceImpl::new(repo.clone(), storage.clone());

        let file_id = Uuid::new_v4();
        let user = Uuid::new_v4();
        // emulate repo state change that create_version would normally do
        // since our InMemFileRepo is simplistic we manually insert after call
        let meta = svc
            .create_version(file_id, b"hello".to_vec(), user)
            .await
            .unwrap();

        // manually reflect metadata into repo to allow get_version to succeed
        repo.latest.insert(file_id, meta.version);
        repo.metas.insert((file_id, meta.version), meta.clone());

        // read back
        let (_m, bytes) = svc.get_version(file_id, meta.version).await.unwrap();
        assert_eq!(bytes, b"hello".to_vec());
    }
}