# Music Player Domain Models

This document describes the core domain models used in the music player module.

## Track

Represents a musical track with metadata.

```rust
pub struct Track {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub album_id: Option<Uuid>,
    pub title: String,
    pub duration_ms: u64,
    pub media_cid: String,  // Content ID for p2p storage
    pub waveform_data_cid: Option<String>, // For visualizer
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## TimestampedComment

A comment on a track at a specific timestamp.

```rust
pub struct TimestampedComment {
    pub id: Uuid,
    pub track_id: Uuid,
    pub user_id: Uuid,
    pub timestamp_ms: u64,  // Position in track where comment applies
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## TrackInteraction

Represents various interactions users can have with tracks.

```rust
pub enum TrackInteraction {
    Like { 
        track_id: Uuid, 
        user_id: Uuid,
        created_at: DateTime<Utc> 
    },
    Repost { 
        track_id: Uuid, 
        user_id: Uuid,
        comment: Option<String>, 
        created_at: DateTime<Utc> 
    },
    FollowArtist { 
        artist_id: Uuid, 
        follower_id: Uuid,
        created_at: DateTime<Utc> 
    },
}
```

## VisualizerPreset

Configuration for visualizer appearance and behavior.

```rust
pub struct VisualizerPreset {
    pub id: Uuid,
    pub name: String,
    pub config: serde_json::Value,  // Custom visualizer settings
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}
```

## Playlist

A collection of tracks in a specific order.

```rust
pub struct Playlist {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub track_positions: Vec<PlaylistTrack>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## PlaylistTrack

Represents a track's position within a playlist.

```rust
pub struct PlaylistTrack {
    pub track_id: Uuid,
    pub position: u32,
}
```

## WaveformData

Data structure for visualizer waveform representation.

```rust
pub struct WaveformData {
    pub sample_rate: u32,
    pub duration_ms: u64,
    pub amplitudes: Vec<f32>,
}
```

## FrequencyData

Data structure for frequency analysis visualization.

```rust
pub struct FrequencyData {
    pub sample_rate: u32,
    pub bands: Vec<String>,
    pub frames: Vec<FrequencyFrame>,
}

pub struct FrequencyFrame {
    pub timestamp_ms: u64,
    pub values: Vec<f32>,
}
```

## DownloadStatus

Represents the status of an offline download.

```rust
pub enum DownloadStatus {
    Pending,
    InProgress { progress: f32 },
    Completed,
    Failed { error: String },
}
```

## DownloadManifest

Metadata for offline content.

```rust
pub struct DownloadManifest {
    pub track_id: Uuid,
    pub track_cid: String,
    pub media_cid: String,
    pub waveform_cid: Option<String>,
    pub metadata: serde_json::Value,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
}
```

## Value Objects

### ColorHex

A validated hex color value.

```rust
pub struct ColorHex(String);
```

### ValidUrl

A validated URL.

```rust
pub struct ValidUrl(String);
```

### ContentId

A validated content identifier for p2p storage.

```rust
pub struct ContentId(String);