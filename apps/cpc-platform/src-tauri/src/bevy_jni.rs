use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::sys::{jint, jlong};
use bevy::prelude::*;
use bevy::window::Window;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use cpc_core::texture_manifest::{TextureManifest, populate_texture_manifest};

struct BevyThreadManager {
    app: Mutex<Option<App>>,
    command_tx: Sender<BevyCommand>,
    running: AtomicBool,
    surface: Mutex<Option<*mut std::ffi::c_void>>,
}

#[derive(Debug)]
enum BevyCommand {
    Resize(u32, u32),
    NewSurface(*mut std::ffi::c_void),
    Exit,
}

lazy_static! {
    static ref BEVY_MANAGER: Arc<BevyThreadManager> = {
        let (tx, rx) = bounded(10);
        let manager = Arc::new(BevyThreadManager {
            app: Mutex::new(None),
            command_tx: tx,
            running: AtomicBool::new(false),
            surface: Mutex::new(None),
        });
        
        // Start Bevy thread
        let manager_clone = Arc::clone(&manager);
        thread::Builder::new()
            .name("bevy_main".into())
            .spawn(move || bevy_thread(rx, manager_clone))
            .expect("Failed to start Bevy thread");
        
        manager
    };
}

fn bevy_thread(rx: Receiver<BevyCommand>, manager: Arc<BevyThreadManager>) {
    while manager.running.load(Ordering::SeqCst) {
        // Process incoming commands
        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                BevyCommand::Resize(w, h) => handle_resize(w, h, &manager),
                BevyCommand::NewSurface(surface) => {
                    *manager.surface.lock().unwrap() = Some(surface);
                    recreate_renderer(&manager);
                }
                BevyCommand::Exit => break,
            }
        }
        
        // Run Bevy update if active
        if let Some(mut app) = manager.app.lock().unwrap().as_mut() {
            app.update();
        }
        
        // Prevent CPU spin
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn handle_resize(width: u32, height: u32, manager: &Arc<BevyThreadManager>) {
    if let Some(mut app) = manager.app.lock().unwrap().as_mut() {
        if let Some(mut window) = app.world.get_resource_mut::<Window>() {
            window.resolution.set(width, height);
        }
    }
}

fn recreate_renderer(manager: &Arc<BevyThreadManager>) {
    // Implementation would recreate renderer with new surface
    // This is platform-specific and would need Vulkan/Metal context recreation
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_BevyView_initializeBevy(
    env: JNIEnv,
    _: JObject,
    surface: JObject,
) {
    let raw_surface = surface.as_raw() as jlong;
    
    BEVY_MANAGER.running.store(true, Ordering::SeqCst);
    BEVY_MANAGER.command_tx.send(BevyCommand::NewSurface(raw_surface as *mut _))
        .expect("Failed to send surface to Bevy thread");
    
    // Initialize Bevy app in thread
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .init_resource::<TextureManifest>()
        .add_startup_system(populate_texture_manifest);
    *BEVY_MANAGER.app.lock().unwrap() = Some(app);
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_BevyView_resizeBevy(
    env: JNIEnv,
    _: JObject,
    width: jint,
    height: jint,
) {
    let width = width.max(1) as u32;
    let height = height.max(1) as u32;
    
    BEVY_MANAGER.command_tx.send(BevyCommand::Resize(width, height))
        .unwrap_or_else(|e| {
            env.throw_new("java/lang/IllegalStateException", format!("Failed to resize: {}", e))
                .expect("Failed to throw exception");
        });
}

#[no_mangle]
pub extern "system" fn Java_com_wtf_BevyView_destroyBevy(_env: JNIEnv, _: JObject) {
    BEVY_MANAGER.running.store(false, Ordering::SeqCst);
    BEVY_MANAGER.command_tx.send(BevyCommand::Exit)
        .expect("Failed to send exit command");
    *BEVY_MANAGER.app.lock().unwrap() = None;
    *BEVY_MANAGER.surface.lock().unwrap() = None;
}

// Helper for lifecycle integration
pub fn pause_bevy() {
    BEVY_MANAGER.running.store(false, Ordering::SeqCst);
}

pub fn resume_bevy() {
    BEVY_MANAGER.running.store(true, Ordering::SeqCst);
}