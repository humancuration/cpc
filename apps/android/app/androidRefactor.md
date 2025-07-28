Android app (we will refactor to use as much of the shared Rust code, bevy, tauri, axum as possible)
apps/cpc-platform/android/WheresThisFrom - Old code
apps/cpc-platform/android/app/ -New code

Backend Framework
	Axum: The Android app will communicate exclusively with the new unified Axum backend via the Rust core. This is the foundational change.
UI Framework
	Yew + Tauri + Bevy:  Primary UI: Rebuild screens (Feed, UBI, Governance, Settings) in Yew inside cpc-platform/src.  Graphics: Use Bevy for the cpc-studio viewport, rendering to an Android SurfaceView.
Networking Client
	Rust Core (hyper): All HTTP requests to the Axum backend will be made from Rust functions inside cpc-core. The Yew UI will call these functions using invoke. This centralizes all networking logic.
GraphQL Client
		Rust Core (graphql_client): The Rust core will handle all GraphQL queries and mutations. The UI remains agnostic to the data fetching mechanism.
Real-time Comms	
Rust Core (tokio-tungstenite): The Rust core will manage the WebSocket connection to the Axum backend. It will listen for messages and forward them to the Yew UI as Tauri Events.
Local Database
Rust Core (rusqlite): Replace Room with rusqlite, an MIT-licensed, embedded SQL database for Rust. All database operations (CRUD, caching) will be handled in cpc-core, making the data layer cross-platform.
Authentication
	Axum Backend + Rust Core: The Axum server handles all JWT validation. The Rust core will manage storing the token securely on the device using a JNI call to Android's Keystore for maximum security.
Dependency Injection
	Tauri State Management: Hilt will only be needed for the minimal Android native shell. All Rust dependencies and shared state (like the database connection pool) will be managed by Tauri's state management system.
Image/Video Handling
		Hybrid Approach:  UI Images (Yew): Continue using a JavaScript library (like <img> tags or a Yew-native solution) for simple UI images (e.g., avatars). Bevy Textures: Use Rust's image crate (MIT/Apache-2.0) within cpc-core to load and manage textures for the Bevy engine.  Video/Codec Logic: Implement your custom, permissive video/audio processing solutions directly in Rust within cpc-core.
Async Programming
	Rust (tokio) + JS (async/await): Rust business logic will use tokio's async runtime. Yew UI will use standard JavaScript async/await to call Rust functions via Tauri's invoke, which is inherently asynchronous.
Data Serialization
	Rust (serde) + JSON: Communication between Rust and Yew will be via JSON. serde (Rust) and kotlinx.serialization (Kotlin) interoperate perfectly with JSON as the common format. Ensure consistent data models.
Date & Time
	Rust (chrono) + JS (Date): Standardize on Unix timestamps (integers) or ISO 8601 strings for data transfer to ensure compatibility between all layers.
