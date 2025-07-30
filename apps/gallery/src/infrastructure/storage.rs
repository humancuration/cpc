use async_trait::async_trait;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[async_trait]
pub trait FileStorage: Send + Sync {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), String>;
    async fn load(&self, path: &str) -> Result<Vec<u8>, String>;
    async fn delete(&self, path: &str) -> Result<(), String>;
    async fn exists(&self, path: &str) -> Result<bool, String>;
    async fn get_temp_path(&self, path: &str) -> Result<String, String>;
    async fn move_to_storage(&self, temp_path: &str, dest_path: &str) -> Result<(), String>;
}

pub struct LocalFileStorage;

#[async_trait]
impl FileStorage for LocalFileStorage {
    async fn save(&self, path: &str, data: &[u8]) -> Result<(), String> {
        // Ensure the directory exists
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create directories: {}", e))?;
        }
        
        // Write the file
        let mut file = fs::File::create(path).await
            .map_err(|e| format!("Failed to create file: {}", e))?;
        
        file.write_all(data).await
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }

    async fn load(&self, path: &str) -> Result<Vec<u8>, String> {
        fs::read(path).await
            .map_err(|e| format!("Failed to read file: {}", e))
    }

    async fn delete(&self, path: &str) -> Result<(), String> {
        fs::remove_file(path).await
            .map_err(|e| format!("Failed to delete file: {}", e))
    }

    async fn exists(&self, path: &str) -> Result<bool, String> {
        Ok(fs::metadata(path).await.is_ok())
    }
    
    async fn get_temp_path(&self, path: &str) -> Result<String, String> {
        // Create a temporary path based on the original path
        let temp_path = format!("{}.tmp", path);
        Ok(temp_path)
    }
    
    async fn move_to_storage(&self, temp_path: &str, dest_path: &str) -> Result<(), String> {
        // Ensure the destination directory exists
        if let Some(parent) = std::path::Path::new(dest_path).parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create directories: {}", e))?;
        }
        
        // Move the file from temp path to destination
        fs::rename(temp_path, dest_path).await
            .map_err(|e| format!("Failed to move file: {}", e))?;
        
        Ok(())
    }
}

/// Cloud storage implementation (e.g., S3-compatible)
pub struct CloudStorage {
    // In a real implementation, this would contain client configuration
    // client: S3Client,
    // bucket: String,
}

#[async_trait]
impl FileStorage for CloudStorage {
    async fn save(&self, _path: &str, _data: &[u8]) -> Result<(), String> {
        // In a real implementation, this would save to cloud storage
        Ok(())
    }

    async fn load(&self, _path: &str) -> Result<Vec<u8>, String> {
        // In a real implementation, this would load from cloud storage
        Ok(vec![])
    }

    async fn delete(&self, _path: &str) -> Result<(), String> {
        // In a real implementation, this would delete from cloud storage
        Ok(())
    }

    async fn exists(&self, _path: &str) -> Result<bool, String> {
        // In a real implementation, this would check existence in cloud storage
        Ok(false)
    }
    
    async fn get_temp_path(&self, path: &str) -> Result<String, String> {
        // In a real implementation, this would create a temporary path for cloud storage
        let temp_path = format!("{}.tmp", path);
        Ok(temp_path)
    }
    
    async fn move_to_storage(&self, _temp_path: &str, _dest_path: &str) -> Result<(), String> {
        // In a real implementation, this would move from temp to permanent cloud storage
        Ok(())
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_local_file_storage() {
        let storage = LocalFileStorage;
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let file_path_str = file_path.to_str().unwrap();
        
        let data = b"Hello, world!";
        
        // Test save
        assert!(storage.save(file_path_str, data).await.is_ok());
        
        // Test exists
        assert!(storage.exists(file_path_str).await.unwrap());
        
        // Test load
        let loaded_data = storage.load(file_path_str).await.unwrap();
        assert_eq!(loaded_data, data);
        
        // Test delete
        assert!(storage.delete(file_path_str).await.is_ok());
        
        // Test exists after delete
        assert!(!storage.exists(file_path_str).await.unwrap());
    }
}