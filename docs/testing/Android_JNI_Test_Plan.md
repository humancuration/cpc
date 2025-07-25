# Android JNI Build Verification

## Overview
This document outlines the steps to verify the Rust JNI build for Android. The process ensures that:
1. Rust code compiles to Android targets
2. Rust build is integrated into the Android Gradle process
3. The JNI library is properly included in the final APK

## Prerequisites
1. Install required Android targets:
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

## Build Process
1. The Rust JNI binding generation is handled by `build.rs`:
```rust
extern crate cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;
    
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("jni_bridge.h");
}
```

2. The Gradle build file (`app/build.gradle`) includes:
```groovy
android {
    ...
    sourceSets.main {
        jniLibs.srcDirs = ['src/main/rust/target/aarch64-linux-android/release'] 
    }
}

task buildRust(type: Exec) {
    workingDir 'src/main/rust'
    commandLine 'cargo', 'build', '--target', 'aarch64-linux-android', '--release'
}

preBuild.dependsOn buildRust
```

## Verification
Run the verification script after building the APK:
```bash
cd apps/cpc-platform/android
./verify_apk.sh
```

The script checks if the native library is included in the APK:
```bash
#!/bin/bash
apk_path="app/build/outputs/apk/debug/app-debug.apk"
if unzip -l "$apk_path" | grep "lib/.*/libcpc_jni.so"; then
    echo "Native library found in APK"
    exit 0
else
    echo "Native library missing from APK"
    exit 1
fi
```

## Expected Result
Successful verification output:
```
Native library found in APK