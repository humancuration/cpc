cmake_minimum_required(VERSION 3.22.1)

# Add the prebuilt Rust library
add_library(cpc_platform SHARED IMPORTED)
set_target_properties(cpc_platform PROPERTIES IMPORTED_LOCATION
    ${CMAKE_CURRENT_SOURCE_DIR}/build/rustJniLibs/${ANDROID_ABI}/libcpc_platform.so)

# Find required libraries
find_library(log-lib log)

# Link libraries to cpc_platform
target_link_libraries(cpc_platform
    android
    ${log-lib}
    # Add Bevy dependencies as needed
)

# Include directories
include_directories(src/main/cpp)