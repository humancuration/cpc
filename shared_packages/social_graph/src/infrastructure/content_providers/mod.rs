mod social_post;
mod video;

use crate::domain::model::ContentProvider;
use std::sync::Arc;

pub use social_post::SocialPostProvider;
pub use video::VideoProvider;

pub fn register_providers(social_service: &mut crate::application::social_service::SocialService<impl crate::domain::repository::RelationshipRepository>) {
    social_service.register_content_provider(Arc::new(SocialPostProvider));
    social_service.register_content_provider(Arc::new(VideoProvider));
}