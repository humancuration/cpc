use crate::engine::composition::{Composition, composition_system};
use tracing::instrument;

/// Public re-exported system entry for rendering
#[instrument(name = "engine.systems.render", skip_all, fields(t_ms = t_ms))]
pub fn render_system_tracks(comp: &Composition, t_ms: u64) -> Vec<u32> {
    let active = composition_system(comp, t_ms);
    let mut v: Vec<u32> = active.keys().cloned().collect();
    v.sort();
    v
}