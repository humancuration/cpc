# Cooperative Peer Cloud (CPC)

Social media and useful apps run on a p2p network.

No license is offered with this software, the license is pending. It will be a "coopyleft" license aimed at use within the federation.

# Core Technologies
bevy = "0.16.1"
p2panda = "0.4.0"  # P2P implementation on hold as noted
p2panda-auth = "0.4.0"
p2panda-blobs = "0.4.0"
p2panda-core = "0.4.0"
p2panda-discovery = "0.4.0"
p2panda-encryption = "0.4.0"
p2panda-net = "0.4.0"
p2panda-store = "0.4.0"
p2panda-stream = "0.4.0"
p2panda-sync = "0.4.0"

# UI and Desktop Framework
tauri = "2.7.0"
yew = { version = "0.21.0", features = ["csr"] }
yew-router = "0.18.0"
stylist = { version = "0.13.0", features = ["yew_integration"] }
wry = "0.52.1"

# Graphics and Rendering
glow = "0.16.0"
ash = "0.38.0"

# Web Assembly
wasm-bindgen = "0.2.100"
wasm-bindgen-futures = "0.4.50"
web-sys = "0.3.77"
js-sys = "0.3.77"
gloo-timers = { version = "0.3.0", features = ["futures"] }

# Data Visualization
plotters = "0.3.7"

# Web Server Framework
axum = "0.8.4"

# Database
sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "sqlite", "chrono", "uuid", "json"] }
sqlx-cli = "0.8.6"
tokio-postgres = "0.7.13"
diesel = "2.2.12"
sled = "0.34.7"

# Redis and Connection Pooling
redis = "0.32.4"
bb8 = "0.9.0"
bb8-redis = "0.24.0"

# Logging
tracing = "0.1.41"
tracing-subscriber = "0.3.18"

# Media Processing (royalty-free codecs only: AV1, Opus, WebM, etc)
rodio = "0.21"
pdf = "0.9.0"
gstreamer = "0.24.0"

# GraphQL for Public API
async-graphql = "7.0.17"
async-graphql-axum = "7.0.17"
graphql-parser = "0.4.1"
graphql_client = "0.14.0"
graphql_query_derive = "0.14.0"
graphql_client_codegen = "0.14.0"

# gRPC for Internal API (version 1.73.1 maps to tonic)
tonic = "0.14.0"  # Latest stable version for gRPC
prost = "0.14.1"
prost-types = "0.14.1"

# Parser Generation
pest = "2.8.1"
pest_derive = "2.8.1"
pest_meta = "2.8.1"
pest_generator = "2.8.1"

# Common dependencies
tokio = { version = "1.47.0", features = ["full"] }
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.141"
chrono = { version = "0.4.41", features = ["serde"] }
uuid = { version = "1.17.0", features = ["v4", "serde"] }
anyhow = "1.0.98"
thiserror = "2.0.12"
async-trait = "0.1.88"
rust_decimal = "1.37.2"
rust_decimal_macros = "1.37.2"
async-stream = "0.3.6"
futures-util = "0.3.31"
log = "0.4.27"
wasm-logger = "0.2.0"
rand = "0.8.5"

# WebSocket support
tokio-tungstenite = "0.20.1"

# Authentication and Security
oauth2 = "5.0.0"
jsonwebtoken = "9.3.1"
argon2 = "0.5.3"
aes-gcm = "0.10.3"
chacha20poly1305 = "0.10.1"
rustls = "0.23.31"
ed25519-dalek = "2.2.0"