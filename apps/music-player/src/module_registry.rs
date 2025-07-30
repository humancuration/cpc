//! Module registry integration for the music player module

use sqlx::PgPool;
use crate::web::modular_module::ModularMusicPlayer;

/// Create a new modular music player instance
pub fn create_module(pool: PgPool) -> ModularMusicPlayer {
    ModularMusicPlayer::new(pool)
}