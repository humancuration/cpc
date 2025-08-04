use criterion::{criterion_group, criterion_main, Criterion, black_box, Throughput};
use uuid::Uuid;
use std::time::Duration;

fn setup_1000_clips_model() -> video_editor_core::ui::timeline::TimelineModel {
    use video_editor_core::ui::timeline::*;
    use video_editor_core::engine::composition::Composition;

    let mut tm = TimelineModel::new();
    let mut comp = Composition::new("Bench");
    // Add tracks and 1000 clips spread across 5 tracks
    for t in 0..5 {
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
    let mut id_counter = 1u64;
    let clip_len = 1000u64; // ms
    for track in &mut comp.video_tracks {
        for i in 0..200 {
            let start = (i as u64) * 500; // 50% overlap
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
    tm
}

/// Create a timeline model with 5000 clips for stress testing
fn setup_5000_clips_model() -> video_editor_core::ui::timeline::TimelineModel {
    use video_editor_core::ui::timeline::*;
    use video_editor_core::engine::composition::Composition;

    let mut tm = TimelineModel::new();
    let mut comp = Composition::new("StressBench");
    // Add tracks and 5000 clips spread across 10 tracks
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
    tm
}
/// Benchmark timeline seek performance with 1000 clips
fn bench_1000_clips(c: &mut Criterion) {
    let mut group = c.benchmark_group("timeline_seek");
    group.throughput(Throughput::Elements(625)); // 10,000ms / 16ms = 625 seek operations
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_function("1000_clips", |b| {
        let mut tm = setup_1000_clips_model();
        b.iter(|| {
            // Seek through ~10s range at 60fps granularity and run lookups
            for t in (0..10_000u64).step_by(16) {
                tm.cursor_position = t;
                let _ = tm.get_value_at_time(
                    1,
                    video_editor_core::engine::composition::PropertyType::Opacity,
                    t,
                );
            }
            black_box(&tm);
        })
    });
    
    group.bench_function("5000_clips", |b| {
        let mut tm = setup_5000_clips_model();
        b.iter(|| {
            // Seek through ~10s range at 60fps granularity and run lookups
            for t in (0..10_000u64).step_by(16) {
                tm.cursor_position = t;
                let _ = tm.get_value_at_time(
                    1,
                    video_editor_core::engine::composition::PropertyType::Opacity,
                    t,
                );
            }
            black_box(&tm);
        })
    });
    
    group.finish();
}

/// Benchmark memory usage with different clip counts
fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("timeline_memory");
    
    group.bench_function("1000_clips_model_size", |b| {
        b.iter(|| {
            let tm = setup_1000_clips_model();
            black_box(tm.compositions.len());
        })
    });
    
    group.bench_function("5000_clips_model_size", |b| {
        b.iter(|| {
            let tm = setup_5000_clips_model();
            black_box(tm.compositions.len());
        })
    });
    
    group.finish();
}
criterion_group!(benches, bench_1000_clips, bench_memory_usage);
criterion_main!(benches);