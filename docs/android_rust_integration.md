**Executive Summary**: The Android-Rust integration for the Music Player now includes comprehensive consent management with five consent types, each requiring specific authorization tokens. All privacy-sensitive operations (playback, recommendations, social interactions, and offline downloads) require valid consent tokens, with proper error handling for expired or missing consents. The Kotlin bridge interface mirrors the Rust implementation with dedicated data classes and privacy-aware methods.

# Android-Rust Integration Architecture

**Version:** 1.0
**Date:** July 21, 2025
**Author:** Gemini

## 1. Overview

This document details the architecture and protocols for integrating the Rust-based `cpc-core` library with the Android platform. The integration enables a hybrid application model where a native Android UI (Jetpack Compose) coexists with a high-performance Bevy-based game engine.

The core components of this integration are:
- **JNI Bridge:** A foreign function interface (FFI) that allows Kotlin/Java code to call Rust functions and vice-versa.
- **Lifecycle Management:** A set of hooks to manage the state of the Rust core in alignment with the Android application lifecycle.
- **Texture Exchange:** A protocol for sharing graphical data (bitmaps and textures) between the Android UI and the Bevy engine.
- **Event Bus:** A system for passing events and commands between the UI layer and the game logic.

## 2. Lifecycle Hook Mapping

The `cpc-core` library mirrors the Android application lifecycle to ensure proper resource management and state synchronization. The following diagram illustrates the mapping of lifecycle events from the Android `MainActivity` to the Rust `android_lifecycle` module.

```mermaid
graph TD
    subgraph MainActivity (Kotlin)
        A[onCreate] --> B[NativeBridge.androidOnCreate]
        C[onPause] --> D[NativeBridge.androidOnPause]
        E[onResume] --> F[NativeBridge.androidOnResume]
        G[onDestroy] --> H[NativeBridge.androidOnDestroy]
    end

    subgraph cpc-core (Rust)
        B --> I[Java_com_cpc_CPCCore_onCreate]
        D --> J[Java_com_cpc_CPCCore_onPause]
        F --> K[Java_com_cpc_CPCCore_onResume]
        H --> L[Java_com_cpc_CPCCore_onDestroy]
    end

    J --> M[Pause Bevy Engine & P2P]
    K --> N[Resume Bevy Engine & P2P]

    style A fill:#A2D5F2
    style C fill:#A2D5F2
    style E fill:#A2D5F2
    style G fill:#A2D5F2
    style I fill:#FFB347
    style J fill:#FFB347
    style K fill:#FFB347
    style L fill:#FFB347
```

**Implementation Details:**
- **`cpc-core/src/android_lifecycle.rs`**: Contains the Rust implementations of the JNI lifecycle functions.
- **`WheresThisFrom/app/src/main/kotlin/com/wtf/app/MainActivity.kt`**: The main Android activity that triggers the lifecycle events.
- **`WheresThisFrom/app/src/main/java/com/cpc/NativeBridge.kt`**: The Kotlin object that declares the `external` JNI functions.

## 3. JNI Interface Specification

The JNI bridge exposes several functions to facilitate communication between the Android app and `cpc-core`.

| Function Signature (Kotlin) | Function Signature (Rust) | Description | Data Flow |
|---|---|---|---|
| `external fun androidOnCreate()` | `Java_com_cpc_CPCCore_onCreate` | Initializes the core library. | Android -> Rust |
| `external fun androidOnPause()` | `Java_com_cpc_CPCCore_onPause` | Pauses the core library systems. | Android -> Rust |
| `external fun androidOnResume()` | `Java_com_cpc_CPCCore_onResume` | Resumes the core library systems. | Android -> Rust |
| `external fun androidOnDestroy()` | `Java_com_cpc_CPCCore_onDestroy` | Cleans up resources used by the core. | Android -> Rust |
| `external fun createBevySurfaceView(context: Context): SurfaceView` | `Java_com_cpc_NativeBridge_createBevySurfaceView` | Creates the Android `SurfaceView` for Bevy to render on. | Android -> Rust -> Android |
| `external fun requestUI(component: String, props: String): Any` | `Java_com_cpc_NativeBridge_requestUI` | Requests the native UI to render a component to a texture (Bitmap). | Rust -> Android -> Rust |
| `external fun sendGameEvent(eventType: String, data: String)` | `Java_com_cpc_NativeBridge_sendGameEvent` | Sends a game-related event from the UI to the Bevy engine. | Android -> Rust |
| `external fun sendTextureToEngine(bitmap: Any)` | `Java_com_cpc_NativeBridge_sendTextureToEngine` | Sends a texture (e.g., from camera) from Android to the Bevy engine. | Android -> Rust |

## 4. Texture Exchange Protocol

A critical feature of the hybrid model is the ability to render native UI components as textures within the Bevy engine. This is achieved by rendering a Jetpack Compose component to an Android `Bitmap`, passing its pixel data to Rust, and creating a Bevy `Texture` from it.

```mermaid
sequenceDiagram
    participant Bevy as Bevy Engine (Rust)
    participant Bridge as JNI Bridge
    participant Android as Android UI (Kotlin)

    Bevy->>Bridge: request_ui("UserProfile", props_json)
    Bridge->>Android: NativeBridge.requestUI("UserProfile", props_json)
    Android->>Android: Render Compose UI to Bitmap
    Android-->>Bridge: return Bitmap object
    Bridge->>Bridge: Copy Bitmap pixel data to Rust buffer
    Bridge-->>Bevy: return TextureData
    Bevy->>Bevy: Create bevy::render::texture::Image
```

**Code References:**
- **Rust:** `cpc-core/src/bridge/android.rs` (`Java_com_cpc_NativeBridge_requestUI`)
- **Kotlin:** `WheresThisFrom/app/src/main/java/com/cpc/NativeBridge.kt`

## 5. Event Handling Sequence

User interactions in the native Android UI can trigger events that need to be handled by the Bevy engine. The `sendGameEvent` JNI function facilitates this communication.

```mermaid
sequenceDiagram
    participant Android as Android UI (Kotlin)
    participant Bridge as JNI Bridge
    participant Bevy as Bevy Engine (Rust)
    participant ECS as Bevy ECS

    Android->>Bridge: NativeBridge.sendGameEvent("button_click", event_data_json)
    Bridge->>Bevy: handle_game_event("button_click", event_data_value)
    Bevy->>ECS: Fire GameEvent
    ECS->>ECS: Systems react to GameEvent
```

**Code References:**
- **Rust:** `cpc-core/src/bridge/android.rs` (`Java_com_cpc_NativeBridge_sendGameEvent`)
- **Bevy:** `cpc-core/src/bevy/mod.rs` (`handle_engine_messages` system)

## 6. Protocol Versioning

To ensure stability and prevent breaking changes as the platform evolves, the JNI interface must be versioned.

### 6.1. Versioned Interface Contracts

- A version number will be exposed via a JNI function (e.g., `getBridgeVersion() -> i32`).
- The Android app will check this version on startup to ensure compatibility.
- Any change to a JNI function signature or the data structure of its parameters constitutes a breaking change and requires a version increment.

### 6.2. Backward Compatibility Strategy

- When a breaking change is necessary, the old JNI function will be marked as deprecated but retained for a limited time (e.g., 2-3 release cycles).
- The new function will be introduced with a versioned name (e.g., `sendGameEvent_v2`).
- The Android app will use feature detection based on the bridge version to call the appropriate function.

### 6.3. Deprecation Process

1.  **Mark as Deprecated:** The old JNI function in Rust is marked with `#[deprecated]`. The corresponding Kotlin `external fun` is marked with `@Deprecated`.
2.  **Log Warnings:** When a deprecated function is called, a warning is logged on both the Android (Logcat) and Rust (stdout) sides.
3.  **Removal:** After the grace period, the deprecated function is removed, and the bridge version is updated.

## 7. Music Player Bridge Implementation

The Music Player module implements a comprehensive bridge interface that handles both functionality and privacy requirements through consent-aware methods.

### Kotlin Data Classes

The bridge uses the following data classes for communication between Kotlin and Rust:

```kotlin
data class PlaySession(
    val sessionId: String,
    val trackId: String,
    val positionMs: Int
)

data class Track(
    val id: String,
    val title: String,
    val artistId: String,
    val durationMs: Int,
    val albumId: String? = null,
    val coverArtUrl: String? = null
)

data class DownloadStatus(
    val trackId: String,
    val status: String, // "pending", "downloading", "completed", "failed"
    val progress: Float = 0.0f,
    val offlineUrl: String? = null
)

data class ConsentStatus(
    val consentType: String,
    val granted: Boolean,
    val expiresAt: Long? // Unix timestamp in milliseconds
)

data class ConsentRequestResult(
    val consentType: String,
    val granted: Boolean,
    val newToken: String? = null
)
```

### Consent-Aware Bridge Methods

All privacy-sensitive operations require consent tokens to verify permissions:

#### 1. Playback Operations
```kotlin
/**
 * Starts playback of a track with position (requires playback consent)
 * @throws MusicPlayerError if consent verification fails
 */
fun playTrack(trackId: UUID, positionMs: Int?): PlaySession {
    return nativePlayTrack(trackId.toString(), positionMs ?: 0)
}
```
#### 2. Recommendation Requests
```kotlin
/**
 * Gets personalized recommendations (requires recommendations consent)
 * @param userId User identifier
 * @param consentToken Token verifying recommendations consent
 * @return List of recommended tracks
 * @throws MusicPlayerError if consent verification fails
 */
fun getRecommendations(userId: UUID, consentToken: String): List<Track> {
    return nativeGetRecommendations(userId.toString(), consentToken)
}
```
```

#### 3. Social Interactions
```kotlin
/**
 * Likes a track (requires social consent)
 * @return true if successful, false otherwise
 * @throws MusicPlayerError if consent verification fails
 */
fun likeTrack(userId: UUID, trackId: UUID, consentToken: String): Boolean {
    return nativeLikeTrack(userId.toString(), trackId.toString(), consentToken)
}
```

```kotlin
/**
 * Comments on a track (requires social consent)
 * @return true if successful, false otherwise
 * @throws MusicPlayerError if consent verification fails
 */
fun commentOnTrack(userId: UUID, trackId: UUID, comment: String, consentToken: String): Boolean {
    return nativeCommentOnTrack(userId.toString(), trackId.toString(), comment, consentToken)
#### 4. Offline Downloads
```kotlin
/**
 * Downloads a track for offline use (requires offline_download consent)
 * @param includeWaveform Whether to include waveform data
 * @param consentToken Token verifying offline download consent
 * @return Download status object
 * @throws MusicPlayerError if consent verification fails
 */
fun downloadTrack(trackId: UUID, includeWaveform: Boolean, consentToken: String): DownloadStatus {
    return nativeDownloadTrack(trackId.toString(), includeWaveform, consentToken)
}
```
}
```

### Consent Management Methods

```kotlin
/**
 * Verifies if a specific consent type is valid
 * @return ConsentStatus object with current status
 */
fun verifyConsent(userId: UUID, consentType: String): ConsentStatus {
    return nativeVerifyConsent(userId.toString(), consentType)
}

/**
 * Requests user consent for a specific operation
 * @return Result of consent request
 */
fun requestConsent(userId: UUID, consentType: String): ConsentRequestResult {
    return nativeRequestConsent(userId.toString(), consentType)
}
### Error Handling for Privacy Operations

All privacy-sensitive operations must handle the following error cases:

```kotlin
try {
    val recommendations = musicPlayerBridge.getRecommendations(
        userId = currentUser.id,
        consentToken = currentUser.consentTokens["recommendations"] ?: ""
    )
    // Process recommendations
} catch (e: MusicPlayerError) {
    when {
        e.message?.contains("Consent renewal required") == true -> {
            // Handle expired consent (show renewal UI)
            showConsentRenewalDialog(ConsentType.RECOMMENDATIONS)
        }
        e.message?.contains("Consent required") == true || e.message?.contains("Consent denied") == true -> {
            // Handle missing consent (show request UI)
            showConsentRequestDialog(ConsentType.RECOMMENDATIONS)
        }
        else -> {
            // Handle other privacy errors
            showError("Privacy error: ${e.message}")
        }
    }
}
```
}
```

## 8. Troubleshooting Common JNI Issues

- **`UnsatisfiedLinkError`**:
  - **Cause:** The native library (`libcpc_core.so`) is not loaded correctly, or the JNI function signature in Kotlin does not exactly match the mangled name in Rust.
  - **Solution:**
    - Verify that `System.loadLibrary("cpc_core")` is called.
    - Double-check the function names, parameter types, and return types in both the `NativeBridge.kt` file and the Rust `#[no_mangle]` functions. The path in the function name (`Java_com_cpc_...`) must match the package and class name.

- **JNI Signature Mismatches:**
  - **Cause:** The Java types in the Kotlin `external fun` do not map to the correct JNI types in the Rust function signature (e.g., `JString` vs. `JObject`).
  - **Solution:** Use the `javap -s` command on the compiled Java class to inspect the expected JNI signatures and ensure the Rust code matches.

- **Incorrect Data Marshaling:**
  - **Cause:** `serde_json` fails to parse a string, or byte buffers are misinterpreted.
  - **Solution:** Add robust error handling and logging around all `env.get_string`, `serde_json::from_str`, and buffer manipulation calls to catch malformed data.

For information about our Android integration architecture for CPC modules, see [Android Architecture](android_architecture.md).