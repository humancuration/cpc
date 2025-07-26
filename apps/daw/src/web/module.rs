use crate::domain::services::{ProjectManager, AudioEngine, Mixer};
use crate::domain::ports::{PersistencePort, AudioProcessingPort, CollaborationPort};
use std::sync::Arc;

/// Web module for DAW GraphQL API
pub struct DawModule {
    pub project_manager: Arc<ProjectManager>,
    pub audio_engine: Arc<AudioEngine>,
    pub mixer: Arc<Mixer>,
}

impl DawModule {
    pub fn new(
        persistence: Box<dyn PersistencePort>,
        audio_processing: Box<dyn AudioProcessingPort>,
        collaboration: Box<dyn CollaborationPort>,
        sample_rate: u32,
        buffer_size: usize,
    ) -> Self {
        let project_manager = Arc::new(ProjectManager::new(persistence));
        let audio_engine = Arc::new(AudioEngine::new(sample_rate, buffer_size, audio_processing));
        let mixer = Arc::new(Mixer::new(sample_rate, collaboration));
        
        Self {
            project_manager,
            audio_engine,
            mixer,
        }
    }
}

/// Module exports for GraphQL integration
pub mod graphql {
    pub use super::super::web::graphql::schema::*;
    pub use super::super::web::graphql::queries::*;
    pub use super::super::web::graphql::mutations::*;
    pub use super::super::web::graphql::subscriptions::*;
}