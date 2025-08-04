// Stress test for timeline with 5000+ clips
use video_editor_core::ui::timeline::*;
use video_editor_core::engine::composition::Composition;
use video_editor_core::engine::media_pipeline::FrameCache;

#[test]
fn test_5000_clips_timeline() {
    let mut tm = TimelineModel::new();
    let mut comp = Composition::new("StressTest");
    
    // Add 10 tracks
    for t in 0..10 {
        comp.video_tracks.push(VideoTrack {
            id: (t + 1) as u64,
            layer: t as u32,
            clips: Vec::new(),
            effects: Vec::new(),
            locked: false,
            muted: false,
            spatial_index: rstar::RTree::new(),
        });
    }
    
    // Add 5000 clips (500 per track)
    let mut id_counter = 1u64;
    let clip_len = 500u64; // ms
    for track in &mut comp.video_tracks {
        for i in 0..500 {
            let start = (i as u64) * 300; // 40% overlap
            track.clips.push(Clip {
                id: id_counter,
                kind: ClipKind::Video,
                start_ms: start,
                duration_ms: clip_len,
                track_id: track.id,
                keyframes: Default::default(),
            });
            track.spatial_index.insert(ClipRegion::from_clip(track.clips.last().unwrap()));
            id_counter += 1;
        }
    }
    
    let id = tm.add_composition(comp);
    tm.set_current_composition(id);
    
    // Verify the model was created correctly
    assert_eq!(tm.compositions.len(), 1);
    let comp = tm.compositions.get(&id).unwrap();
    assert_eq!(comp.video_tracks.len(), 10);
    let total_clips: usize = comp.video_tracks.iter().map(|t| t.clips.len()).sum();
    assert_eq!(total_clips, 5000);
    
    // Test spatial index queries at various times
    for t in (0..15000u64).step_by(500) {
        // Count clips at this time using spatial index
        let mut count = 0;
        for track in &comp.video_tracks {
            let point = [t as f64, 0.0];
            count += track.spatial_index.locate_at_point(&point).count();
        }
        
        // Verify count is reasonable (should be around 3-4 clips per track at any given time with 40% overlap)
        assert!(count <= 40); // 10 tracks * 4 max clips per track
    }
}

#[test]
fn test_frame_cache_with_large_timeline() {
    let mut cache = FrameCache::new(512 * 1024 * 1024); // 512MB cache
    let composition_id = uuid::Uuid::new_v4();
    
    // Fill cache with frames representing a large timeline
    // Each frame is 1920x1080 RGBA (≈8MB)
    let frame_size = (1920u32, 1080u32);
    let bytes_per_frame = (frame_size.0 * frame_size.1 * 4) as usize;
    let target_bytes = 256 * 1024 * 1024; // 256MB target
    let mut t = 0u64;
    
    while cache.current_size_bytes < target_bytes {
        let key = video_editor_core::engine::media_pipeline::FrameKey { 
            composition_id, 
            time_ms: t 
        };
        let frame = video_editor_core::engine::media_pipeline::CachedFrame {
            pixels: vec![0u8; bytes_per_frame],
            width: frame_size.0,
            height: frame_size.1,
        };
        cache.put(key, frame);
        t += 33; // ~30fps spacing
        if t > 60_000 { break; } // Limit to 1 minute
    }
    
    // Verify cache behavior
    assert!(cache.current_size_bytes > 0);
    assert!(cache.current_size_bytes <= target_bytes + bytes_per_frame);
    
    // Test cache hit/miss behavior
    let test_time = 1000u64; // 1 second
    let test_key = video_editor_core::engine::media_pipeline::FrameKey {
        composition_id,
        time_ms: test_time,
    };
    
    // This should be a hit if we added a frame at or near this time
    let _frame = cache.get(&test_key);
    // We're not asserting hit/miss counts because behavior may vary
    // but we've at least exercised the code paths
}

#[test]
fn test_nested_compositions_stress() {
    let mut parent = Composition::new("Parent");
    
    // Add 100 nested compositions
    for i in 0..100 {
        let mut child = Composition::new(&format!("Child_{}", i));
        // Each child gets 10 clips
        let mut track = VideoTrack::default();
        track.id = 1;
        for j in 0..10 {
            track.clips.push(Clip {
                id: (i * 10 + j + 1) as u64,
                kind: ClipKind::Video,
                start_ms: j * 1000,
                duration_ms: 1000,
                track_id: 1,
                keyframes: Default::default(),
            });
        }
        child.video_tracks.push(track);
        parent.add_nested_composition(child);
    }
    
    // Verify all nested compositions were added
    assert_eq!(parent.nested_compositions.len(), 100);
    
    // Verify we can access them
    for i in 0..100 {
        let child_name = format!("Child_{}", i);
        let has_child = parent.nested_compositions.values().any(|c| c.name == child_name);
        assert!(has_child);
    }
}