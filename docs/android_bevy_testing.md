# Android Bevy Threading Test Plan

## 1. Lifecycle State Transitions

### Test Cases:
1. Cold Start → Pause → Resume → Destroy
   - Verify state transitions: Initialized → Running → Paused → Resumed → Running → Destroyed
   - Validate texture manifest preservation during pause
   - Check GPU resource cleanup during pause

2. Backgrounding During Surface Recreation
   - Trigger pause while surface recreation is in progress
   - Validate thread state consistency
   - Check resource reloading after resume

3. Rapid Configuration Changes
   - Rotate device 10x rapidly
   - Measure surface recreation time
   - Verify no memory leaks
   - Check rendering stability

## 2. Memory Validation

### Test Cases:
1. Heap Allocation Tracking
   - Monitor native heap during backgrounding
   - Verify >50% reduction in GPU memory during pause
   - Check texture reload accuracy from manifest

2. Texture Memory Validation
   - Compare texture memory pre-pause vs post-resume
   - Verify essential textures reloaded (from manifest)
   - Check for redundant texture reloads

3. Thread Cleanup Verification
   - Validate thread termination on destroy
   - Check resource deallocation
   - Monitor JNI global reference counts

## 3. Performance Metrics

### Benchmarks:
1. Surface Recreation Time
   - Budget: <500ms on mid-range devices
   - Measure from surfaceCreated to first frame

2. Thread State Transition Latency
   - Pause: <100ms
   - Resume: <300ms (including texture reload)

3. Cross-Thread Message Throughput
   - Measure command processing rate (commands/sec)
   - Stress test with 1000+ rapid commands

## 4. Failure Cases

### Test Cases:
1. Surface Destruction During Render
   - Destroy surface mid-frame
   - Verify graceful pause state
   - Check error logging

2. Native Thread Panic Recovery
   - Inject panic in Bevy thread
   - Validate app doesn't crash
   - Check restart mechanism

3. JNI Boundary Exception Handling
   - Pass invalid surface handles
   - Test null parameters
   - Verify exception logging
   - Check app stability

## Testing Tools Setup

### Required Tools:
1. Android Studio Profiler
   - Memory profiler for native heap
   - CPU profiler for thread activity
   - Network profiler for P2P (future)

2. Systrace
   - Track surface recreation timing
   - Identify rendering bottlenecks

3. Logcat Filters
   - `tag:BevyThread` for Rust logs
   - `tag:CPC_Core` for JNI logs

4. Android Debug Bridge (ADB)
   - Trigger configuration changes: `adb shell content insert...`
   - Simulate low memory: `adb shell am send-trim-memory`

## Stress Test Automation

### Strategy:
1. Lifecycle Stress Test:
```bash
for i in {1..100}; do
  adb shell input keyevent KEYCODE_POWER  # Sleep/wake
  adb shell input keyevent KEYCODE_APP_SWITCH  # Background/foreground
  adb shell am broadcast -a android.intent.action.CONFIGURATION_CHANGED
done
```

2. Memory Stress Test:
- Use Android's `am` tool to simulate trim memory events
- Gradually increase pressure from `TRIM_MEMORY_RUNNING_MODERATE` to `TRIM_MEMORY_COMPLETE`

3. Crash Recovery Test:
- Randomly kill Bevy thread process
- Verify auto-recovery mechanism

## Pass/Fail Criteria

| Test Category       | Pass Criteria                                  |
|---------------------|-----------------------------------------------|
| Lifecycle           | State transitions match diagram in architecture doc |
| Memory              | Max native heap ≤ 150MB after 10 pause/resume cycles |
| Performance         | Surface recreation ≤ 500ms, Resume ≤ 300ms    |
| Failure Recovery    | No app crashes, errors properly logged        |
| Stress Tests        | No memory leaks after 100 configuration changes |