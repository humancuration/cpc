use criterion::{criterion_group, criterion_main, Criterion};
use sqlx::PgPool;
use uuid::Uuid;
use learning_platform_server::{
    database::{models::*, repository::DatabaseRepository},
    grpc::course_service::CourseService,
};

async fn create_test_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/learning_platform".to_string());
    PgPool::connect(&db_url).await.unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create_course", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| async {
            let pool = create_test_pool().await;
            let repository = DatabaseRepository::new(pool);
            let course_service = CourseService::new(repository);
            
            // Create a course
            let course = learning_platform_server::Course {
                id: String::new(),
                title: "Benchmark Course".to_string(),
                description: "Benchmark Description".to_string(),
                creator_id: Uuid::new_v4().to_string(),
                modules: vec![],
            };
            
            let request = tonic::Request::new(learning_platform_server::CreateCourseRequest {
                course: Some(course),
            });
            
            // Call the create_course method
            let _response = course_service.create_course(request).await.unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);