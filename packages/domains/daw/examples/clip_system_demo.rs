//! Demonstrates the new clip system functionality
use apps_daw::domain::models::{Track, Clip, TrackType, Project};
use std::path::PathBuf;

fn main() {
    println!("=== DAW Clip System Demo ===\n");

    // Create a new project
    let mut project = Project::new("Demo Project".to_string(), 44100);

    // Add different types of tracks
    let audio_track = project.add_audio_track("Drums".to_string());
    let midi_track = project.add_midi_track("Synth Lead".to_string());
    let _bus_track = project.add_bus_track("Master Bus".to_string());

    println!("Created project with {} tracks", project.tracks.len());

    // Add audio clips
    let audio_clip = Clip::new_audio(
        "Kick Pattern".to_string(),
        0,
        44100, // 1 second at 44.1kHz
        PathBuf::from("samples/kick.wav"),
        44100,
        2,
        "wav".to_string(),
    );

    audio_track.add_clip(audio_clip).unwrap();
    println!("Added audio clip to drums track");

    // Add MIDI clips
    let mut midi_clip = Clip::new_midi("Melody".to_string(), 44100, 88200); // 2 seconds
    if let apps_daw::domain::models::ClipType::Midi(ref mut midi_data) = midi_clip.clip_type {
        // Add some MIDI notes
        midi_data.notes.push(apps_daw::domain::models::MidiNote {
            pitch: 60, // Middle C
            velocity: 100,
            start_time: 0,
            duration: 22050, // 0.5 seconds
            channel: 0,
        });
        midi_data.notes.push(apps_daw::domain::models::MidiNote {
            pitch: 64, // E
            velocity: 90,
            start_time: 22050,
            duration: 22050,
            channel: 0,
        });
    }

    midi_track.add_clip(midi_clip).unwrap();
    println!("Added MIDI clip with 2 notes to synth track");

    // Demonstrate clip operations
    println!("\nTrack durations:");
    for track in &project.tracks {
        println!("  {}: {} samples", track.name, track.total_duration());
    }

    // Show all clips in a range
    let clips_in_range = project.get_all_clips_in_range(0, 44100);
    println!("\nClips in first second: {}", clips_in_range.len());

    // Demonstrate clip movement
    let clip_id = midi_track.clips[0].id;
    midi_track.move_clip(clip_id, 88200).unwrap();
    println!("Moved MIDI clip to start at 2 seconds");

    println!("\nFinal track durations:");
    for track in &project.tracks {
        println!("  {}: {} samples", track.name, track.total_duration());
    }

    println!("\nProject total duration: {} samples", project.total_duration());
}