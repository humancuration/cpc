use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shtairir::parse_script;

fn benchmark_simple_script_parsing(c: &mut Criterion) {
    let script = r#"bevy:create_entity()"#;
    
    c.bench_function("parse_simple_script", |b| {
        b.iter(|| {
            let _ = parse_script(black_box(script));
        })
    });
}

fn benchmark_complex_script_parsing(c: &mut Criterion) {
    let script = r#"
// Complex workflow
ffmpeg:convert("input.mp4", "output.webm")
ffmpeg:extract_audio("input.mp4", "audio.opus")
bevy:create_entity()
bevy:add_component("camera", "Camera")
bevy:add_component("light", "Light")
redis:set("video:input", "input.mp4")
redis:set("video:output", "output.webm")
"#;
    
    c.bench_function("parse_complex_script", |b| {
        b.iter(|| {
            let _ = parse_script(black_box(script));
        })
    });
}

criterion_group!(benches, benchmark_simple_script_parsing, benchmark_complex_script_parsing);
criterion_main!(benches);