# Allat-Yapper Integration

## Overview
This document describes the integration between Allat (forum) and Yapper (microblog) to provide a unified social experience within the CPC ecosystem.

## Unified Feed
- Combines Allat posts and Yapper posts in a single timeline view
- Uses consent manager for privacy controls on cross-app visibility
- Allows users to filter content by source (Allat, Yapper, or both)
- Supports algorithmic and chronological sorting options

## Cross-Posting
- gRPC service between apps for seamless content sharing
- Privacy settings per cross-post (public, followers only, private)
- Automatic hashtag propagation from Yapper to Allat communities
- Media asset synchronization between platforms

### Cross-Posting Flow
```
1. User creates post in Yapper with "Cross-post to Allat" option
2. System validates user permissions and privacy settings
3. Post is sent to Allat via gRPC service
4. Allat processes and stores the post in appropriate community
5. Both platforms update user feeds
6. Engagement metrics are synchronized
```

## Shared Identity
- Unified karma/reputation system across both platforms
- Same authentication across apps using `cpc_oauth2`
- Consistent user profiles with single sign-on
- Cross-app notification system

## Data Model Integration

### Unified User Profile
```rust
struct UnifiedUserProfile {
    user_id: Uuid,
    username: String,
    display_name: String,
    bio: Option<String>,
    avatar_url: Option<String>,
    allat_karma: i32,
    yapper_followers: i32,
    communities_joined: Vec<Uuid>,
    yapper_following: Vec<Uuid>,
}
```

### Cross-Posted Content
```rust
struct CrossPost {
    id: Uuid,
    source_app: SocialApp,
    source_id: Uuid,
    target_app: SocialApp,
    target_id: Uuid,
    user_id: Uuid,
    privacy_level: PrivacyLevel,
    created_at: DateTime<Utc>,
}
```

## API Contracts

### gRPC Service
```protobuf
service CrossPostingService {
  rpc CrossPostToYapper(CrossPostRequest) returns (CrossPostResponse);
  rpc CrossPostToAllat(CrossPostRequest) returns (CrossPostResponse);
  rpc GetCrossPostStatus(CrossPostStatusRequest) returns (CrossPostStatusResponse);
}

message CrossPostRequest {
  string source_app = 1;
  string source_id = 2;
  string target_app = 3;
  string user_id = 4;
  PrivacyLevel privacy_level = 5;
}

message CrossPostResponse {
  bool success = 1;
  string target_id = 2;
  string error_message = 3;
}
```

## Integration Points

### Consent Management
- Uses `consent_manager` crate for cross-app data sharing
- Explicit user consent required for cross-posting
- Granular privacy controls per post and per app

### Task Manager Integration
- Cross-posting activities tracked as tasks in task manager
- Dabloons rewards for cross-platform engagement
- Achievement system for cross-app participation

### Real-time Updates
- WebSocket connections for live feed updates
- Synchronized notification system
- Real-time engagement metrics

## Performance Considerations
- Asynchronous cross-posting to prevent blocking UI
- Caching of cross-posted content to reduce database load
- Efficient synchronization of engagement metrics
- Rate limiting for cross-posting to prevent spam

## Security Considerations
- Validation of cross-post requests to prevent abuse
- Privacy level enforcement across platforms
- Audit logging for all cross-posting activities
- Secure communication between services via gRPC

## TODO
- [ ] Implement real-time synchronization of engagement metrics
- [ ] Add cross-app search functionality
- [ ] Create unified notification system
- [ ] Implement advanced privacy controls for cross-posting