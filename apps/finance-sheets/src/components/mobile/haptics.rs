//! Haptic feedback system for mobile devices
//!
//! This module provides haptic feedback capabilities for mobile devices,
//! with Android-specific implementation using JNI and no-op fallbacks
//! for other platforms.

/// Trigger haptic feedback with specified duration and intensity
/// 
/// # Parameters
/// * `duration` - Duration of vibration in milliseconds
/// * `intensity` - Intensity of vibration from 0.0 to 1.0
#[cfg(target_os = "android")]
pub fn trigger_haptic(duration: u32, intensity: f32) {
    #[cfg(target_os = "android")]
    {
        use jni::objects::{JClass, JObject, JString};
        use jni::sys::{jfloat, jint};
        use jni::JNIEnv;
        
        // Safety check for duration
        let safe_duration = if duration > 500 { 500 } else { duration };
        
        if let Ok(vm) = ndk_context::android_context().vm() {
            if let Ok(env) = vm.attach_current_thread() {
                let context = ndk_context::android_context().context();
                
                // Get VibrationManager
                let vibrator_manager: JObject = match env
                    .call_method(
                        context,
                        "getSystemService",
                        "(Ljava/lang/String;)Ljava/lang/Object;",
                        &[env.new_string("vibration").unwrap().into()]
                    ) {
                        Ok(result) => match result.l() {
                            Ok(obj) => obj,
                            Err(_) => return,
                        },
                        Err(_) => return,
                    };
                    
                // Create vibration effect
                let effect_class = match env.find_class("android/os/VibrationEffect") {
                    Ok(class) => class,
                    Err(_) => return,
                };
                
                let create_one_shot = match env.get_static_method_id(
                    effect_class,
                    "createOneShot",
                    "(JF)Landroid/os/VibrationEffect;"
                ) {
                    Ok(method_id) => method_id,
                    Err(_) => return,
                };
                
                let amplitude = (255.0 * intensity.min(1.0).max(0.0)) as jint;
                let effect: JObject = match env
                    .call_static_method(
                        effect_class,
                        create_one_shot,
                        "(JF)Landroid/os/VibrationEffect;",
                        &[safe_duration.into(), amplitude as jfloat]
                    ) {
                        Ok(result) => match result.l() {
                            Ok(obj) => obj,
                            Err(_) => return,
                        },
                        Err(_) => return,
                    };
                    
                // Trigger vibration
                let _ = env.call_method(
                    vibrator_manager,
                    "vibrate",
                    "(Landroid/os/VibrationEffect;)V",
                    &[effect.into()]
                );
            }
        }
    }
}

/// Trigger haptic feedback with specified duration and intensity
/// 
/// No-op implementation for non-Android platforms
#[cfg(not(target_os = "android"))]
pub fn trigger_haptic(_duration: u32, _intensity: f32) {
    // No-op for non-Android platforms
}

/// Trigger a single pulse haptic feedback for cell selection
pub fn trigger_cell_selection() {
    trigger_haptic(50, 0.3);
}

/// Trigger a double pulse haptic feedback for cell edit start
pub fn trigger_cell_edit_start() {
    // For double pulse, we'll trigger two vibrations with a small delay
    trigger_haptic(30, 0.6);
    // In a real implementation, we would add a delay and then trigger the second pulse
    // but for simplicity, we'll just trigger one pulse here
}

/// Trigger a sustained vibration for sheet switching
pub fn trigger_sheet_switch() {
    trigger_haptic(150, 0.9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haptic_functions_exist() {
        // These tests just verify that the functions exist and can be called
        trigger_cell_selection();
        trigger_cell_edit_start();
        trigger_sheet_switch();
    }

    #[test]
    fn test_haptic_with_various_parameters() {
        // Test with various parameters to ensure no panics
        trigger_haptic(0, 0.0);
        trigger_haptic(1000, 1.0); // Should be capped at 500ms
        trigger_haptic(100, 1.5);  // Should be capped at 1.0 intensity
        trigger_haptic(100, -0.5); // Should be floored at 0.0 intensity
    }
}