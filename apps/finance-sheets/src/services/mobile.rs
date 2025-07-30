1 | //! Mobile services for Finance-Sheets
2 | //!
3 | //! This module contains service implementations that handle mobile-specific functionality
4 | //! such as device detection, storage, and performance optimizations.

pub mod storage;
pub mod sync;
pub mod performance;
#[cfg(test)]
pub mod test;

/// Device size categories for responsive design
#[derive(Clone, Debug, PartialEq)]
pub enum DeviceSize {
    Mobile,
    Tablet,
    Desktop,
}

/// Get the current device size based on screen dimensions
///
/// This function uses window dimensions to determine the device category:
/// - Mobile: width <= 768px
/// - Tablet: width > 768px and <= 1024px
/// - Desktop: width > 1024px
pub fn get_device_size() -> DeviceSize {
    // In a web environment, we can use web-sys to get window dimensions
    use web_sys::window;
    
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            let width = document.document_element()
                .map(|el| el.client_width())
                .unwrap_or(0);
                
            match width {
                w if w <= 768 => DeviceSize::Mobile,
                w if w <= 1024 => DeviceSize::Tablet,
                _ => DeviceSize::Desktop,
            }
        } else {
            DeviceSize::Desktop // Default to desktop if we can't determine
        }
    } else {
        DeviceSize::Desktop // Default to desktop if we can't access window
    }
}

/// Check if the current environment is a mobile device
///
/// This function provides a simple boolean check for mobile optimization
pub fn is_mobile() -> bool {
    matches!(get_device_size(), DeviceSize::Mobile)
}

/// Check if the current environment is a tablet device
pub fn is_tablet() -> bool {
    matches!(get_device_size(), DeviceSize::Tablet)
}

/// Check if the current environment is a desktop device
pub fn is_desktop() -> bool {
    matches!(get_device_size(), DeviceSize::Desktop)
}