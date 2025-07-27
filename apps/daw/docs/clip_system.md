# DAW Clip System Documentation

## Overview

The enhanced Track model now supports a sophisticated clip-based system that replaces the simple audio_data approach. This system supports both audio and MIDI clips with comprehensive editing capabilities.

## Core Components

### Track Types
- **Audio**: For audio clips with file references
- **MIDI**: For MIDI clips with note data and instrument plugins
- **Bus**: For routing and mixing (cannot contain clips directly)

### Clip Types
- **AudioClip**: References audio files with fade curves
- **MidiClip**: Contains MIDI note data with instrument plugin references

## Data Structures

### Clip
- `id`: UUID for unique identification
- `name`: Display name for the clip
- `start_position`: Position in samples from track start
- `duration`: Length in samples
- `fade_in`/`fade_out`: Fade curves for audio clips
- `clip_type`: Enum specifying Audio or MIDI
- `muted`: Whether the clip should be played
- `color`: Optional color for visual organization

### AudioClipData
- `file_path`: Path to the audio file
- `sample_rate`: Audio file sample rate
- `channels`: Number of audio channels
- `format`: File format (wav, flac, mp3, etc.)
- `cached_samples`: Optional cached audio data for performance

### MidiClipData
- `notes`: Vector of MIDI note events
- `instrument_plugin_id`: Reference to the instrument plugin
- `transpose`: Transposition in semitones
- `velocity_scale`: Velocity scaling factor

### MIDI Note
- `pitch`: MIDI note number (0-127)
- `velocity`: MIDI velocity (0-127)
- `start_time`: Note start in samples
- `duration`: Note duration in samples
- `channel`: MIDI channel (0-15)

## Available Operations

### Track Operations
- `add_clip()`: Add a clip to a track (type-checked)
- `remove_clip()`: Remove a clip by ID
- `get_clip()`: Get a clip reference by ID
- `move_clip()`: Move a clip to a new position
- `split_clip()`: Split a clip at a specified position
- `trim_clip_start()`: Trim the start of a clip
- `trim_clip_end()`: Trim the end of a clip
- `duplicate_clip()`: Create a copy of a clip
- `get_clips_in_range()`: Find clips within a time range
- `get_clips_at_position()`: Find clips at a specific position
- `has_overlapping_clips()`: Check for clip overlaps

### Project Operations
- `add_audio_track()`: Add a new audio track
- `add_midi_track()`: Add a new MIDI track
- `add_bus_track()`: Add a new bus track
- `total_duration()`: Get project duration
- `get_all_clips_in_range()`: Find clips across all tracks
- `find_clip()`: Find a clip by ID across all tracks

## Usage Examples

### Creating a Project with Clips
```rust
let mut project = Project::new("My Song".to_string(), 44100);

// Add tracks
let drums = project.add_audio_track("Drums".to_string());
let synth = project.add_midi_track("Synth".to_string());

// Add audio clip
let audio_clip = Clip::new_audio(
    "Kick Pattern".to_string(),
    0,
    44100,
    PathBuf::from("kick.wav"),
    44100,
    2,
    "wav".to_string(),
);
drums.add_clip(audio_clip).unwrap();

// Add MIDI clip
let mut midi_clip = Clip::new_midi("Melody".to_string(), 0, 88200);
if let ClipType::Midi(midi_data) = &mut midi_clip.clip_type {
    midi_data.notes.push(MidiNote {
        pitch: 60,
        velocity: 100,
        start_time: 0,
        duration: 22050,
        channel: 0,
    });
}
synth.add_clip(midi_clip).unwrap();
```

### Editing Clips
```rust
// Split a clip
track.split_clip(clip_id, 1000).unwrap();

// Move a clip
track.move_clip(clip_id, 2000).unwrap();

// Trim clip start
track.trim_clip_start(clip_id, 500).unwrap();

// Duplicate a clip
track.duplicate_clip(clip_id, 4000).unwrap();
```

## Serialization

All structures implement `Serialize` and `Deserialize` for JSON persistence:
- Compatible with existing project persistence
- Supports cross-platform serialization
- Handles file path references appropriately
- Maintains backward compatibility

## Performance Considerations

- Audio data is cached using `Arc<Vec<f32>>` for efficient sharing
- Large audio files are referenced by path rather than stored in memory
- MIDI data is stored efficiently as note events
- Clip operations are optimized for real-time use

## Error Handling

All clip operations return `Result<T, String>` with descriptive error messages:
- Type mismatches between track and clip types
- Invalid clip positions or durations
- Missing clips for operations
- Overlapping clip detection

## Future Enhancements

The clip system is designed to support:
- Audio warping and time-stretching
- MIDI quantization
- Clip grouping and linking
- Loop regions
- Automation clips
- Video synchronization