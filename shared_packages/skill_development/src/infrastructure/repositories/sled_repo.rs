use sled::Db;
use uuid::Uuid;

use crate::domain::*;

pub struct SledRepository {
    db: Db,
}

impl SledRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn save_skill_progress(
        &self,
        progress: &SkillProgress,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("skill_progress:{}", progress.id);
        let value = serde_json::to_vec(progress)?;
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    pub async fn get_skill_progress(
        &self,
        progress_id: Uuid,
    ) -> Result<SkillProgress, Box<dyn std::error::Error>> {
        let key = format!("skill_progress:{}", progress_id);
        let value = self.db.get(key.as_bytes())?
            .ok_or("Skill progress not found")?;
        let progress: SkillProgress = serde_json::from_slice(&value)?;
        Ok(progress)
    }

    pub async fn get_user_skill_progress(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<SkillProgress>, Box<dyn std::error::Error>> {
        let mut progress_list = Vec::new();
        let prefix = format!("skill_progress:");
        
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_, value) = item?;
            let progress: SkillProgress = serde_json::from_slice(&value)?;
            if progress.user_id == user_id {
                progress_list.push(progress);
            }
        }
        
        Ok(progress_list)
    }

    pub async fn save_learning_path(
        &self,
        path: &LearningPath,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("learning_path:{}", path.id);
        let value = serde_json::to_vec(path)?;
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    pub async fn get_learning_path(
        &self,
        path_id: Uuid,
    ) -> Result<Option<LearningPath>, Box<dyn std::error::Error>> {
        let key = format!("learning_path:{}", path_id);
        let value = self.db.get(key.as_bytes())?;
        
        match value {
            Some(v) => {
                let path: LearningPath = serde_json::from_slice(&v)?;
                Ok(Some(path))
            }
            None => Ok(None),
        }
    }

    pub async fn save_certification(
        &self,
        certification: &Certification,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("certification:{}", certification.id);
        let value = serde_json::to_vec(certification)?;
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    pub async fn get_user_certifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Certification>, Box<dyn std::error::Error>> {
        let mut certifications = Vec::new();
        let prefix = format!("certification:");
        
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_, value) = item?;
            let cert: Certification = serde_json::from_slice(&value)?;
            if cert.user_id == user_id {
                certifications.push(cert);
            }
        }
        
        Ok(certifications)
    }

    pub async fn get_all_skill_progress(&self) -> Result<Vec<SkillProgress>, Box<dyn std::error::Error>> {
        let mut progress_list = Vec::new();
        let prefix = format!("skill_progress:");
        
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_, value) = item?;
            let progress: SkillProgress = serde_json::from_slice(&value)?;
            progress_list.push(progress);
        }
        
        Ok(progress_list)
    }

    pub async fn get_all_learning_paths(&self) -> Result<Vec<LearningPath>, Box<dyn std::error::Error>> {
        let mut paths = Vec::new();
        let prefix = format!("learning_path:");
        
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_, value) = item?;
            let path: LearningPath = serde_json::from_slice(&value)?;
            paths.push(path);
        }
        
        Ok(paths)
    }

    pub async fn get_all_certifications(&self) -> Result<Vec<Certification>, Box<dyn std::error::Error>> {
        let mut certifications = Vec::new();
        let prefix = format!("certification:");
        
        for item in self.db.scan_prefix(prefix.as_bytes()) {
            let (_, value) = item?;
            let cert: Certification = serde_json::from_slice(&value)?;
            certifications.push(cert);
        }
        
        Ok(certifications)
    }
}