use std::path::{Path, PathBuf};
use std::fs;
use sqlx::PgPool;
use directories::ProjectDirs;
use cpc_core::asset_browser::{AssetMetadata, AssetType};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
struct AssetRecord {
    id: Uuid,
    name: String,
    path: String,
    size: i64,
    asset_type: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    thumbnail_path: Option<String>,
}

impl AssetRecord {
    fn to_core_metadata(&self) -> AssetMetadata {
        AssetMetadata {
            id: self.id,
            name: self.name.clone(),
            path: PathBuf::from(&self.path),
            size: self.size as u64,
            asset_type: match self.asset_type.as_str() {
                "Image" => AssetType::Image,
                "Video" => AssetType::Video,
                "Audio" => AssetType::Audio,
                "Document" => AssetType::Document,
                "Model3D" => AssetType::Model3D,
                _ => AssetType::Other,
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
            thumbnail_path: self.thumbnail_path.as_ref().map(|p| PathBuf::from(p)),
        }
    }
}

pub struct AssetStorageService {
    db: PgPool,
    assets_dir: PathBuf,
    thumbnails_dir: PathBuf,
}

impl AssetStorageService {
    pub fn new(db: DatabaseConnection) -> Result<Self, Box<dyn std::error::Error>> {
        let project_dirs = ProjectDirs::from("coop", "cpc", "cpc")
            .ok_or("Failed to get project directories")?;
        
        let assets_dir = project_dirs.data_dir().join("assets");
        let thumbnails_dir = assets_dir.join("thumbnails");
        
        // Create directories if they don't exist
        fs::create_dir_all(&assets_dir)?;
        fs::create_dir_all(&thumbnails_dir)?;
        
        Ok(Self {
            db,
            assets_dir,
            thumbnails_dir,
        })
    }

    pub async fn get_assets_in_path(&self, path: &Path) -> Result<Vec<AssetMetadata>, Box<dyn std::error::Error>> {
        let path_str = path.to_string_lossy().to_string();
        
        let assets = sqlx::query_as!(
            AssetRecord,
            r#"
            SELECT id, name, path, size, asset_type, created_at, updated_at, thumbnail_path
            FROM assets
            WHERE path LIKE $1 || '%'
            ORDER BY name ASC
            "#,
            path_str
        )
        .fetch_all(&self.db)
        .await?;
            
        Ok(assets.into_iter().map(|a| a.to_core_metadata()).collect())
    }

    pub async fn import_asset(
        &self,
        source_path: &Path,
        asset_type: AssetType,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let asset_id = Uuid::new_v4();
        let file_name = source_path.file_name()
            .ok_or("Invalid file name")?
            .to_string_lossy()
            .to_string();
        
        let dest_path = self.assets_dir.join(&file_name);
        
        // Check for duplicate
        if dest_path.exists() {
            return Err("Asset already exists".into());
        }
        
        // Copy file to assets directory
        fs::copy(source_path, &dest_path)?;
        
        let metadata = fs::metadata(&dest_path)?;
        let now = Utc::now();
        
        // Create database entry
        let asset_type_str = match asset_type {
            AssetType::Image => "Image",
            AssetType::Video => "Video",
            AssetType::Audio => "Audio",
            AssetType::Document => "Document",
            AssetType::Model3D => "Model3D",
            AssetType::Other => "Other",
        };
        
        sqlx::query!(
            r#"
            INSERT INTO assets (id, name, path, size, asset_type, created_at, updated_at, thumbnail_path)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            asset_id,
            file_name,
            dest_path.to_string_lossy().to_string(),
            metadata.len() as i64,
            asset_type_str,
            now,
            now,
            None::<String>
        )
        .execute(&self.db)
        .await?;
        
        Ok(asset_id)
    }

    pub async fn delete_asset(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let asset = sqlx::query_as!(
            AssetRecord,
            "SELECT * FROM assets WHERE id = $1",
            id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or("Asset not found")?;
            
        // Delete file from filesystem
        let file_path = PathBuf::from(&asset.path);
        if file_path.exists() {
            fs::remove_file(&file_path)?;
        }
        
        // Delete thumbnail if exists
        if let Some(thumb_path) = asset.thumbnail_path {
            let thumb_path = PathBuf::from(thumb_path);
            if thumb_path.exists() {
                fs::remove_file(&thumb_path)?;
            }
        }
        
        // Delete from database
        sqlx::query!("DELETE FROM assets WHERE id = $1", id)
            .execute(&self.db)
            .await?;
        
        Ok(())
    }

    pub async fn get_asset_metadata(&self, id: Uuid) -> Result<Option<AssetMetadata>, Box<dyn std::error::Error>> {
        let asset = sqlx::query_as!(
            AssetRecord,
            "SELECT * FROM assets WHERE id = $1",
            id
        )
        .fetch_optional(&self.db)
        .await?;
            
        Ok(asset.map(|a| a.to_core_metadata()))
    }

    pub async fn update_asset_thumbnail(
        &self,
        id: Uuid,
        thumbnail_path: Option<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let thumbnail_path_str = thumbnail_path.map(|p| p.to_string_lossy().to_string());
        
        sqlx::query!(
            "UPDATE assets SET thumbnail_path = $1, updated_at = $2 WHERE id = $3",
            thumbnail_path_str,
            Utc::now(),
            id
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }

    pub fn get_assets_dir(&self) -> &Path {
        &self.assets_dir
    }

    pub fn get_thumbnails_dir(&self) -> &Path {
        &self.thumbnails_dir
    }

    pub async fn get_asset(&self, id: &Uuid) -> Result<Option<AssetMetadata>, Box<dyn std::error::Error>> {
        let record = sqlx::query_as!(
            AssetRecord,
            r#"
            SELECT id, name, path, size, asset_type, created_at, updated_at, thumbnail_path
            FROM assets
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(record.map(|r| r.to_core_metadata()))
    }

    pub async fn list_assets(&self, limit: i64, offset: i64) -> Result<Vec<AssetMetadata>, Box<dyn std::error::Error>> {
        let records = sqlx::query_as!(
            AssetRecord,
            r#"
            SELECT id, name, path, size, asset_type, created_at, updated_at, thumbnail_path
            FROM assets
            ORDER BY created_at DESC
            LIMIT $1
            OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        Ok(records.into_iter().map(|r| r.to_core_metadata()).collect())
    }

    pub async fn search_assets(&self, query: &str, limit: i64, offset: i64) -> Result<Vec<AssetMetadata>, Box<dyn std::error::Error>> {
        let records = sqlx::query_as!(
            AssetRecord,
            r#"
            SELECT id, name, path, size, asset_type, created_at, updated_at, thumbnail_path
            FROM assets
            WHERE name ILIKE $1 OR path ILIKE $1
            ORDER BY created_at DESC
            LIMIT $2
            OFFSET $3
            "#,
            format!("%{}%", query)
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        Ok(records.into_iter().map(|r| r.to_core_metadata()).collect())
    }
}
