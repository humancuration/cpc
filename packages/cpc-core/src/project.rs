use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::scene::SceneData;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData {
    pub metadata: ProjectMetadata,
    pub scene: SceneData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectMetadata {
    pub project_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub version: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}