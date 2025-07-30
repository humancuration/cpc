use crate::domain::models::Project;
use crate::domain::ports::persistence::{PersistencePort, PersistenceError};
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct SqlRepository {
    pool: SqlitePool,
}

impl SqlRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Create tables if they don't exist
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data BLOB NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl PersistencePort for SqlRepository {
    async fn save_project(&self, project: &Project) -> Result<(), PersistenceError> {
        let data = serde_json::to_vec(project)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
        
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO projects (id, name, data, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(project.id.to_string())
        .bind(&project.name)
        .bind(data)
        .bind(project.created_at)
        .bind(project.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn load_project(&self, id: Uuid) -> Result<Option<Project>, PersistenceError> {
        let row = sqlx::query(
            r#"
            SELECT data FROM projects WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e.to_string()))?;
        
        if let Some(row) = row {
            let data: Vec<u8> = row.get(0);
            let project: Project = serde_json::from_slice(&data)
                .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
            Ok(Some(project))
        } else {
            Ok(None)
        }
    }
    
    async fn delete_project(&self, id: Uuid) -> Result<(), PersistenceError> {
        sqlx::query(
            r#"
            DELETE FROM projects WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn list_projects(&self) -> Result<Vec<Project>, PersistenceError> {
        let rows = sqlx::query(
            r#"
            SELECT data FROM projects ORDER BY updated_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| PersistenceError::DatabaseError(e.to_string()))?;
        
        let mut projects = Vec::new();
        for row in rows {
            let data: Vec<u8> = row.get(0);
            let project: Project = serde_json::from_slice(&data)
                .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;
            projects.push(project);
        }
        
        Ok(projects)
    }
}