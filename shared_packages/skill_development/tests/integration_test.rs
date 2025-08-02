use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use skill_development::*;

#[tokio::test]
async fn test_database_setup_and_basic_operations() -> Result<(), Box<dyn std::error::Error>> {
    // This test assumes you have a test database set up
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://skill_dev_user:secure_password@localhost/skill_dev_test_db".to_string());
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Run migrations
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await?;
    
    let repo = infrastructure::repositories::postgres_repo::PostgresRepository::new(pool.clone());
    let service = application::SkillDevelopmentService::new(repo);
    
    // Test adding a skill
    let skill = service.add_skill(
        "Rust Programming".to_string(),
        Some("Master Rust programming language".to_string())
    ).await?;
    
    assert_eq!(skill.name, "Rust Programming");
    
    // Test adding skill progress
    let user_id = Uuid::new_v4();
    let progress = service.update_skill_progress(skill.id, user_id, 75).await?;
    
    assert_eq!(progress.skill_id, skill.id);
    assert_eq!(progress.user_id, user_id);
    assert_eq!(progress.progress, 75);
    
    // Test adding certification
    let certification = service.add_certification(
        "Rust Certified Developer".to_string(),
        "Rust Foundation".to_string(),
        chrono::Utc::today().naive_utc(),
        user_id
    ).await?;
    
    assert_eq!(certification.name, "Rust Certified Developer");
    
    // Test getting user profile
    let profile = service.get_user_skill_profile(user_id).await?;
    
    assert_eq!(profile.user_id, user_id);
    assert_eq!(profile.skills.len(), 1);
    assert_eq!(profile.certifications.len(), 1);
    
    Ok(())
}