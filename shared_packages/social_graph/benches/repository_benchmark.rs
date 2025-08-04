//! Benchmark tests for the social_graph repository

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use social_graph::{
    Relationship, RelationshipType,
    InMemoryRelationshipRepository, RelationshipRepository
};
use uuid::Uuid;
use tokio::runtime::Runtime;

fn benchmark_create_relationship(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let repo = InMemoryRelationshipRepository::new();
    
    c.bench_function("create_relationship", |b| {
        b.iter(|| {
            let user1_id = Uuid::new_v4();
            let user2_id = Uuid::new_v4();
            
            let relationship = Relationship::new(
                user1_id,
                user2_id,
                RelationshipType::Friend,
            );
            
            rt.block_on(async {
                black_box(repo.create_relationship(relationship).await.unwrap());
            });
        })
    });
}

fn benchmark_get_relationship(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let repo = InMemoryRelationshipRepository::new();
    
    // Create a relationship to retrieve
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    let relationship = Relationship::new(
        user1_id,
        user2_id,
        RelationshipType::Friend,
    );
    
    let saved_relationship = rt.block_on(repo.create_relationship(relationship)).unwrap();
    
    c.bench_function("get_relationship", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(repo.get_relationship(saved_relationship.id).await.unwrap());
            });
        })
    });
}

fn benchmark_get_friends(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let repo = InMemoryRelationshipRepository::new();
    
    // Create multiple relationships
    let user_id = Uuid::new_v4();
    for _ in 0..100 {
        let target_id = Uuid::new_v4();
        let relationship = Relationship::new(
            user_id,
            target_id,
            RelationshipType::Friend,
        );
        rt.block_on(repo.create_relationship(relationship)).unwrap();
    }
    
    c.bench_function("get_friends_100", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(repo.get_friends(user_id).await.unwrap());
            });
        })
    });
}

criterion_group!(
    benches,
    benchmark_create_relationship,
    benchmark_get_relationship,
    benchmark_get_friends
);
criterion_main!(benches);