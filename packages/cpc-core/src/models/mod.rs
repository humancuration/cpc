#[cfg(test)]
mod user_test;
#[cfg(test)]
mod product_test;

pub mod user;
pub mod token;
pub mod api;
pub mod product;
pub mod vision;
pub mod social;
pub mod governance;

pub use user::{User, UserProfile, CooperativeScore, ContributionFactor, UserRelationship, UserRelationshipType};
pub use token::AuthToken;
pub use api::APIResponse;
pub use product::{Product, ProductOrigin, SupplyChain, SupplyChainStage, VerificationStatus};
pub use vision::{
    RecognitionResult, RecognitionItem, BoundingBox, VisionModelConfig, ModelType,
    VisionOptions, VisionMetrics, VisionCapabilities,
};
pub use social::{
    Relationship, Post, MediaItem, Comment, Reply, Like, Share, Repost, PostEdit, CommentEdit,
    Visibility, MediaType, ProcessingStatus, LikeTargetType, ShareType,
    PostType, Community, CommunityRule, Forum, ForumCategory, ModerationSettings,
    Thread, ThreadReply, ThreadReplyEdit, CommunityMembership, CommunityRole,
    Follow, Block, Mute, MuteType, Notification, NotificationType, NotificationPriority,
    Feed, FeedType, FeedAlgorithm, FeedItem, FeedContentType, FeedSettings,
    Vote, VoteTargetType, VoteType, ModerationAction, ModerationTargetType, ModerationActionType,
    UserActivity, ActivityType
};
pub use social::{
    Relationship, Post, MediaItem, Comment, Reply, Like, Share, Repost, PostEdit, CommentEdit,
    Visibility, MediaType, ProcessingStatus, LikeTargetType, ShareType,
    PostType, Community, CommunityRule, Forum, ForumCategory, ModerationSettings,
    Thread, ThreadReply, ThreadReplyEdit, CommunityMembership, CommunityRole,
    Follow, Block, Mute, MuteType, Notification, NotificationType, NotificationPriority,
    Feed, FeedType, FeedAlgorithm, FeedItem, FeedContentType, FeedSettings,
    Vote, VoteTargetType, VoteType, ModerationAction, ModerationTargetType, ModerationActionType,
    UserActivity, ActivityType
};

pub use governance::{
    Proposal, ProposalStatus, ProposalType, ProposedChange,
    Vote as GovernanceVote, VoteTally, VoteCount, VotingResult, GovernanceParticipation
};