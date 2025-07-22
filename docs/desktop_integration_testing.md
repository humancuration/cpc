# Desktop Integration Testing Guide

## Test Cases
1. Minimize/Restore Transitions
   - Verify GPU resource management during minimization
   - Check state restoration correctness
2. Focus Changes
   - Validate rendering quality adjustments
   - Confirm event handling
3. Surface Creation Timing
   - Ensure surfaces only created after window positioning
   - Test on initial launch and window moves
4. Texture Management
   - Verify texture preservation during pause/resume
   - Check memory usage patterns
5. Platform Compatibility
   - Windows: Test surface handles
   - macOS: Test windowing system integration
   - Linux: Verify both X11 and Wayland support

## Test Execution
```bash
# Build and run desktop app
./apps/cpc-platform/verify_desktop_build.sh --test
```

## Verification Points
- [ ] Window state transitions
- [ ] GPU resource handling
- [ ] Cross-platform consistency