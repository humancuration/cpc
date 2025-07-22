use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::thread;
use std::sync::{Arc, Mutex};
use std::ffi::c_void;
use crate::TextureManifest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BevyThreadState {
    Initialized,
    Running,
    Paused,
    Resumed,
    Destroyed,
    Minimized, // Desktop-specific state
    Restored,  // Desktop-specific state
}

pub enum BevyThreadCommand {
    Pause,
    Resume,
    SurfaceCreated(*mut c_void),
    SurfaceDestroyed,
    Minimize,
    Restore,
    FocusChanged(bool),
}

pub struct BevyThread {
    command_sender: Sender<BevyThreadCommand>,
    state: Arc<Mutex<BevyThreadState>>,
    surface_handle: Arc<Mutex<Option<*mut c_void>>>,
    texture_manifest: Arc<Mutex<Option<Vec<String>>>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl BevyThread {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        let state = Arc::new(Mutex::new(BevyThreadState::Initialized));
        let surface_handle = Arc::new(Mutex::new(None));
        let texture_manifest = Arc::new(Mutex::new(None));
        let thread_state = state.clone();
        let thread_surface = surface_handle.clone();
        let thread_texture_manifest = texture_manifest.clone();

        let handle = thread::spawn(move || {
            let mut app = None;
            
            // Main game loop
            loop {
                // Process incoming commands
                while let Ok(command) = rx.try_recv() {
                    match command {
                        BevyThreadCommand::Pause => {
                            *thread_state.lock().unwrap() = BevyThreadState::Paused;
                            println!("Bevy thread: Paused");
                            
                            // Release GPU resources and capture texture manifest
                            if let Some(app) = app.as_mut() {
                                // Capture texture manifest if it exists
                                if let Some(manifest) = app.world.get_resource::<TextureManifest>() {
                                    let mut global_manifest = thread_texture_manifest.lock().unwrap();
                                    *global_manifest = Some(manifest.paths.clone());
                                }

                                // Clean up textures to reduce memory pressure
                                println!("Cleaning up Bevy textures");
                                app.world.resource_mut::<Assets<Texture>>().clear();
                            }
                        }
                        BevyThreadCommand::Resume => {
                            *thread_state.lock().unwrap() = BevyThreadState::Resumed;
                            println!("Bevy thread: Resumed");
                            
                            // Handle surface recreation if needed
                            if let Some(handle) = *thread_surface.lock().unwrap() {
                                println!("Recreating surface with handle: {:?}", handle);
                                app = Some(Self::create_bevy_app(handle, thread_texture_manifest.lock().unwrap().clone()));
                                *thread_state.lock().unwrap() = BevyThreadState::Running;
                            }
                        }
                        BevyThreadCommand::SurfaceCreated(handle) => {
                            *thread_surface.lock().unwrap() = Some(handle);
                            println!("Bevy thread: Surface created");
                            
                            // If we're initialized or resumed, create the app
                            let current_state = thread_state.lock().unwrap().clone();
                            if current_state == BevyThreadState::Initialized ||
                               current_state == BevyThreadState::Resumed {
                                app = Some(Self::create_bevy_app(handle, thread_texture_manifest.lock().unwrap().clone()));
                                *thread_state.lock().unwrap() = BevyThreadState::Running;
                            }
                        }
                        BevyThreadCommand::SurfaceDestroyed => {
                            *thread_surface.lock().unwrap() = None;
                            println!("Bevy thread: Surface destroyed");
                            
                            if *thread_state.lock().unwrap() == BevyThreadState::Running {
                                *thread_state.lock().unwrap() = BevyThreadState::Paused;
                            }
                        }
                        BevyThreadCommand::Minimize => {
                            *thread_state.lock().unwrap() = BevyThreadState::Minimized;
                            println!("Bevy thread: Minimized");
                        }
                        BevyThreadCommand::Restore => {
                            *thread_state.lock().unwrap() = BevyThreadState::Restored;
                            println!("Bevy thread: Restored");
                        }
                        BevyThreadCommand::FocusChanged(focused) => {
                            println!("Bevy thread: Focus changed to {}", focused);
                            // Reduce rendering quality when not focused
                        }
                    }
                }
                
                // Run game update if in running state
                if *thread_state.lock().unwrap() == BevyThreadState::Running {
                    if let Some(app) = app.as_mut() {
                        app.update();
                    }
                }
                
                // Sleep to avoid busy-waiting
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        });

        Self {
            command_sender: tx,
            state,
            surface_handle,
            texture_manifest,
            thread_handle: Some(handle),
        }
    }

    fn create_bevy_app(surface: *mut c_void, texture_manifest: Option<Vec<String>>) -> App {
        let mut app = App::new();
        
        // Configure Bevy for Android
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(surface as _),
                ..default()
            }),
            ..default()
        }));
        
        // Add texture manifest resource
        if let Some(paths) = texture_manifest {
            app.world.insert_resource(TextureManifest { paths: paths.clone() });
        }
        
        // Add our custom plugins
        app.add_plugin(crate::bevy::CpcBevyPlugin);
        
        // Preload textures if manifest exists
        if let Some(manifest) = app.world.get_resource::<TextureManifest>() {
            let asset_server = app.world.resource::<AssetServer>();
            for path in &manifest.paths {
                asset_server.load::<Texture>(path);
            }
        }
        
        app
    }

    pub fn send_command(&self, command: BevyThreadCommand) {
        self.command_sender.send(command).unwrap();
    }

    pub fn state(&self) -> BevyThreadState {
        self.state.lock().unwrap().clone()
    }
}