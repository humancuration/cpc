//! Document service for collaborative documents using CRDT (Automerge)
//!
//! Responsibilities:
//! - Create documents
//! - Retrieve documents
//! - Apply CRDT operations with last-write-wins conflict resolution
//! - Persist via repository
//! - Publish DocumentUpdated events

use async_trait::async_trait;
use automerge::{AutoCommit, transaction::Transactable};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::CollaborativeDocument;

/// Errors for the document service
#[derive(thiserror::Error, Debug)]
pub enum DocumentServiceError {
    #[error("document not found: {0}")]
    NotFound(Uuid),
    #[error("repository error: {0}")]
    Repository(String),
    #[error("crdt operation error: {0}")]
    Crdt(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("event publish error: {0}")]
    Event(String),
}

/// Repository trait expected from infrastructure for documents.
/// We keep this minimal so app layer compiles while infra is being implemented.
#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn create(
        &self,
        meta: &CollaborativeDocument,
        crdt_bytes: Vec<u8>,
    ) -> Result<(), String>;

    async fn get(
        &self,
        id: Uuid,
    ) -> Result<(CollaborativeDocument, Vec<u8>), String>;

    async fn update_state(
        &self,
        id: Uuid,
        updated_at: chrono::DateTime<Utc>,
        crdt_bytes: Vec<u8>,
    ) -> Result<(), String>;
}

/// Event publisher trait used by the service
#[async_trait]
pub trait CollaborationEventPublisher: Send + Sync {
    async fn publish_document_updated(
        &self,
        event: DocumentUpdated,
    ) -> Result<(), String>;
}

/// Event emitted after document changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUpdated {
    pub document_id: Uuid,
    pub updated_by: Uuid,
    pub updated_at: chrono::DateTime<Utc>,
    /// Optionally, a summary or hash of the new state
    pub state_hash: Option<String>,
}

/// Operation descriptor applied to the CRDT.
/// For now we model a simple "set text" path/value; can be extended later.
/// We keep it generic so tests can exercise it without real OT protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentOperation {
    /// Set a string value at a top-level key
    SetString { key: String, value: String, lamport_ts: u64 },
}

#[async_trait]
pub trait DocumentService: Send + Sync {
    async fn create_document(
        &self,
        title: String,
        user_id: Uuid,
    ) -> Result<CollaborativeDocument, DocumentServiceError>;

    async fn get_document(
        &self,
        document_id: Uuid,
    ) -> Result<(CollaborativeDocument, AutoCommit), DocumentServiceError>;

    async fn apply_operation(
        &self,
        document_id: Uuid,
        operation: DocumentOperation,
        user_id: Uuid,
    ) -> Result<(CollaborativeDocument, AutoCommit), DocumentServiceError>;
}

/// Implementation using Postgres repository and event publisher
pub struct DocumentServiceImpl<R: DocumentRepository, P: CollaborationEventPublisher> {
    repo: Arc<R>,
    publisher: Arc<P>,
}

impl<R: DocumentRepository, P: CollaborationEventPublisher> DocumentServiceImpl<R, P> {
    pub fn new(repo: Arc<R>, publisher: Arc<P>) -> Self {
        Self { repo, publisher }
    }

    fn encode(doc: &AutoCommit) -> Result<Vec<u8>, DocumentServiceError> {
        doc.save().map_err(|e| DocumentServiceError::Serialization(e.to_string()))
    }

    fn decode(bytes: &[u8]) -> Result<AutoCommit, DocumentServiceError> {
        AutoCommit::load(bytes).map_err(|e| DocumentServiceError::Serialization(e.to_string()))
    }

    // Very coarse "last write wins": compare provided logical timestamp (lamport_ts) to a stored one.
    // Since we don't persist lamport here yet, we approximate by always applying and letting Automerge converge.
    // If we needed strict LWW, we'd store per-key timestamps; for now, we rely on Automerge's conflict handling
    // and simply overwrite on SetString to emulate LWW semantics.
    fn apply_op(doc: &mut AutoCommit, op: &DocumentOperation) -> Result<(), DocumentServiceError> {
        match op {
            DocumentOperation::SetString { key, value, .. } => {
                doc.put(automerge::ObjId::Root, key, value.as_str())
                    .map_err(|e| DocumentServiceError::Crdt(e.to_string()))?;
                Ok(())
            }
        }
    }
}

#[async_trait]
impl<R: DocumentRepository, P: CollaborationEventPublisher> DocumentService for DocumentServiceImpl<R, P> {
    async fn create_document(
        &self,
        title: String,
        user_id: Uuid,
    ) -> Result<CollaborativeDocument, DocumentServiceError> {
        let now = Utc::now();
        let meta = CollaborativeDocument {
            id: Uuid::new_v4(),
            title,
            created_by: user_id,
            created_at: now,
            updated_at: now,
        };

        let mut crdt = AutoCommit::new();
        // initialize with title as a field for convenience
        crdt.put(automerge::ObjId::Root, "title", meta.title.as_str())
            .map_err(|e| DocumentServiceError::Crdt(e.to_string()))?;

        let bytes = Self::encode(&crdt)?;
        self.repo
            .create(&meta, bytes)
            .await
            .map_err(DocumentServiceError::Repository)?;

        Ok(meta)
    }

    async fn get_document(
        &self,
        document_id: Uuid,
    ) -> Result<(CollaborativeDocument, AutoCommit), DocumentServiceError> {
        let (meta, bytes) = self
            .repo
            .get(document_id)
            .await
            .map_err(|e| {
                if e.to_lowercase().contains("not found") {
                    DocumentServiceError::NotFound(document_id)
                } else {
                    DocumentServiceError::Repository(e)
                }
            })?;
        let crdt = Self::decode(&bytes)?;
        Ok((meta, crdt))
    }

    async fn apply_operation(
        &self,
        document_id: Uuid,
        operation: DocumentOperation,
        user_id: Uuid,
    ) -> Result<(CollaborativeDocument, AutoCommit), DocumentServiceError> {
        let (mut meta, bytes) = self
            .repo
            .get(document_id)
            .await
            .map_err(|e| {
                if e.to_lowercase().contains("not found") {
                    DocumentServiceError::NotFound(document_id)
                } else {
                    DocumentServiceError::Repository(e)
                }
            })?;

        let mut crdt = Self::decode(&bytes)?;
        // Apply op with LWW-ish semantics (see apply_op)
        Self::apply_op(&mut crdt, &operation)?;

        // Persist
        meta.updated_at = Utc::now();
        let new_bytes = Self::encode(&crdt)?;
        self.repo
            .update_state(document_id, meta.updated_at, new_bytes)
            .await
            .map_err(DocumentServiceError::Repository)?;

        // Publish event
        let event = DocumentUpdated {
            document_id,
            updated_by: user_id,
            updated_at: meta.updated_at,
            state_hash: Some(format!("{:x}", xxhash_rust::xxh3::xxh3_128(bytes_of(&crdt)))) ,
        };
        self.publisher
            .publish_document_updated(event)
            .await
            .map_err(DocumentServiceError::Event)?;

        Ok((meta, crdt))
    }
}

/// Helper to generate a quick hash over CRDT save bytes
fn bytes_of(crdt: &AutoCommit) -> Vec<u8> {
    crdt.save().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct InMemoryDocRepo {
        meta: HashMap<Uuid, CollaborativeDocument>,
        state: HashMap<Uuid, Vec<u8>>,
    }

    #[async_trait]
    impl DocumentRepository for InMemoryDocRepo {
        async fn create(&self, _meta: &CollaborativeDocument, _crdt_bytes: Vec<u8>) -> Result<(), String> {
            // Not used in tests (we use mutable instance below)
            Err("immutable stub".into())
        }
        async fn get(&self, id: Uuid) -> Result<(CollaborativeDocument, Vec<u8>), String> {
            let meta = self.meta.get(&id).cloned().ok_or("not found")?;
            let bytes = self.state.get(&id).cloned().ok_or("not found")?;
            Ok((meta, bytes))
        }
        async fn update_state(&self, _id: Uuid, _updated_at: chrono::DateTime<Utc>, _crdt_bytes: Vec<u8>) -> Result<(), String> {
            Err("immutable stub".into())
        }
    }

    struct InMemoryDocRepoMut {
        meta: HashMap<Uuid, CollaborativeDocument>,
        state: HashMap<Uuid, Vec<u8>>,
    }
    #[async_trait]
    impl DocumentRepository for InMemoryDocRepoMut {
        async fn create(&self, _meta: &CollaborativeDocument, _crdt_bytes: Vec<u8>) -> Result<(), String> {
            Ok(()) // unused
        }
        async fn get(&self, id: Uuid) -> Result<(CollaborativeDocument, Vec<u8>), String> {
            let meta = self.meta.get(&id).cloned().ok_or("not found")?;
            let bytes = self.state.get(&id).cloned().ok_or("not found")?;
            Ok((meta, bytes))
        }
        async fn update_state(&self, id: Uuid, updated_at: chrono::DateTime<Utc>, crdt_bytes: Vec<u8>) -> Result<(), String> {
            if let Some(m) = self.meta.get_mut(&id) {
                m.updated_at = updated_at;
            }
            self.state.insert(id, crdt_bytes);
            Ok(())
        }
    }

    struct NoopPublisher;
    #[async_trait]
    impl CollaborationEventPublisher for NoopPublisher {
        async fn publish_document_updated(&self, _event: DocumentUpdated) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn create_and_apply_setstring_operation() {
        let mut_repo = InMemoryDocRepoMut {
            meta: HashMap::new(),
            state: HashMap::new(),
        };

        // we need interior mutability for test setup
        let repo = Arc::new(mut_repo);
        let publisher = Arc::new(NoopPublisher);
        let svc = DocumentServiceImpl::new(repo.clone(), publisher);

        // create
        let user = Uuid::new_v4();
        let meta = CollaborativeDocument {
            id: Uuid::new_v4(),
            title: "Doc".into(),
            created_by: user,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let mut crdt = AutoCommit::new();
        crdt.put(automerge::ObjId::Root, "title", "Doc").unwrap();
        let bytes = crdt.save().unwrap();

        // seed repo
        repo.meta.insert(meta.id, meta.clone());
        repo.state.insert(meta.id, bytes);

        // apply op
        let op = DocumentOperation::SetString { key: "content".into(), value: "hello".into(), lamport_ts: 1 };
        let (_meta2, crdt2) = svc.apply_operation(meta.id, op, user).await.unwrap();

        // verify
        let text = crdt2.get(automerge::ObjId::Root, "content").unwrap().unwrap().0.to_string();
        assert!(text.contains("hello"));
    }
}