//! Benchmarks for the consent manager performance targets.

use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain, ConsentProfile},
        audit::{Actor, AuditEvent},
        errors::ConsentError,
    },
    application::service::{ConsentService, ConsentStorage},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// In-memory storage implementation for benchmarking
struct InMemoryStorage {
    profiles: Mutex<HashMap<String, ConsentProfile>>,
    audit_events: Mutex<HashMap<String, Vec<AuditEvent>>>,
}

impl InMemoryStorage {
    fn new() -> Self {
        Self {
            profiles: Mutex::new(HashMap::new()),
            audit_events: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl ConsentStorage for InMemoryStorage {
    async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
        let key = format!("{}:{:?}", user_id, domain);
        let profiles = self.profiles.lock().unwrap();
        Ok(profiles.get(&key).cloned())
    }

    async fn save_consent_profile(&self, profile: &ConsentProfile) -> Result<(), ConsentError> {
        let key = format!("{}:{:?}", profile.user_id, profile.domain);
        let mut profiles = self.profiles.lock().unwrap();
        profiles.insert(key, profile.clone());
        Ok(())
    }

    async fn revoke_domain(&self, user_id: &str, domain: &Domain) -> Result<(), ConsentError> {
        let key = format!("{}:{:?}", user_id, domain);
        let mut profiles = self.profiles.lock().unwrap();
        profiles.remove(&key);
        Ok(())
    }

    async fn get_audit_events(&self, user_id: &str) -> Result<Vec<AuditEvent>, ConsentError> {
        let audit_events = self.audit_events.lock().unwrap();
        Ok(audit_events.get(user_id).cloned().unwrap_or_default())
    }

    async fn save_audit_event(&self, event: &AuditEvent) -> Result<(), ConsentError> {
        let mut audit_events = self.audit_events.lock().unwrap();
        audit_events.entry(event.user_id.clone()).or_insert_with(Vec::new).push(event.clone());
        Ok(())
    }
}

fn benchmark_consent_check(c: &mut Criterion) {
    let storage = Box::new(InMemoryStorage::new());
    let consent_service = ConsentService::new(storage);
    
    // Pre-populate with some data
    let user_id = "benchmark_user";
    let domain = Domain::FinancialData;
    let actor = Actor::System;
    
    // Set up initial consent
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        consent_service.update_consent_level(user_id, domain.clone(), DataSharingLevel::Standard, actor).await.unwrap();
    });
    
    let consent_service = Arc::new(consent_service);
    
    c.bench_function("consent_check", |b| {
        b.to_async(&rt).iter(|| async {
            consent_service.get_consent_level(user_id, domain.clone()).await.unwrap()
        })
    });
}

fn benchmark_consent_update(c: &mut Criterion) {
    let storage = Box::new(InMemoryStorage::new());
    let consent_service = ConsentService::new(storage);
    let consent_service = Arc::new(consent_service);
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("consent_update", |b| {
        b.iter_batched(|| {
            let user_id = format!("user_{}", rand::random::<u32>());
            let domain = Domain::FinancialData;
            let actor = Actor::System;
            (user_id, domain, actor, consent_service.clone())
        }, |(user_id, domain, actor, consent_service)| {
            async move {
                consent_service.update_consent_level(&user_id, domain, DataSharingLevel::Standard, actor).await.unwrap();
            }
        }, BatchSize::SmallInput)
    });
}

fn benchmark_migration_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("migration_throughput", |b| {
        b.iter_batched(|| {
            // Create test data
            let mut data = Vec::new();
            for i in 0..100 {
                data.push(consent_manager::migration::finance::FinanceConsentData {
                    user_id: format!("user_{}", i),
                    data_sharing_enabled: true,
                    anonymized_data: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                });
            }
            data
        }, |data| {
            async move {
                let storage = Box::new(InMemoryStorage::new());
                let consent_service = std::sync::Arc::new(ConsentService::new(storage));
                let actor = Actor::System;
                consent_manager::migration::finance::migrate_finance_consent(consent_service, data, actor).await.unwrap();
            }
        }, BatchSize::SmallInput)
    });
}

criterion_group!(benches, benchmark_consent_check, benchmark_consent_update, benchmark_migration_throughput);
criterion_main!(benches);