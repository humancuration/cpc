//! Repositories for social integration

pub mod in_memory_unified_post_repository;
pub mod postgres_unified_post_repository;
pub mod postgres_user_following_repository;

#[cfg(test)]
pub mod postgres_unified_post_repository_test;
#[cfg(test)]
pub mod postgres_user_following_repository_test;

pub use in_memory_unified_post_repository::InMemoryUnifiedPostRepository;
pub use postgres_unified_post_repository::PostgresUnifiedPostRepository;
pub use postgres_user_following_repository::{PostgresUserFollowingRepository, UserFollowingRepository};