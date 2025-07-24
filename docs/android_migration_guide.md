# Android App Migration to Unified Tech Stack

## Overview
This document outlines the strategy for migrating our Android app from the current Kotlin-based architecture to our unified Rust-based tech stack.

## Architecture Comparison

### Current Architecture
```
Android Client (UI) → Core Android (Network) → Ktor Server (Business Logic)
```

### New Architecture
```
Yew UI → Tauri → Rust Core (Business Logic/Network) → Axum Backend
             │
             └── Bevy (Graphics)
```

## Phase Implementation Details

### Phase 1: Tauri/WebView Shell
**Tasks:**
1. Create `MainActivity.kt` with WebView component
2. Implement JNI methods for Android Keystore
3. Set up basic Tauri command pipeline

**File Changes:**
- `android/app/src/main/java/com/wtf/MainActivity.kt`
- `cpc-core/src/jni_bridge.rs`

### Phase 2: Database Migration
**Migration Strategy:**
1. Create equivalent Rust structs for Room entities
2. Implement rusqlite CRUD operations
3. Write data migration script

**Example Conversion:**
```kotlin
// Room Entity
@Entity
data class User(
    @PrimaryKey val id: String,
    val name: String
)

// Rust Equivalent
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
}
```

### Phase 3: Network Logic
**HTTP Stack:**
- Use `hyper` for HTTP requests
- `graphql-client` for GraphQL operations
- JWT handling via `jsonwebtoken` crate

**Authentication Flow:**
1. Yew UI triggers auth via Tauri command
2. Rust core handles login with Axum backend
3. Token stored via JNI Keystore bridge

### Phase 4: Yew UI
**Component Mapping:**
| Current Component      | New Location                        |
|------------------------|-------------------------------------|
| GovernanceScreen.kt    | src/components/governance/mod.rs    |
| UBICard.kt             | src/components/ubi/card.rs          |
| WalletViewModel.kt     | src/state/wallet.rs                 |

### Phase 5: Bevy Integration
**Bevy-Yew Bridge:**
1. Bevy renders to Android SurfaceView
2. Yew controls via Tauri events
3. Shared texture memory for performance

**File Structure:**
```
src-tauri/
└── src/
    ├── bevy_plugin.rs
    └── yew_integration.rs
```

## Risk Mitigation
1. **Legacy Feature Parity**:
   - Maintain feature flags during transition
   - Dual-write database during migration
   
2. **Performance Optimization**:
   - Profile critical paths with criterion
   - Implement Rust caching layer

3. **Error Handling**:
   - Unified error codes across layers
   - Sentry integration for crash reporting

   here is a detailed, feature-by-feature plan for replacing the Android-specific libraries with solutions from your Rust ecosystem.
High-Level Architectural Vision: Tauri on Android

The core idea is to transform your Android app into a lightweight native "shell" that hosts a Tauri WebView. This architecture allows you to build the majority of your UI and all of your business logic in a shared, cross-platform way.

    UI Layer: The primary UI will be built with Yew, running inside a WebView (Wry) managed by Tauri. This replaces most of the Jetpack Compose UI code.

    Business Logic & Networking: All application logic, data fetching, and state management will be implemented in cpc-core (Rust). The Yew UI will interact with this Rust backend via Tauri's invoke API (asynchronous function calls).

    Graphics-Intensive Views: For components that require high-performance graphics (like the studio viewport), Bevy will render to a native Android SurfaceView, which can be integrated into the native layout. Communication between Kotlin and the Bevy instance will happen via a JNI (Java Native Interface) bridge.

    Native Shell: The Kotlin code will be minimal, primarily responsible for setting up the Tauri Activity and managing the lifecycle of the Bevy SurfaceView.

Feature-by-Feature Replacement Plan

Here’s how each piece of the current Android stack can be replaced or refactored:
Feature / Library	Current (Android)	Proposed (Rust / Unified Stack)
Backend Framework	(Pointing to old Ktor/PDS)	Axum: The Android app will communicate exclusively with the new unified Axum backend via the Rust core. This is the foundational change.
UI Framework	Jetpack Compose	Yew + Tauri + Bevy: <br> • Primary UI: Rebuild screens (Feed, UBI, Governance, Settings) in Yew inside cpc-platform/src. <br> • Graphics: Use Bevy for the cpc-studio viewport, rendering to an Android SurfaceView.
Networking Client	Ktor Client	Rust Core (reqwest or hyper): All HTTP requests to the Axum backend will be made from Rust functions inside cpc-core. The Yew UI will call these functions using invoke. This centralizes all networking logic.
GraphQL Client	Apollo-Android	Rust Core (graphql_client or similar): The Rust core will handle all GraphQL queries and mutations. The UI remains agnostic to the data fetching mechanism.
Real-time Comms	Ktor WebSockets	Rust Core (tokio-tungstenite): The Rust core will manage the WebSocket connection to the Axum backend. It will listen for messages and forward them to the Yew UI as Tauri Events.
Local Database	Room	Rust Core (rusqlite): Replace Room with rusqlite, an MIT-licensed, embedded SQL database for Rust. All database operations (CRUD, caching) will be handled in cpc-core, making the data layer cross-platform.
Authentication	Ktor JWT Client	Axum Backend + Rust Core: The Axum server handles all JWT validation. The Rust core will manage storing the token securely on the device using a JNI call to Android's Keystore for maximum security.
Dependency Injection	Hilt (KSP)	Tauri State Management: Hilt will only be needed for the minimal Android native shell. All Rust dependencies and shared state (like the database connection pool) will be managed by Tauri's state management system.
Image/Video Handling	Coil-kt	Hybrid Approach: <br> • UI Images (Yew): Use standard HTML `<img>` tags or a Yew component for simple UI images (e.g., avatars), loaded via the Rust backend. <br> • Bevy Textures: Use Rust's image crate (MIT/Apache-2.0) within cpc-core to load and manage textures for the Bevy engine. <br> • Video/Codec Logic: Implement your custom, permissive video/audio processing solutions directly in Rust within cpc-core.
Async Programming	Kotlin Coroutines & Flow	Rust (tokio) + Yew (async components): <br> • Rust business logic will use tokio's async runtime. <br> • Yew UI will use async components and `wasm-bindgen-futures` to call Rust functions via Tauri's invoke, which is inherently asynchronous.
Data Serialization	Kotlinx Serialization	Rust (serde) + JSON: Communication between Rust and the Yew frontend will be via JSON. Serde is used on both sides for seamless serialization/deserialization.
Date & Time	kotlinx-datetime	Rust (chrono) + JS (Date): Standardize on Unix timestamps (integers) or ISO 8601 strings for data transfer to ensure compatibility between all layers.
Step-by-Step Refactoring Plan

    Finalize the Unified Axum Backend: Ensure the new backend crate has all the necessary GraphQL and REST endpoints to replace the old orchestrator, pds, and Ktor servers.

    Integrate Tauri into the Android Project:

        Follow the official Tauri documentation to set up a Tauri project within an existing Android application. This will involve creating a TauriActivity in Kotlin.

        Your cpc-platform/src-tauri directory will contain the Rust code that bridges the native app and the WebView.

    Build the Yew UI:

        In cpc-platform/src, create the Yew components for the main app screens (Feed, UBI, Governance, Settings, etc.). This UI will replace the Jetpack Compose UI.

    Create the Rust Bridge in cpc-core:

        Expose Rust functions to the frontend using the #[tauri::command] macro. These functions will contain all the business logic.

        Example (Data Fetching):
        Generated rust

      
// In cpc-core, exposed via cpc-platform/src-tauri/src/main.rs
#[tauri::command]
async fn get_timeline() -> Result<Vec<Post>, String> {
    // Use reqwest to call the Axum backend's /graphql endpoint
    // ...
}

    

IGNORE_WHEN_COPYING_START
Use code with caution. Rust
IGNORE_WHEN_COPYING_END

Example (Yew):
```rust
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::bindings::get_timeline; // Assuming generated bindings for Tauri commands

#[function_component(Timeline)]
fn timeline() -> Html {
    let posts = use_state(|| vec![]);
    {
        let posts = posts.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                if let Ok(fetched_posts) = get_timeline().await {
                    posts.set(fetched_posts);
                }
            });
            || ()
        }, ());
    }

    // ... render posts
    html! {}
}
```

    Port Networking Logic:

        Go through every Ktor Client call in the Android app.

        Create a corresponding async fn in Rust that performs the same network request using reqwest.

        Expose that Rust function as a Tauri command.

        Replace the original Kotlin call with an invoke call in your new Yew UI.

    Port Database Logic:

        Identify all Room DAO interfaces and entities.

        Recreate the tables and queries in Rust using rusqlite.

        Create a data access layer in cpc-core with functions like get_user_profile_from_db, cache_feed_items, etc.

        Expose these functions as Tauri commands.

    Integrate the Bevy View:

        In your Android TauriActivity layout, add a SurfaceView.

        Create a NativeBridge.kt file (similar to the one you already have) to make JNI calls.

        In cpc-core, write the JNI functions (Java_com_cpc_NativeBridge_...) that receive the SurfaceView handle and initialize the Bevy app to render into it. This keeps the high-performance graphics rendering native while the UI is in the WebView.

By following this plan, you will successfully migrate your Android app to your unified Rust stack, creating a truly cross-platform application with maximum code reuse and adherence to your licensing principles.