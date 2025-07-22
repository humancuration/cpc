use super::DesktopLifecycle;
use crate::bevy::thread::{BevyThread, BevyThreadCommand};
use objc::{runtime::Object, sel, sel_impl};
use std::os::raw::c_void;

pub struct MacOSLifecycle;

impl DesktopLifecycle for MacOSLifecycle {
    fn new() -> Self {
        MacOSLifecycle
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

    fn get_native_window(&self) -> *mut c_void {
        unsafe {
            let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
            let window: *mut Object = msg_send![app, mainWindow];
            window as _
        }
    }
}