# DAW Module Architecture

## Overview
Digital Audio Workstation application supporting multi-track editing, mixing, mastering, and remixing. Follows hexagonal architecture with vertical slices per feature.

## Module Structure
```
apps/daw/
├── Cargo.toml
└── src/
    ├── lib.rs                # Module exports
    ├── domain/               # Core business logic
    │   ├── models/           # Domain entities
    │   │   ├── track.rs
    │   │   ├── project.rs
    │   │   ├── effect.rs
    │   │   └── mix.rs
    │   ├── services/         # Domain services
    │   │   ├── audio_engine.rs
    │   │   ├── mixer.rs
    │   │   └── project_manager.rs
    │   └── ports/            # Interfaces for external systems
    │       ├── audio_processing.rs
    │       ├── collaboration.rs
    │       └── persistence.rs
    ├── infrastructure/       # External implementations
    │   ├── audio/
    │   │   ├── rodio_engine.rs
    │   ├── collaboration/
    │   │   └── p2panda_adapter.rs
    │   └── persistence/
    │       └── sql_repository.rs
    └── web/                  # Interface adapters
        ├── graphql/
        │   ├── schema.rs
        │   ├── mutations.rs
        │   ├── queries.rs
        │   └── subscriptions.rs
        └── module.rs         # Axum router setup
```

## Core Domain Models
```rust
// track.rs
pub struct Track {
    pub id: Uuid,
    pub name: String,
    pub clips: Vec<AudioClip>,
    pub volume: f32,
    pub pan: f32,
    pub effects: Vec<EffectInstance>,
}

// project.rs
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub tracks: Vec<Track>,
    pub bpm: u32,
    pub time_signature: (u8, u8),
}

// effect.rs
pub enum EffectType {
    Reverb,
    Equalizer,
    Compressor,
    // ... others
}

pub struct EffectInstance {
    pub effect_type: EffectType,
    pub parameters: HashMap<String, f32>,
}
```

## Audio Processing Service
- **RodioEngine** (domain service):
  - Real-time audio processing
  - Mixing multiple tracks
  - Applying effects chain

  - Encoding/decoding AV1/Opus/WebM
  - File format conversion
  - Audio analysis

```rust
// ports/audio_processing.rs
pub trait AudioProcessor {
    fn mix_tracks(tracks: &[Track]) -> Result<AudioBuffer, AudioError>;
    fn apply_effect(buffer: AudioBuffer, effect: &EffectInstance) -> Result<AudioBuffer, AudioError>;
}

// infrastructure/audio/rodio_engine.rs
impl AudioProcessor for RodioEngine {
    // Actual rodio implementation
}
```

## p2panda Collaboration
- **P2PandaAdapter** implements Collaboration port
- Operations:
  - Project sharing via p2panda documents
  - Real-time collaboration using CRDTs
  - Version history and conflict resolution
- Schema:
  - Project → p2panda collection
  - Track → p2panda document
  - Operations as p2panda entries

## GraphQL API Design
### Mutations
```graphql
type Mutation {
  createProject(name: String!): Project!
  addTrack(projectId: ID!, name: String!): Track!
  addAudioClip(trackId: ID!, file: Upload!): AudioClip!
  applyEffect(trackId: ID!, effectType: EffectType!, params: [EffectParam!]!): EffectInstance!
  renderProject(projectId: ID!, format: AudioFormat!): RenderJob!
}
```

### Subscriptions
```graphql
type Subscription {
  projectUpdated(projectId: ID!): ProjectUpdate!
  renderProgress(jobId: ID!): RenderProgress!
}
```

## UI Component Requirements (Yew)
1. **Project Explorer** - Tree view of projects
2. **Timeline View** - Multi-track arrangement
3. **Mixer Console** - Track controls (volume, pan, effects)
4. **Effect Rack** - Effect parameter controls
5. **Visualizer** - Winamp-style audio visualization
6. **Transport Controls** - Play, stop, record, loop
7. **Export Dialog** - Render settings and progress

## Integration Strategy
1. **Audio Stack**:
   - Use rodio for real-time processing
   - Plotters for visualization
   
2. **Collaboration**:
   - Map domain models to p2panda schemas
   - Use GraphQL subscriptions for real-time updates
   - Conflict resolution through operational transforms

3. **Performance**:
   - Offload heavy processing to Web Workers
   - Use wasm-bindgen for JS-Rust interop
   - Implement incremental rendering

## Next Steps
1. Implement core domain models
2. Create rodio audio engine
3. Set up p2panda schemas
4. Design GraphQL API endpoints
5. Build Yew UI components