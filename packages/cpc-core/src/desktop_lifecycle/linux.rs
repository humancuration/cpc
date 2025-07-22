#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
    use tauri::Window;
    use std::os::raw::c_void;

    pub struct LinuxLifecycle {
        window: Window,  // Keep window reference
    }

    impl DesktopLifecycle for LinuxLifecycle {
        fn new(window: &Window) -> Self {
            LinuxLifecycle { window: window.clone() }
        }
        
        fn create_surface(&self) -> *mut c_void {
            match self.window.raw_window_handle() {
                RawWindowHandle::Xlib(handle) => handle.window as *mut _,
                RawWindowHandle::Wayland(handle) => handle.surface as *mut _,
                _ => panic!("Unsupported Linux windowing system"),
            }
        }
        
        fn on_minimize(&self, bevy_thread: &BevyThread) {
            bevy_thread.send_command(BevyThreadCommand::Minimize);
        }
        
        fn on_restore(&self, bevy_thread: &BevyThread) {
            bevy_thread.send_command(BevyThreadCommand::Restore);
        }
        
        fn on_focus_change(&self, bevy_thread: &BevyThread, focused: bool) {
            bevy_thread.send_command(BevyThreadCommand::FocusChanged(focused));
        }
    }
}