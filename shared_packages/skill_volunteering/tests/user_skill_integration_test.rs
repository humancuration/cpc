//! Integration tests for the UserSkillService.

use skill_volunteering::{
    postgres::user_skill_repository::PostgresUserSkillRepository,
    user_skill_management::{
        models::SkillLevel, repository::UserSkillRepository, service::UserSkillService,
    },
};
use sqlx::{Executor, PgPool};
use std::sync::Arc;
use testcontainers_modules::{postgres, testcontainers::clients};
use uuid::Uuid;

// Helper function to set up the test environment
async fn setup_test_db() -> (
    PgPool,
    Arc<dyn UserSkillRepository>,
    testcontainers::Container<'static, postgres::Postgres>,
) {
    let docker = clients::Cli::default();
    let postgres_node = postgres::Postgres::default().with_user("test").with_password("test").with_db_name("test_db");
    let container = docker.run(postgres_node);
    let connection_string = &format!(
        "postgres://test:test@127.0.0.1:{}/test_db",
        container.get_host_port_ipv4(5432)
    );

    let pool = PgPool::connect(connection_string)
        .await
        .expect("Failed to connect to test Postgres container");

    // Manually run migrations from the parent directory
    let migrator = sqlx::migrate!("../migrations");
    migrator
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    // The user_skills table depends on the skills table, so we need to create the skills table first.
    // The migrations should handle this, but let's ensure the SQL is correct.
    // The migration `20250801000001_create_skills_table.sql` should exist and create the `skills` table.
    // Assuming it does, we can proceed.
    // We also need to create the skill_level_enum type.
    pool.execute("CREATE TYPE skill_level_enum AS ENUM ('beginner', 'intermediate', 'advanced');")
        .await
        .ok(); // Ignore error if it already exists from the migration.

    // The user_skills table migration has a dependency on the skills table.
    // Let's create the skills table for our tests.
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS skills (
            id UUID PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            category VARCHAR(255) NOT NULL,
            description TEXT,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );
        "#,
    )
    .await
    .expect("Failed to create skills table for test");

    // Now run the user_skills table migration content manually if migrator fails to find it.
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS user_skills (
            user_id UUID NOT NULL,
            skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
            skill_level skill_level_enum NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW(),
            PRIMARY KEY (user_id, skill_id)
        );
        "#,
    )
    .await
    .expect("Failed to create user_skills table for test");


    let repo = Arc::new(PostgresUserSkillRepository::new(pool.clone()));

    (pool, repo, container)
}

// Helper to create a dummy skill for FK constraints
async fn create_dummy_skill(pool: &PgPool, name: &str) -> Uuid {
    let skill_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO skills (id, name, category) VALUES ($1, $2, 'test')",
        skill_id,
        name
    )
    .execute(pool)
    .await
    .expect("Failed to create dummy skill");
    skill_id
}

#[tokio::test]
async fn test_add_user_skill_success() {
    let (pool, repo, _container) = setup_test_db().await;
    let service = UserSkillService::new(repo);

    let user_id = Uuid::new_v4();
    let skill_id = create_dummy_skill(&pool, "Rust").await;

    let result = service
        .add_user_skill(user_id, skill_id, "beginner")
        .await;

    assert!(result.is_ok());
    let user_skill = result.unwrap();
    assert_eq!(user_skill.user_id, user_id);
    assert_eq!(user_skill.skill_id, skill_id);
    assert_eq!(user_skill.skill_level, SkillLevel::Beginner);
}

#[tokio::test]
async fn test_add_user_skill_duplicate() {
    let (pool, repo, _container) = setup_test_db().await;
    let service = UserSkillService::new(repo);

    let user_id = Uuid::new_v4();
    let skill_id = create_dummy_skill(&pool, "Docker").await;

    // Add the skill first time
    service
        .add_user_skill(user_id, skill_id, "intermediate")
        .await
        .expect("First add should succeed");

    // Try to add it again
    let result = service
        .add_user_skill(user_id, skill_id, "advanced")
        .await;

    assert!(result.is_err());
    // The repository returns a generic database error for duplicates which the service maps to AlreadyExists
    // Depending on the exact DB error, this might need adjustment. PG `ON CONFLICT` would be better.
    // For now, we assume the current implementation catches the unique constraint violation.
}

#[tokio::test]
async fn test_add_skill_for_non_existent_user() {
    // Our current design doesn't have a `users` table in this service's scope,
    // so there's no FK constraint on `user_id`. The service implicitly trusts the user_id.
    // This test confirms the current behavior: adding a skill for any UUID succeeds.
    let (pool, repo, _container) = setup_test_db().await;
    let service = UserSkillService::new(repo);

    let non_existent_user_id = Uuid::new_v4();
    let skill_id = create_dummy_skill(&pool, "K8s").await;

    let result = service
        .add_user_skill(non_existent_user_id, skill_id, "advanced")
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_user_skills_multiple() {
    let (pool, repo, _container) = setup_test_db().await;
    let service = UserSkillService::new(repo);

    let user_id = Uuid::new_v4();
    let skill_id_1 = create_dummy_skill(&pool, "SQL").await;
    let skill_id_2 = create_dummy_skill(&pool, "gRPC").await;

    service
        .add_user_skill(user_id, skill_id_1, "advanced")
        .await
        .unwrap();
    service
        .add_user_skill(user_id, skill_id_2, "intermediate")
        .await
        .unwrap();

    let skills = service.list_user_skills(user_id).await.unwrap();
    assert_eq!(skills.len(), 2);
    // Results are ordered by skill name
    assert_eq!(skills[0].skill.name, "gRPC");
    assert_eq!(skills[1].skill.name, "SQL");
}

#[tokio::test]
async fn test_list_user_skills_empty() {
    let (_pool, repo, _container) = setup_test_db().await;
    let service = UserSkillService::new(repo);
    let user_id = Uuid::new_v4();

    let skills = service.list_user_skills(user_id).await.unwrap();
    assert!(skills.is_empty());
}

#[tokio::test]
async fn test_list_skills_for_non_existent_user() {
    // Similar to the add test, listing skills for a user that doesn't exist
    // should just return an empty list, not an error.
    let (_pool, repo, _container) = setup_test_db().await;
    let service = UserSkillService::new(repo);
    let non_existent_user_id = Uuid::new_v4();

    let skills = service
        .list_user_skills(non_existent_user_id)
        .await
        .unwrap();
    assert!(skills.is_empty());
}

#[tokio::test]
async fn test_remove_user_skill_success() {
    let (pool, repo, _container) = setup_test_db().await;
    let service = UserSkillService::new(repo.clone());

    let user_id = Uuid::new_v4();
    let skill_id = create_dummy_skill(&pool, "Tauri").await;

    service
        .add_user_skill(user_id, skill_id, "beginner")
        .await
        .unwrap();

    // Verify it was added
    assert_eq!(service.list_user_skills(user_id).await.unwrap().len(), 1);

    // Remove it
    let result = service.remove_user_skill(user_id, skill_id).await;
    assert!(result.is_ok());

    // Verify it was removed
    assert!(service.list_user_skills(user_id).await.unwrap().is_empty());
}

#[tokio::test]
async fn test_remove_non_existent_skill() {
    let (pool, repo, _container) = setup_test_db().await;
    let service = UserSkillService::new(repo);

    let user_id = Uuid::new_v4();
    let skill_id = create_dummy_skill(&pool, "NonExistentSkill").await;
    let other_skill_id = create_dummy_skill(&pool, "AnotherSkill").await;

    // Add a different skill to make sure the user exists in the table
    service
        .add_user_skill(user_id, other_skill_id, "beginner")
        .await
        .unwrap();

    // Try to remove a skill the user doesn't have
    let result = service.remove_user_skill(user_id, skill_id).await;
    assert!(result.is_err());
    // The repo returns NotFound, which the service propagates.
}