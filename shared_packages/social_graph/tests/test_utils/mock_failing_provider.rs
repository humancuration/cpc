//! Mock provider that can be configured to fail during serialization/deserialization

use async_trait::async_trait;
use social_graph::domain::model::{
    ContentItem, ContentProvider, ContentProviderError, ContentType, FeedFilter
};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct FailingMockProvider {
    pub fail_serialize: bool,
    pub fail_deserialize: bool,
    pub state: Mutex<String>,
}

impl FailingMockProvider {
    pub fn new() -> Self {
        Self {
            fail_serialize: false,
            fail_deserialize: false,
            state: Mutex::new("initial_state".to_string()),
        }
    }
}

#[async_trait]
impl ContentProvider for FailingMockProvider {
    fn content_type(&self) -> ContentType {
        ContentType::SocialPost
    }

    async fn get_content(
        &self,
        _user_id: uuid::Uuid,
        _after: Option<chrono::DateTime<chrono::Utc>>,
        _limit: usize,
        _filters: &[FeedFilter],
    ) -> Result<Vec<ContentItem>, ContentProviderError> {
        Ok(vec![])
    }

    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.fail_serialize {
            Err(Box::new(ContentProviderError::StateSerializationError))
        } else {
            Ok(self.state.lock().unwrap().as_bytes().to_vec())
        }
    }

    fn deserialize_state(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if self.fail_deserialize {
            Err(Box::new(ContentProviderError::StateDeserializationError))
        } else {
            *self.state.lock().unwrap() = String::from_utf8_lossy(data).to_string();
            Ok(())
        }
    }
}