//! CPC Video Editor Core
//!
//! Reference: apps/video_editor/docs/architecture.md
//!
//! Hexagonal architecture with vertical slices: engine, media, ui, audio, plugins, project, storage.

pub mod bootstrap {
    use crate::{
        media,
        audio,
        ui::timeline::TimelinePlugin,
        engine::composition::CompositionPlugin,
        engine::effects::{EffectsPlugin, EffectsRegistry, WgpuDevice},
        plugins::{CrossfadeEffect, SlideEffect, WipeEffect},
    };
    use tracing::info;

    // Temporary standalone wgpu bootstrap until we integrate with Bevy's render sub-app device.
    // See docs/architecture.md "Effects Pipeline".
    fn init_wgpu_blocking() -> WgpuDevice {
        // Conservative device request: default limits/features.
        // This is runtime-only; unit tests should not hit this path.
        let instance = wgpu::Instance::default();
        let adapter = futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })).expect("Failed to find a suitable GPU adapter");

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("cpc-video-editor-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(), // conservative
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None, // trace path
        )).expect("Failed to create wgpu device");

        WgpuDevice { device, queue }
    }

    // Headless bootstrap keeps systems constructed; host app (desktop/web) can build a Bevy App and add plugins.
    pub fn run_headless() {
        info!("Bootstrapping headless core for smoke test");
        let _aud = audio::AudioMixer::new();
        let _mp = media::MediaPipeline::new();

        // Example of ECS world setup without full Bevy App runner:
        let mut app = bevy::prelude::App::new();
        app.add_plugins(bevy::MinimalPlugins);
        app.add_plugins(bevy::log::LogPlugin::default());
        app.add_plugins(bevy::time::TimePlugin::default());
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.add_plugins(bevy::diagnostic::DiagnosticsPlugin::default());

        // Insert EffectsRegistry with built-in GPU effects registered at startup.
        let mut reg = EffectsRegistry::default();
        reg.0.register_effect(CrossfadeEffect);
        reg.0.register_effect(SlideEffect);
        reg.0.register_effect(WipeEffect);
        app.insert_resource(reg);

        // Insert a real WgpuDevice resource so EffectsPlugin can run GPU passes
        // once frame IO is available. This is a temporary bootstrap until we
        // integrate with Bevy's render device.
        let wgpu_dev = init_wgpu_blocking();
        app.insert_resource(wgpu_dev);

        app.add_plugins(TimelinePlugin);
        app.add_plugins(CompositionPlugin);
        app.add_plugins(EffectsPlugin);

        info!("Headless core initialized (ECS world with Timeline/Composition/Effects plugins, EffectsRegistry, WgpuDevice)");
    }
}

pub mod engine {
    // Split into submodules for screaming architecture
    pub use crate::engine::*;
    pub mod composition;
    pub mod systems {
        pub use crate::engine::systems::*;
    }
    pub mod effects;
}

pub mod media {
    use tracing::info;
    pub use crate::engine::media_pipeline::{FrameProvider, DecodedFrame, ClipId, Frame as CpuFrame, render_frame};

    // LRU cache size target ~2GB configurable; placeholder type
    pub struct FrameCacheConfig {
        pub max_bytes: usize,
    }

    impl Default for FrameCacheConfig {
        fn default() -> Self {
            Self { max_bytes: 2 * 1024 * 1024 * 1024 } // 2GB
        }
    }

    pub struct MediaPipeline {
        cache_cfg: FrameCacheConfig,
    }

    impl MediaPipeline {
        pub fn new() -> Self {
            info!("Media pipeline initialized (stub)");
            Self { cache_cfg: FrameCacheConfig::default() }
        }

        pub fn import_media(&mut self, _path: &str) {
            // Later: background proxy generation (AV1/Opus/WebM), ffmpeg.wasm in WASM context
        }
    }

    // Proxy management API
    pub struct ProxyRequest {
        pub input_path: String,
        pub target_height: u32,
        pub bitrate_kbps: u32,
    }

    pub trait ProxyManager {
        fn enqueue(&self, req: ProxyRequest);
    }
pub mod ui {
    // Yew components will live here; kept minimal so core compiles without web
    // Accessibility considerations: keyboard navigation, ARIA roles, color contrast.
    pub mod timeline;
}
    }
}

pub mod audio {
    use tracing::info;

    pub struct AudioMixer;

    impl AudioMixer {
        pub fn new() -> Self {
            // Later: rodio output stream and keyframeable volume automation
            info!("Audio mixer initialized (stub)");
            Self
        }

        pub fn set_master_volume(&self, _vol_db: f32) {
            // keyframeable in timeline
        }
    }
}

pub mod plugins;

pub mod project {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Default)]
    pub struct Project {
        pub name: String,
        pub width: u32,
        pub height: u32,
        pub fps: f32,
    }

    #[derive(Debug)]
    pub enum SaveBackend {
        Local,
        Cloud,
    }
}

pub mod storage {
    use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, OsRng}};
    use sha2::{Sha256, Digest};
    use anyhow::Result;

    pub fn encrypt_project(bytes: &[u8], passphrase: &str) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(passphrase.as_bytes());
        let key = hasher.finalize();

        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        let nonce = aes_gcm::Nonce::from_slice(&rand::random::<[u8;12]>());
        let mut out = Vec::from(nonce.as_slice());
        let ct = cipher.encrypt(nonce, bytes)?;
        out.extend_from_slice(&ct);
        Ok(out)
    }

    pub fn decrypt_project(bytes: &[u8], passphrase: &str) -> Result<Vec<u8>> {
        if bytes.len() < 12 { anyhow::bail!("ciphertext too short"); }
        let (nonce_bytes, ct) = bytes.split_at(12);

        let mut hasher = sha2::Sha256::new();
        hasher.update(passphrase.as_bytes());
        let key = hasher.finalize();

        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
        let pt = cipher.decrypt(nonce, ct)?;
        Ok(pt)
    }
}