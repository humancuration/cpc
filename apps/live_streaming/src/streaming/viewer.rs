//! Stream viewer implementation

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a stream viewer
#[derive(Debug, Clone)]
pub struct Viewer {
    /// Viewer's user ID
    pub user_id: Uuid,
    
    /// Streams the viewer is currently watching
    pub watching_streams: HashMap<Uuid, WatchingStream>,
}

/// Represents a stream being watched
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchingStream {
    /// Stream ID
    pub stream_id: Uuid,
    
    /// When the viewer started watching
    pub started_watching_at: DateTime<Utc>,
    
    /// Viewer's current position in the stream (for VOD)
    pub position: Option<u64>,
    
    /// Whether the viewer has the stream muted
    pub is_muted: bool,
    
    /// Viewer's volume level (0-100)
    pub volume: u8,
}

impl Viewer {
    /// Create a new viewer
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            watching_streams: HashMap::new(),
        }
    }
    
    /// Start watching a stream
    pub fn start_watching(&mut self, stream_id: Uuid) {
        let watching = WatchingStream {
            stream_id,
            started_watching_at: Utc::now(),
            position: None,
            is_muted: false,
            volume: 100,
        };
        
        self.watching_streams.insert(stream_id, watching);
    }
    
    /// Stop watching a stream
    pub fn stop_watching(&mut self, stream_id: &Uuid) -> Option<WatchingStream> {
        self.watching_streams.remove(stream_id)
    }
    
    /// Get information about a stream the viewer is watching
    pub fn get_watching_stream(&self, stream_id: &Uuid) -> Option<&WatchingStream> {
        self.watching_streams.get(stream_id)
    }
    
    /// Update viewer's position in a stream (for VOD)
    pub fn update_position(&mut self, stream_id: &Uuid, position: u64) -> Option<()> {
        if let Some(watching) = self.watching_streams.get_mut(stream_id) {
            watching.position = Some(position);
            Some(())
        } else {
            None
        }
    }
    
    /// Toggle mute for a stream
    pub fn toggle_mute(&mut self, stream_id: &Uuid) -> Option<()> {
        if let Some(watching) = self.watching_streams.get_mut(stream_id) {
            watching.is_muted = !watching.is_muted;
            Some(())
        } else {
            None
        }
    }
    
    /// Set volume for a stream
    pub fn set_volume(&mut self, stream_id: &Uuid, volume: u8) -> Option<()> {
        if let Some(watching) = self.watching_streams.get_mut(stream_id) {
            watching.volume = volume.min(100);
            Some(())
        } else {
            None
        }
    }
}