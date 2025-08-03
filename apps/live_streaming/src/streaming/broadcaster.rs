//! Stream broadcaster implementation

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a stream broadcaster
#[derive(Debug, Clone)]
pub struct Broadcaster {
    /// Broadcaster's user ID
    pub user_id: Uuid,
    
    /// Active streams mapped by stream key
    pub active_streams: HashMap<String, Stream>,
}

/// Represents a live stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    /// Unique identifier for the stream
    pub id: Uuid,
    
    /// Stream key used for broadcasting
    pub stream_key: String,
    
    /// Channel ID this stream belongs to
    pub channel_id: Uuid,
    
    /// Title of the stream
    pub title: String,
    
    /// Category/game being streamed
    pub category: String,
    
    /// When the stream started
    pub started_at: DateTime<Utc>,
    
    /// Current viewer count
    pub viewer_count: u32,
    
    /// Stream metadata
    pub metadata: StreamMetadata,
}

/// Metadata for a stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMetadata {
    /// Stream resolution (e.g., "1920x1080")
    pub resolution: String,
    
    /// Stream bitrate in kbps
    pub bitrate: u32,
    
    /// Stream FPS
    pub fps: u32,
    
    /// Whether the stream is using hardware encoding
    pub hardware_encoding: bool,
}

impl Broadcaster {
    /// Create a new broadcaster
    pub fn new() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            active_streams: HashMap::new(),
        }
    }
    
    /// Start a new stream
    pub fn start_stream(
        &mut self,
        channel_id: Uuid,
        stream_key: String,
        title: String,
        category: String,
        metadata: StreamMetadata,
    ) -> Stream {
        let stream = Stream {
            id: Uuid::new_v4(),
            stream_key: stream_key.clone(),
            channel_id,
            title,
            category,
            started_at: Utc::now(),
            viewer_count: 0,
            metadata,
        };
        
        self.active_streams.insert(stream_key, stream.clone());
        stream
    }
    
    /// Stop a stream
    pub fn stop_stream(&mut self, stream_key: &str) -> Option<Stream> {
        self.active_streams.remove(stream_key)
    }
    
    /// Get an active stream by key
    pub fn get_stream(&self, stream_key: &str) -> Option<&Stream> {
        self.active_streams.get(stream_key)
    }
    
    /// Update viewer count for a stream
    pub fn update_viewer_count(&mut self, stream_key: &str, count: u32) -> Option<()> {
        if let Some(stream) = self.active_streams.get_mut(stream_key) {
            stream.viewer_count = count;
            Some(())
        } else {
            None
        }
    }
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}