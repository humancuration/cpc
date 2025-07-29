use tauri::{App, Runtime, Window, WindowEvent};
use cpc_core::bevy::thread::{BevyThread, BevyThreadCommand};
use cpc_core::desktop_lifecycle;

pub struct BevyPlugin {
    bevy_thread: Option<BevyThread>,
    lifecycle: Option<Box<dyn desktop_lifecycle::DesktopLifecycle>>,
}

impl BevyPlugin {
    pub fn new() -> Self {
        BevyPlugin {
            bevy_thread: None,
            lifecycle: None,
        }
    }
}

impl<R: Runtime> tauri::plugin::Plugin<R> for BevyPlugin {
    fn name(&self) -> &'static str {
        "bevy"
    }

    fn initialize(&mut self, app: &App<R>) -> tauri::plugin::Result<()> {
        self.bevy_thread = Some(BevyThread::new());
        Ok(())
    }

    fn on_window_event(&mut self, event: WindowEvent, window: Window<R>) {
        if self.lifecycle.is_none() {
            // Initialize platform-specific lifecycle handler with window reference
            #[cfg(target_os = "macos")]
            let lifecycle = Box::new(desktop_lifecycle::MacOSLifecycle::new(&window));
            #[cfg(target_os = "windows")]
            let lifecycle = Box::new(desktop_lifecycle::WindowsLifecycle::new(&window));
            #[cfg(target_os = "linux")]
            let lifecycle = Box::new(desktop_lifecycle::LinuxLifecycle::new(&window));
            
            self.lifecycle = Some(lifecycle);
        }
        
        if let Some(bevy_thread) = &self.bevy_thread {
            match event {
                WindowEvent::Moved | WindowEvent::Resized(_) => {
                    // Create surface only after window is positioned
                    if let Some(lifecycle) = &self.lifecycle {
                        let handle = lifecycle.create_surface();
                        bevy_thread.send_command(BevyThreadCommand::SurfaceCreated(handle));
                    }
                }
                WindowEvent::CloseRequested => {
                    bevy_thread.send_command(BevyThreadCommand::Destroy);
                }
                WindowEvent::Focused(focused) => {
                    bevy_thread.send_command(BevyThreadCommand::FocusChanged(focused));
                }
                WindowEvent::Minimized => {
                    if let Some(lifecycle) = &self.lifecycle {
                        lifecycle.on_minimize(bevy_thread);
                    }
                }
                WindowEvent::Restored => {
                    if let Some(lifecycle) = &self.lifecycle {
                        lifecycle.on_restore(bevy_thread);
                    }
                }
                _ => {}
            }
        }
    }

    fn on_window_event(&mut self, event: WindowEvent, window: Window<R>) {
        if let Some(bevy_thread) = &self.bevy_thread {
            match event {
                WindowEvent::Resized(_) => {
                    // Handle resize
                }
                WindowEvent::CloseRequested => {
                    bevy_thread.send_command(BevyThreadCommand::Destroy);
                }
                WindowEvent::Focused(focused) => {
                    bevy_thread.send_command(BevyThreadCommand::FocusChanged(focused));
                }
                WindowEvent::Minimized => {
                    if let Some(lifecycle) = &self.lifecycle {
                        lifecycle.on_minimize(bevy_thread);
                    }
                }
                WindowEvent::Restored => {
                    if let Some(lifecycle) = &self.lifecycle {
                        lifecycle.on_restore(bevy_thread);
                    }
                }
                _ => {}
            }
        }
    }
}