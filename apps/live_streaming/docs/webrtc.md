# WebRTC Integration

This document describes how WebRTC is integrated into the Live Streaming module for peer-to-peer streaming.

## Overview

WebRTC (Web Real-Time Communication) is used to enable real-time streaming between broadcasters and viewers. It provides low-latency communication and works directly in web browsers without plugins.

## Architecture

The WebRTC implementation consists of:

1. **Broadcaster Side**: Captures and sends audio/video streams
2. **Viewer Side**: Receives and displays audio/video streams
3. **Signaling Server**: Facilitates connection establishment
4. **STUN/TURN Servers**: Handle NAT traversal

## Implementation

### Broadcaster

Location: `src/streaming/broadcaster.rs`

The broadcaster component handles:

- Stream creation and management
- Media capture and encoding
- Peer connection establishment
- Stream distribution to viewers

Key methods:
- `start_stream()`: Begins a new stream
- `stop_stream()`: Ends an active stream
- `update_viewer_count()`: Updates viewer statistics

### Viewer

Location: `src/streaming/viewer.rs`

The viewer component handles:

- Stream discovery and connection
- Media decoding and playback
- User interaction (mute, volume control)

Key methods:
- `start_watching()`: Connects to a stream
- `stop_watching()`: Disconnects from a stream
- `toggle_mute()`: Toggles audio mute
- `set_volume()`: Sets audio volume

## Signaling

Signaling is the process of coordinating communication between peers. In this implementation:

1. Broadcaster creates a stream and registers with the signaling server
2. Viewer requests to watch a stream
3. Signaling server facilitates exchange of ICE candidates and SDP offers/answers
4. Direct peer-to-peer connection is established

## Media Processing

### Codecs

The platform uses royalty-free codecs:

- **Video**: AV1 for efficient compression
- **Audio**: Opus for high-quality audio

### Transcoding

Location: `src/media_processing/transcoder.rs`

For compatibility with different devices and network conditions:

- Streams are transcoded to multiple quality levels
- Adaptive bitrate streaming adjusts quality based on viewer's connection
- WebM container format is used for browser compatibility

## Integration with wry

The `wry` crate is used for WebView integration in the Tauri application. It provides:

- Web browser engine for rendering the UI
- JavaScript execution environment
- Access to WebRTC APIs

## Security

WebRTC security features include:

- DTLS-SRTP encryption for media streams
- Identity verification through signaling
- Consent freshness to prevent denial-of-service attacks

## Performance Considerations

### Latency

WebRTC provides sub-second latency, which is crucial for interactive streaming:

- Connection establishment: ~100-500ms
- Media transmission: <100ms

### Bandwidth

Adaptive streaming optimizes bandwidth usage:

- Automatic quality adjustment based on network conditions
- Congestion control to prevent network overload

### Scalability

For large-scale streaming:

- Peer-to-peer distribution reduces server load
- Selective forwarding units (SFUs) can be used for large audiences
- CDN integration for static content

## API

### Stream Management

```rust
// Start a new stream
let stream = broadcaster.start_stream(
    channel_id,
    stream_key,
    title,
    category,
    metadata
);

// Stop a stream
broadcaster.stop_stream(&stream_key);

// Viewer connects to stream
viewer.start_watching(stream_id);
```

### Media Processing

```rust
// Create transcoding job
let job = transcoder.start_transcoding_job(
    stream_key,
    input_format,
    output_format
);

// Update job status
transcoder.update_job_status(job_id, status, progress);
```

## Testing

WebRTC functionality can be tested using:

- Unit tests for individual components
- Integration tests for end-to-end streaming
- Browser compatibility testing
- Network condition simulation

## Troubleshooting

Common WebRTC issues and solutions:

### Connection Failure

- Verify STUN/TURN server configuration
- Check firewall and NAT settings
- Ensure proper signaling implementation

### Poor Quality

- Check network bandwidth
- Verify codec settings
- Monitor CPU usage for encoding/decoding

### Audio/Video Sync

- Adjust buffer sizes
- Check clock drift between peers
- Verify timestamp handling in media processing

## Future Enhancements

Planned improvements:

- SVC (Scalable Video Coding) support
- Improved error recovery mechanisms
- Enhanced analytics and monitoring
- Integration with p2panda network for decentralized streaming