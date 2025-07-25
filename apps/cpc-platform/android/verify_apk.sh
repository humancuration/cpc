#!/bin/bash

# Expected ABIs
abis=("aarch64" "armv7" "i686" "x86_64")
build_types=("debug" "release")

exit_code=0

for build_type in "${build_types[@]}"; do
    apk_path="app/build/outputs/apk/${build_type}/app-${build_type}.apk"
    
    if [ ! -f "$apk_path" ]; then
        echo "APK not found: ${apk_path}"
        continue
    fi

    echo "Verifying APK: ${apk_path}"
    missing_count=0

    for abi in "${abis[@]}"; do
        lib_path="lib/${abi}/libcpc_jni.so"
        if unzip -l "$apk_path" | grep -q "$lib_path"; then
            echo "  ✓ Found ${abi} library"
        else
            echo "  ✗ Missing ${abi} library"
            ((missing_count++))
        fi
    done

    if [ $missing_count -eq 0 ]; then
        echo "✅ All native libraries present in ${build_type} APK"
    else
        echo "❌ ${missing_count} native libraries missing in ${build_type} APK"
        exit_code=1
    fi
done

exit $exit_code