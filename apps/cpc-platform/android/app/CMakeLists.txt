cmake_minimum_required(VERSION 3.18.1)

project("cpc_core")

# Set the path to the Rust library
set(RUST_LIBRARY_DIR "${CMAKE_SOURCE_DIR}/../../../../target/release")

# Find the Rust library
find_library(
    log-lib
    log
)

# Import the Rust library
add_library(
    cpc_core
    SHARED
    IMPORTED
)

# Set the properties for the imported library
set_target_properties(
    cpc_core
    PROPERTIES IMPORTED_LOCATION
    ${RUST_LIBRARY_DIR}/libcpc_core.so
)

# Include directories
include_directories(${CMAKE_SOURCE_DIR}/src/main/cpp/include)

# Create the final shared library
add_library(
    cpc_android
    SHARED
    src/main/cpp/native-lib.cpp
)

# Link the libraries
target_link_libraries(
    cpc_android
    ${log-lib}
    cpc_core
)