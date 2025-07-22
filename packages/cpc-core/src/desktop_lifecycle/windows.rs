use super::DesktopLifecycle;
use crate::bevy::thread::{BevyThread, BevyThreadCommand};
use std::os::raw::c_void;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetActiveWindow;

pub struct WindowsLifecycle;

impl DesktopLifecycle for WindowsLifecycle {
    fn new() -> Self {
        WindowsLifecycle
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
            let hwnd = GetActiveWindow();
            hwnd.0 as *mut c_void
        }
    }
}