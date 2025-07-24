# Android Implementation Verification Guide

## 1. Secure Storage Testing
### Key Files:
- `SecureStorage.kt` (Kotlin implementation)
- `android_keystore.rs` (Rust bindings)

### Test Cases:
1. **Encryption/Decryption Cycle**
   ```bash
   adb shell am start-activity -n com.wtf/.MainActivity -e test_storage true
   ```
   - [ ] Store sample data using `SecureStorage.store()`
   - [ ] Retrieve data using `SecureStorage.retrieve()`
   - [ ] Verify data integrity after roundtrip
   - [ ] Test with special characters and large payloads (>1KB)

2. **Keystore Failure Handling**
   - [ ] Force key generation failure by revoking KeyStore permissions
   - [ ] Verify graceful error handling in `getOrCreateKey()`
   - [ ] Test storage/retrieval with invalid key aliases

3. **Cross-Language Validation**
   - [ ] Verify JNI calls between `SecureStorage.kt` and `android_keystore.rs`
   - [ ] Test Tauri commands `secure_store`/`secure_retrieve` from Rust

## 2. Asset Loading Verification
### Key Files:
- `MainActivity.kt` (Asset loading logic)
- AndroidManifest.xml (WebView config)

### Test Matrix:
| Mode        | Location                | Verification Method          |
|-------------|-------------------------|-------------------------------|
| Production  | `android_asset/`        | APK inspection & file hashes  |
| Development | `WebViewAssetLoader`    | Network proxy inspection      |

### Test Steps:
1. **Production Mode**
   ```bash
   ./gradlew assembleRelease && adb install app-release.apk
   ```
   - [ ] Verify assets load from APK bundle
   - [ ] Check console for file system errors

2. **Development Mode**
   ```bash
   adb reverse tcp:3000 tcp:3000 && npm run dev
   ```
   - [ ] Confirm hot-reload functionality
   - [ ] Verify asset loader intercepts requests correctly

## 3. Bevy Rendering Check
### Key Components:
- `BevyView.kt` (Surface management)
- `bevy_jni.rs` (Rust bindings)

### Validation Procedure:
1. **Surface Initialization**
   ```kotlin
   findViewById<BevyView>(R.id.bevy_view).visibility = View.VISIBLE
   ```
   - [ ] Confirm EGL context creation in logcat
   - [ ] Verify `AndroidSurface` resource injection in Bevy

2. **Rendering Pipeline**
   - [ ] Add test entity with colored cube in `bevy_jni.rs`
   - [ ] Verify frame rendering with `adb shell dumpsys SurfaceFlinger`

3. **Lifecycle Testing**
   - [ ] Rotate device to test surface recreation
   - [ ] Verify memory cleanup after surface destruction

## 4. Tauri Service Validation
### Critical Paths:
- `TauriService.kt` (Service lifecycle)
- `main.rs` (IPC implementation)

### Test Protocol:
1. **Service Startup**
   ```bash
   adb shell am start-service com.wtf/.TauriService
   ```
   - [ ] Verify foreground notification persistence
   - [ ] Check JNI initialization in `main.rs`

2. **IPC Validation**
   ```javascript
   // From Yew frontend
   invoke('secure_store', { key: 'test', value: 'data' })
   ```
   - [ ] Confirm message routing through Rust/Kotlin bridge
   - [ ] Measure roundtrip latency with `console.time()`

3. **Stress Testing**
   - [ ] Perform 100+ consecutive IPC calls
   - [ ] Monitor memory usage with Android Profiler
   - [ ] Verify service stability after OOM scenarios

## 5. 72-Hour Stress Test Protocol

### Objectives
- Validate long-term stability under continuous operation
- Identify memory leaks and performance degradation
- Verify behavior under worst-case scenarios
- Ensure compliance with Android background process restrictions

### Key Performance Metrics
1. **Memory Usage**:
   - Heap allocations (Java/Kotlin + Native)
   - Graphics memory utilization
   - Memory growth per hour (measured in MB/hr)
2. **CPU Utilization**:
   - Per-core usage percentage
   - Thermal throttling events
3. **Rendering Performance**:
   - Frames per second (FPS) consistency
   - Frame time variance (jitter)
4. **Battery Impact**:
   - Power consumption (mW)
   - Thermal profile (device temperature)
5. **IPC Latency**:
   - Tauri command roundtrip times
   - Bevy event processing latency

### Worst-case Scenarios
1. **Continuous High Interaction**
   - Rapid screen touches/swipes while rendering complex 3D scenes
   - Background asset loading during peak interaction
   - Triggered every 30 minutes during test
2. **Background/Foreground Cycling**
   - App moved to background every 15 minutes
   - Background tasks executed (data sync, notifications)
   - Returned to foreground after 2 minutes
3. **Resource Constrained Operation**
   - Simulate low memory conditions (<100MB available)
   - CPU throttling to 50% capacity
   - Network bandwidth limitation (2G speeds)

### Pass/Fail Criteria
| Metric | Threshold | Measurement |
|--------|-----------|-------------|
| **Memory Growth** | ≤ 5% per hour | `dumpsys meminfo` |
| **Crashes** | 0 allowed | Crashlytics monitoring |
| **Rendering Consistency** | ≥30 FPS (95% of frames) | Bevy metrics system |
| **Thermal Limits** | ≤45°C sustained | Android thermal API |
| **Background Restrictions** | No service terminations | `ActivityManager` logs |

### Automation Hooks
```kotlin
// Bevy metrics integration
BevyMetricRecorder.recordStressEvent(
    type = "memory_usage",
    value = runtime.totalMemory().toFloat()
)

// ADB command triggers
adb shell am start-activity -n com.wtf/.MainActivity -e stress_test scenario1
adb shell am broadcast -a com.wtf.STRESS_TEST_SCENARIO2

// Memory pressure simulation
val memoryAllocator = MemoryAllocator()
memoryAllocator.allocatePressure(500) // MB
```

### Monitoring Tools
1. **Bevy Metrics System** (Integrated):
   ```rust
   // In bevy_jni.rs
   pub fn record_metric(metric: &str, value: f32) {
       android_log!("STRESS_METRIC: {}={}", metric, value);
   }
   ```
2. **Android Profiler Suite**:
   - Memory Profiler
   - CPU Profiler
   - Energy Profiler
3. **Perfetto System Tracing**:
   ```bash
   adb shell perfetto --config :test --out /data/misc/perfetto-traces/stress_test.pftrace
   ```
4. **Custom Log Parsing**:
   ```bash
   adb logcat -s STRESS_METRIC | tee stress_metrics.log
   ```

## Implementation Notes:
1. Current Limitations:
   - JNI bindings in `android_keystore.rs` show placeholder implementations (lines 17-19, 31-33)
   - Bevy resize functionality not fully implemented (bevy_jni.rs:47-49)
   - Tauri Android context handling appears duplicated in `main.rs` (lines 41-98)

2. Recommended Test Tools:
   ```gradle
   androidTestImplementation 'androidx.test:rules:1.4.1'
   androidTestImplementation 'org.mockito:mockito-android:4.5.1'
   androidTestImplementation 'com.github.takahirom:robolectric.shadow.support.v4:4.6.1'
   // Stress test additions
   androidTestImplementation 'com.google.android.gms:play-services-basement:18.1.0'
   androidTestImplementation 'androidx.benchmark:benchmark-macro-junit4:1.1.1'
   ```

## Verification Sign-off:
```markdown
- [ ] Security Review Completed
- [ ] Performance Targets Met
- [ ] Cross-Platform Consistency Verified
- [ ] Crash-Free Session (72hr stress test)