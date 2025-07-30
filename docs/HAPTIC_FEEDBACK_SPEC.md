# Haptic Feedback Specification for Finance-Sheets

## Core Implementation Requirements
All haptic responses must:
- Use Android's `VibrationEffect` API (minimum API 26)
- Respect system vibration settings (check `Settings.System.VIBRATE_SETTING`)
- Implement timeout safety (max 500ms duration per trigger)
- Support configurable intensity in app settings (0-100%)

## Gesture-to-Haptic Mapping
| Interaction        | Duration | Intensity | Pattern               | Android API Call                           |
|--------------------|----------|-----------|-----------------------|--------------------------------------------|
| Cell Selection     | 50ms     | 30%       | Single pulse          | `VibrationEffect.createOneShot(50, 255*0.3)` |
| Cell Edit Start    | 100ms    | 60%       | Double pulse          | `VibrationEffect.createWaveform([0,50,100], [153,153], -1)` |
| Sheet Switch       | 150ms    | 90%       | Sustained vibration   | `VibrationEffect.createOneShot(150, 255*0.9)` |

## Technical Implementation Plan

### 1. Android Bridge Setup
```rust
// apps/finance-sheets/src/mobile/haptics.rs
#[cfg(target_os = "android")]
pub fn trigger_haptic(duration: u32, intensity: f32) {
    #[cfg(target_os = "android")]
    {
        use jni::objects::{JClass, JObject, JString};
        use jni::sys::{jfloat, jint};
        use jni::JNIEnv;
        
        if let Ok(vm) = ndk_context::android_context().vm() {
            let env = vm.attach_current_thread().unwrap();
            let context = ndk_context::android_context().context();
            
            // Get VibrationManager
            let vibrator_manager: JObject = env
                .call_method(
                    context,
                    "getSystemService",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    &[env.new_string("vibration").unwrap().into()]
                )
                .unwrap()
                .l()
                .unwrap();
                
            // Create vibration effect
            let effect_class = env.find_class("android/os/VibrationEffect").unwrap();
            let create_one_shot = env.get_static_method_id(
                effect_class,
                "createOneShot",
                "(JF)Landroid/os/VibrationEffect;"
            ).unwrap();
            
            let amplitude = (255.0 * intensity) as jint;
            let effect: JObject = env
                .call_static_method(
                    effect_class,
                    create_one_shot,
                    "(JF)Landroid/os/VibrationEffect;",
                    &[duration.into(), amplitude as jfloat]
                )
                .unwrap()
                .l()
                .unwrap();
                
            // Trigger vibration
            env.call_method(
                vibrator_manager,
                "vibrate",
                "(Landroid/os/VibrationEffect;)V",
                &[effect.into()]
            ).unwrap();
        }
    }
}

#[cfg(not(target_os = "android"))]
pub fn trigger_haptic(_duration: u32, _intensity: f32) {
    // No-op for non-Android platforms
}
```

### 2. Integration with Gesture Detector
```rust
// apps/finance-sheets/src/components/mobile/gesture_detector.rs
use super::haptics; // New import

impl Component for GestureDetector {
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // ... existing cases ...
            
            GestureDetectorMsg::TouchEnd(x, y) => {
                // ... existing logic ...
                
                if /* cell selection condition */ {
                    haptics::trigger_haptic(50, 0.3);
                    ctx.props().on_tap.emit(());
                } else if /* edit start condition */ {
                    haptics::trigger_haptic(100, 0.6);
                    // ... edit handling ...
                }
                // ... other gesture handling ...
            }
            
            // ... other message handlers ...
        }
    }
}
```

## Testing Protocol
1. **Device Validation Matrix**
   | Device Type | Minimum API | Critical Tests |
   |-------------|-------------|----------------|
   | Budget      | 26+         | Battery impact, basic pulse |
   | Mid-range   | 28+         | Waveform accuracy, concurrency |
   | Flagship    | 30+         | Advanced haptic channels |

2. **Acceptance Criteria**
   - Vibration occurs within 10ms of gesture recognition
   - No battery drain exceeding 0.05% per interaction
   - Proper cancellation when app backgrounded
   - Works with physical keyboard attachments

## Safety Constraints
- Maximum 3 consecutive vibrations without user pause
- Automatic intensity reduction after 20 interactions/minute
- Immediate cancellation when system battery < 15%
- Respect "Do Not Disturb" modes