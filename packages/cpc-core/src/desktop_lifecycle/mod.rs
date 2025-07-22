//! Desktop platform lifecycle management

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "windows")]
pub use windows::*;
#[cfg(target_os = "linux")]
pub use linux::*;

use crate::bevy::thread::{BevyThread, BevyThreadCommand};
use std::os::raw::c_void;

/// Trait for desktop lifecycle management
pub trait DesktopLifecycle {
    /// Create a new instance with window reference
    fn new(window: &Window) -> Self;
    
    /// Create a surface and return its handle
    fn create_surface(&self) -> *mut c_void;
    
    /// Handle minimize event
    fn on_minimize(&self, bevy_thread: &BevyThread);
    
    /// Handle restore event
    fn on_restore(&self, bevy_thread: &BevyThread);
    
    /// Handle focus change event
    fn on_focus_change(&self, bevy_thread: &BevyThread, focused: bool);
}