//! Performance benchmarks for the collaborative documentation system

use collaborative_docs::{
    crdt::CrdtDocument,
};
use serde_json::json;
use criterion::{criterion_group, criterion_main, Criterion};

/// Benchmark creating and modifying a CRDT document
fn bench_crdt_operations(c: &mut Criterion) {
    c.bench_function("crdt_create_and_modify", |b| {
        b.iter(|| {
            // Create a new CRDT document
            let mut doc = CrdtDocument::new("benchmark_actor".to_string());
            let root = doc.doc.get_object_root();
            
            // Perform a series of operations
            for i in 0..100 {
                let key = format!("key_{}", i);
                let value = json!({
                    "id": i,
                    "name": format!("item_{}", i),
                    "value": i * 2
                });
                
                // This will panic if it fails, which is what we want in benchmarks
                doc.put(&root, &key, value).unwrap();
            }
            
            // Retrieve some values
            for i in 0..10 {
                let key = format!("key_{}", i);
                let _value = doc.get(&root, &key).unwrap();
            }
        })
    });
}

/// Benchmark merging two CRDT documents
fn bench_crdt_merge(c: &mut Criterion) {
    c.bench_function("crdt_merge", |b| {
        b.iter(|| {
            // Create two documents
            let mut doc1 = CrdtDocument::new("actor1".to_string());
            let doc2 = CrdtDocument::new("actor2".to_string());
            let root1 = doc1.doc.get_object_root();
            let root2 = doc2.doc.get_object_root();
            
            // Make changes to both documents
            for i in 0..50 {
                let key = format!("key_{}", i);
                let value1 = json!({"source": "doc1", "value": i});
                let value2 = json!({"source": "doc2", "value": i * 2});
                
                doc1.put(&root1, &key, value1).unwrap();
                doc2.put(&root2, &key, value2).unwrap();
            }
            
            // Merge doc2 into doc1
            let heads1 = doc1.get_heads();
            let changes = doc2.get_changes(&heads1);
            doc1.apply_changes(&changes).unwrap();
        })
    });
}

/// Benchmark serialization and deserialization
fn bench_crdt_serialization(c: &mut Criterion) {
    // First create a document with substantial content
    let mut doc = CrdtDocument::new("serial_bench_actor".to_string());
    let root = doc.doc.get_object_root();
    
    for i in 0..100 {
        let key = format!("item_{}", i);
        let value = json!({
            "id": i,
            "data": "some text data for benchmarking purposes",
            "nested": {
                "field1": "value1",
                "field2": "value2",
                "array": (0..10).collect::<Vec<_>>()
            }
        });
        doc.put(&root, &key, value).unwrap();
    }
    
    c.bench_function("crdt_serialize", |b| {
        b.iter(|| {
            // Serialize the document
            let _data = doc.save().unwrap();
        })
    });
    
    let serialized_data = doc.save().unwrap();
    
    c.bench_function("crdt_deserialize", |b| {
        b.iter(|| {
            // Deserialize the document
            let _loaded_doc = CrdtDocument::load(&serialized_data, "deserial_bench_actor".to_string()).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_crdt_operations,
    bench_crdt_merge,
    bench_crdt_serialization
);

criterion_main!(benches);