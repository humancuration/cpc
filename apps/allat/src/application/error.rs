use crate::infrastructure::repositories::community_repo::CommunityRepositoryError;
use crate::infrastructure::repositories::post_repo::PostRepositoryError;
use crate::infrastructure::repositories::comment_repo::CommentRepositoryError;
use crate::infrastructure::repositories::user_repository::UserRepositoryError;
use crate::infrastructure::repositories::vote_repo::VoteRepositoryError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Community repository error: {0}")]
    CommunityRepositoryError(#[from] CommunityRepositoryError),
    
    #[error("Post repository error: {0}")]
    PostRepositoryError(#[from] PostRepositoryError),
    
    #[error("Comment repository error: {0}")]
    CommentRepositoryError(#[from] CommentRepositoryError),
    
    #[error("User repository error: {0}")]
    UserRepositoryError(#[from] UserRepositoryError),
    
    #[error("Vote repository error: {0}")]
    VoteRepositoryError(#[from] VoteRepositoryError),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Resource not found")]
    NotFound,
    
    #[error("Karma limit exceeded")]
    KarmaLimitExceeded,
}