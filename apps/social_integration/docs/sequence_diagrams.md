# Social Apps Sequence Diagrams

## Overview
This document contains sequence diagrams illustrating key workflows in the Allat and Yapper social applications, as well as their integration.

## User Registration Flow

### Allat Registration
```mermaid
sequenceDiagram
    participant U as User
    participant A as Allat API
    participant AS as AuthService
    participant UR as UserRepository
    participant EB as EventBus
    
    U->>A: POST /register {credentials}
    A->>AS: register(credentials)
    AS->>UR: create_user(user_data)
    UR-->>AS: User
    AS->>EB: publish(UserRegistered)
    AS-->>A: User
    A-->>U: 201 Created
```

### Yapper Registration
```mermaid
sequenceDiagram
    participant U as User
    participant Y as Yapper API
    participant AS as AuthService
    participant UR as UserRepository
    participant EB as EventBus
    
    U->>Y: POST /register {credentials}
    Y->>AS: register(credentials)
    AS->>UR: create_user(user_data)
    UR-->>AS: User
    AS->>EB: publish(UserRegistered)
    AS-->>Y: User
    Y-->>U: 201 Created
```

## Post Creation Flow

### Allat Post Creation
```mermaid
sequenceDiagram
    participant U as User
    participant A as Allat API
    participant PS as PostService
    participant MP as MediaProcessor
    participant PR as PostRepository
    participant FR as FeedRepository
    participant TM as TaskManager
    participant EB as EventBus
    
    U->>A: POST /communities/{id}/posts {content, media}
    A->>PS: create_post(community_id, content, media)
    PS->>MP: process_media(media_files)
    MP-->>PS: processed_media
    PS->>PR: store_post(post_data)
    PR-->>PS: Post
    PS->>FR: update_community_feed(post_id)
    PS->>TM: create_reward_task(user_id, "post_created")
    PS->>EB: publish(PostCreated)
    PS-->>A: Post
    A-->>U: 201 Created
```

### Yapper Post Creation
```mermaid
sequenceDiagram
    participant U as User
    participant Y as Yapper API
    participant PS as PostService
    participant MP as MediaProcessor
    participant PR as PostRepository
    participant FR as FeedRepository
    participant TM as TaskManager
    participant EB as EventBus
    
    U->>Y: POST /posts {content, media}
    Y->>PS: create_post(content, media)
    PS->>MP: process_media(media_files)
    MP-->>PS: processed_media
    PS->>PR: store_post(post_data)
    PR-->>PS: Post
    PS->>FR: update_user_feeds(post_id)
    PS->>TM: create_reward_task(user_id, "post_created")
    PS->>EB: publish(PostCreated)
    PS-->>Y: Post
    Y-->>U: 201 Created
```

## Cross-Posting Flow

### Yapper to Allat Cross-Post
```mermaid
sequenceDiagram
    participant U as User
    participant Y as Yapper API
    participant CPS as CrossPostingService
    participant A as Allat gRPC
    participant AP as Allat PostService
    
    U->>Y: POST /posts {content, cross_post: true}
    Y->>CPS: cross_post_to_allat(content, user_id)
    CPS->>A: CrossPostRequest
    A->>AP: create_post_from_crosspost(request)
    AP-->>A: Post
    A-->>CPS: CrossPostResponse
    CPS-->>Y: Success
    Y-->>U: 201 Created
```

### Allat to Yapper Cross-Post
```mermaid
sequenceDiagram
    participant U as User
    participant A as Allat API
    participant CPS as CrossPostingService
    participant Y as Yapper gRPC
    participant YP as Yapper PostService
    
    U->>A: POST /communities/{id}/posts {content, cross_post: true}
    A->>CPS: cross_post_to_yapper(content, user_id)
    CPS->>Y: CrossPostRequest
    Y->>YP: create_post_from_crosspost(request)
    YP-->>Y: Post
    Y-->>CPS: CrossPostResponse
    CPS-->>A: Success
    A-->>U: 201 Created
```

## Voting/Engagement Flow

### Allat Vote
```mermaid
sequenceDiagram
    participant U as User
    participant A as Allat API
    participant VS as VoteService
    participant VR as VoteRepository
    participant KUS as KarmaUpdateService
    participant EB as EventBus
    
    U->>A: POST /posts/{id}/vote {direction}
    A->>VS: record_vote(post_id, user_id, direction)
    VS->>VR: store_vote(vote_data)
    VR-->>VS: Vote
    VS->>KUS: update_user_karma(post_author_id, direction)
    VS->>EB: publish(VoteRecorded)
    VS-->>A: Vote
    A-->>U: 200 OK
```

### Yapper Like
```mermaid
sequenceDiagram
    participant U as User
    participant Y as Yapper API
    participant ES as EngagementService
    participant ER as EngagementRepository
    participant EB as EventBus
    
    U->>Y: POST /posts/{id}/like
    Y->>ES: record_like(post_id, user_id)
    ES->>ER: store_engagement(engagement_data)
    ER-->>ES: Engagement
    ES->>EB: publish(LikeRecorded)
    ES-->>Y: Engagement
    Y-->>U: 200 OK
```

## Feed Generation Flow

### Allat Community Feed
```mermaid
sequenceDiagram
    participant U as User
    participant A as Allat API
    participant FS as FeedService
    participant FR as FeedRepository
    participant S as Sled Cache
    
    U->>A: GET /communities/{id}/feed
    A->>FS: get_community_feed(community_id)
    FS->>S: get_cached_feed(community_id)
    S-->>FS: Feed (if cached)
    alt Cache Miss
        FS->>FR: generate_feed(community_id)
        FR-->>FS: Feed
        FS->>S: cache_feed(community_id, feed)
    end
    FS-->>A: Feed
    A-->>U: 200 OK + Feed
```

### Yapper Personal Feed
```mermaid
sequenceDiagram
    participant U as User
    participant Y as Yapper API
    participant FS as FeedService
    participant FR as FeedRepository
    participant S as Sled Cache
    
    U->>Y: GET /feed
    Y->>FS: get_personal_feed(user_id)
    FS->>S: get_cached_feed(user_id)
    S-->>FS: Feed (if cached)
    alt Cache Miss
        FS->>FR: generate_feed(user_id, following)
        FR-->>FS: Feed
        FS->>S: cache_feed(user_id, feed)
    end
    FS-->>Y: Feed
    Y-->>U: 200 OK + Feed