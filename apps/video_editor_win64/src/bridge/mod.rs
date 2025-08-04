use tracing::{instrument, info};
use crate::engine::composition::{Composition, VideoTrack};

/// Timeline-ECS bridge stub plugin (no Bevy dep yet).
/// Acts as the adapter between UI timeline state and engine Composition.
pub struct TimelineBridge {
    pub composition: Composition,
}

impl Default for TimelineBridge {
    fn default() -> Self {
        Self { composition: Composition::default() }
    }
}

impl TimelineBridge {
    pub fn new() -> Self { Self::default() }

    /// Add a new enabled track to the composition and return its id.
    #[instrument(name = "bridge.add_track", skip_all)]
    pub fn add_track(&mut self) -> u32 {
        let id = (self.composition.tracks.len() as u32) + 1;
        self.composition.tracks.push(VideoTrack { id, enabled: true, ..Default::default() });
        info!("TimelineBridge added track {}", id);
        id
    }

    /// Update basic project settings.
    #[instrument(name = "bridge.update_settings", skip_all)]
    pub fn update_settings(&mut self, width: u32, height: u32, fps: f32) {
        self.composition.width = width;
        self.composition.height = height;
        self.composition.fps = fps;
    }

    /// Simulate a seek by returning contributing track ids at time.
    #[instrument(name = "bridge.seek", skip_all, fields(t_ms = t_ms))]
    pub fn seek_active_tracks(&self, t_ms: u64) -> Vec<u32> {
        crate::engine::composition::render_system(&self.composition, t_ms)
    }
}