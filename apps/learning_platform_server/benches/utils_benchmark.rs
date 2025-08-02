use criterion::{criterion_group, criterion_main, Criterion};
use learning_platform_server::utils::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_valid_uuid", |b| {
        b.iter(|| {
            let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
            parse_uuid(uuid_str).unwrap();
        })
    });
    
    c.bench_function("validate_not_empty_valid", |b| {
        b.iter(|| {
            validate_not_empty("test", "field").unwrap();
        })
    });
    
    c.bench_function("validate_not_empty_invalid", |b| {
        b.iter(|| {
            let _ = validate_not_empty("", "field");
        })
    });
    
    c.bench_function("validate_email_valid", |b| {
        b.iter(|| {
            validate_email("test@example.com").unwrap();
        })
    });
    
    c.bench_function("validate_email_invalid", |b| {
        b.iter(|| {
            let _ = validate_email("invalid-email");
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);