// Basic smoke tests to validate core scaffolding compiles and minimal behaviors work.

use video_editor_core as ve;

#[test]
fn engine_initializes() {
    let mut eng = ve::engine::Engine::new();
    eng.render_frame();
}

#[test]
fn media_cache_put_get() {
    let mut pipeline = ve::media::MediaPipeline::new();
    {
        let cache = pipeline.cache();
        // put a frame
        cache.put(ve::media::Frame { pts_ms: 1000, width: 1920, height: 1080 });
    }
    let f = pipeline.cache().get(1000);
    assert!(f.is_some());
    let f = f.unwrap();
    assert_eq!(f.pts_ms, 1000);
    assert_eq!(f.width, 1920);
    // Ensure memory accounting is non-zero
    assert!(pipeline.cache().current_bytes() > 0);
}

#[test]
fn audio_mixer_tracks() {
    let mut mixer = ve::audio::AudioMixer::new();
    let id = mixer.add_track();
    assert_eq!(id, 1);
    assert_eq!(mixer.tracks().len(), 1);
}

#[test]
fn crypto_roundtrip() {
    let proj = ve::storage::ProjectFile {
        name: "Test".to_string(),
        width: 1280,
        height: 720,
        fps: 30.0,
    };
    let pass = "secret";
    let json = serde_json::to_vec(&proj).unwrap();
    let enc = ve::storage::encrypt_project(&json, pass).unwrap();
    let dec = ve::storage::decrypt_project(&enc, pass).unwrap();
    let round: ve::storage::ProjectFile = serde_json::from_slice(&dec).unwrap();
    assert_eq!(round.name, proj.name);
    assert_eq!(round.width, proj.width);
    assert_eq!(round.height, proj.height);
    assert_eq!(round.fps, proj.fps);
}