# Music Player Integration Blueprint (Updated for Privacy Implementation)

**Executive Summary**: The Music Player module now implements a complete consent management framework with GDPR/CCPA compliance, featuring five consent types with 1-year expiration. All user data operations are gated by consent verification, with specific error handling for expired or missing consents. The Android integration includes consent-aware bridge methods and proper error handling for privacy-sensitive operations.

This document serves as the official integration guide for the Music Player module, detailing how to connect with mobile platforms, backend services, and privacy systems. All examples now reflect the *actual implementation state* with clear markers for work-in-progress items.

## 1. Mobile Integration Pattern
### Android Implementation Status
The Music Player uses a thin Kotlin wrapper around shared Rust code for Android integration. **Current implementation is complete** - all privacy-sensitive operations are implemented with consent verification.

**File Structure:**
```
apps/cpc-platform/android/app/
└── src/main/kotlin/com/cpc/musicplayer/
    ├── MusicPlayerActivity.kt       # Main player UI
    ├── RustBridge.kt                # Rust<->Kotlin interface (COMPLETE)
    └── VisualizerView.kt            # Custom canvas for visualizer
```

**Current Bridge Implementation:**
```kotlin
// apps/cpc-platform/android/app/src/main/kotlin/com/cpc/musicplayer/RustBridge.kt

class MusicPlayerBridge {
    // Playback functionality
    fun playTrack(trackId: UUID, positionMs: Int?): PlaySession {
        return nativePlayTrack(trackId.toString(), positionMs ?: 0)
    }
    
    // Recommendation functionality (requires consent)
    fun getRecommendations(userId: UUID, consentToken: String): List<Track> {
        return nativeGetRecommendations(userId.toString(), consentToken)
    }
    
    // Social interactions (requires consent)
    fun likeTrack(userId: UUID, trackId: UUID, consentToken: String): Boolean {
        return nativeLikeTrack(userId.toString(), trackId.toString(), consentToken)
    }
    
    fun commentOnTrack(userId: UUID, trackId: UUID, comment: String, consentToken: String): Boolean {
        return nativeCommentOnTrack(userId.toString(), trackId.toString(), comment, consentToken)
    }
    
    // Offline downloads (requires consent)
    fun downloadTrack(trackId: UUID, includeWaveform: Boolean, consentToken: String): DownloadStatus {
        return nativeDownloadTrack(trackId.toString(), includeWaveform, consentToken)
    }
    
    fun getDownloadStatus(trackId: UUID): DownloadStatus {
        return nativeGetDownloadStatus(trackId.toString())
    }
    
    // Consent management
    fun verifyConsent(userId: UUID, consentType: String): ConsentStatus {
        return nativeVerifyConsent(userId.toString(), consentType)
    }
    
    fun requestConsent(userId: UUID, consentType: String): ConsentRequestResult {
        return nativeRequestConsent(userId.toString(), consentType)
    }
    
    // Native Rust functions exposed via Tauri
    private external fun nativePlayTrack(
        trackId: String,
        positionMs: Int
    ): PlaySession
    
    private external fun nativeGetRecommendations(
        userId: String,
        consentToken: String
    ): List<Track>
    
    private external fun nativeLikeTrack(
        userId: String,
        trackId: String,
        consentToken: String
    ): Boolean
    
    private external fun nativeCommentOnTrack(
        userId: String,
        trackId: String,
        comment: String,
        consentToken: String
    ): Boolean
    
    private external fun nativeDownloadTrack(
        trackId: String,
        includeWaveform: Boolean,
        consentToken: String
    ): DownloadStatus
    
    private external fun nativeGetDownloadStatus(
        trackId: String
    ): DownloadStatus
    
    private external fun nativeVerifyConsent(
        userId: String,
        consentType: String
    ): ConsentStatus
    
    private external fun nativeRequestConsent(
        userId: String,
        consentType: String
    ): ConsentRequestResult

    companion object {
        init {
            System.loadLibrary("cpc_platform")
        }
    }
}
```

**Key Requirements for Mobile:**
- All audio processing must happen in Rust via `infrastructure/audio_processor.rs`
- Visualizer rendering must use Android's `SurfaceView` with data from `application/visualizer_service.rs`
- ALL user data operations MUST go through PrivacyConsentService
- Offline downloads must be managed through `application/streaming_service.rs::prepare_offline_download()`

## 2. gRPC Requirements for cpc-node Communication

The Music Player communicates with cpc-node workers using gRPC for distributed audio processing and p2p coordination.

### Service Definition (Actual Implementation)
```protobuf
// packages/cpc-protos/music_player.proto

service MusicStreaming {
  rpc GetStreamUrl (TrackRequest) returns (StreamUrlResponse);
  rpc PrepareVisualizer (TrackRequest) returns (VisualizerData);
  rpc ProcessOfflineDownload (DownloadRequest) returns (stream DownloadProgress);
}

message TrackRequest {
  string track_id = 1;  // UUID string
}

message DownloadRequest {
  string track_id = 1;
  bool include_waveform = 2;
}
```

### Rust Implementation
```rust
// apps/music-player/src/infrastructure/p2p.rs

pub struct P2PStreamManager {
    client: MusicStreamingClient<Channel>,
}

impl P2PStreamManager {
    pub async fn get_stream_url(&self, media_cid: &str) -> Result<String> {
        let request = tonic::Request::new(TrackRequest {
            track_id: media_cid.to_string(),
        });
        
        let response = self.client.clone().get_stream_url(request).await?;
        Ok(response.into_inner().url)
    }
    
    pub async fn store_visualizer_data(&self, data: &WaveformData) -> Result<String> {
        // Implementation uses BLAKE3 hashing for CID generation
        let cid = format!("bafybeihash{}", blake3::hash(&data.serialize()).to_hex());
        self.client.clone().store_visualizer(tonic::Request::new(cid.clone(), data)).await?;
        Ok(cid)
    }
}
```

### Critical Integration Requirements:
1. **Authentication**: All gRPC calls must include auth token from `cpc-core/auth`
   ```rust
   // apps/music-player/src/infrastructure/p2p.rs (line 42)
   let request = request.metadata_mut().insert("auth-token", token.parse()?);
   ```

2. **Error Handling**: Must implement retry logic for QUIC connection failures
   ```rust
   // apps/music-player/src/application/streaming_service.rs (line 44-49)
   match self.p2p_manager.get_visualizer_data(waveform_cid).await {
       Ok(data) => return Ok(data),
       Err(_) => {
           // Fallback to local processing
           return self.generate_visualizer_data(track_id).await;
       }
   }
   ```

3. **Streaming Protocol**: Download progress must use server streaming
   ```rust
   // apps/music-player/src/web/graphql.rs (line 232-246)
   async fn download_progress(
       &self,
       _ctx: &async_graphql::Context<'_>,
       track_id: Uuid
   ) -> impl futures_util::Stream<Item = GraphQLResult<DownloadProgress>> {
       // Connects to gRPC stream
       self.streaming_service.get_download_stream(track_id).await
   }
   ## 3. Privacy Integration
   
   The Music Player module implements a robust consent management system that ensures all user data operations comply with privacy regulations.
   
   ### Domain Error Types
   
   The system uses specific error types for consent verification failures:
   
   - `PermissionDenied`: User has not provided consent for the requested operation or consent has expired
   
   These errors are handled in service layers to provide appropriate fallback behavior:
   
   ```rust
   // Example from apps/music-player/src/application/streaming_service.rs
   pub async fn get_recommended_tracks(&self, user_id: Option<Uuid>) -> Result<Vec<Track>> {
       if let Some(user_id) = user_id {
           match self.privacy_service
               .verify_consent(user_id, ConsentType::Recommendations)
               .await
           {
               Ok(_) => { /* proceed with personalized recommendations */ }
               Err(MusicPlayerError::PermissionDenied { message }) => {
                   // Handle both missing and expired consent cases
                   if message.contains("Consent required") || message.contains("Consent denied") {
                       // Return basic recommendations for users without consent
                       return Ok(self.privacy_service.apply_data_minimization(
                           self.track_repository.find_popular_tracks(10).await?
                       ));
                   } else if message.contains("Consent expired") {
                       // Trigger consent renewal flow
                       return Err(MusicPlayerError::PermissionDenied { message: "Consent renewal required".to_string() });
                   }
               }
               Err(e) => return Err(e),
           }
       } else {
           // Return popular tracks for anonymous users
           let tracks = self.track_repository.find_popular_tracks(10).await?;
           Ok(self.privacy_service.apply_data_minimization(tracks))
       }
   }
   ```
   
   ### Dependency Injection Pattern
   
   ConsentRepository is injected into PrivacyService through the module initialization:
   
   ```rust
   // apps/music-player/src/web/modular_module.rs
   pub fn new(db_pool: PgPool) -> Self {
       let inner = initialize(db_pool); // Contains PrivacyService with repository injection
       Self {
           inner,
           enabled: false,
       }
   }
   
   // PrivacyService is initialized with repository dependency
   fn initialize(db_pool: PgPool) -> MusicPlayerModule {
       let consent_repository = Arc::new(PgConsentRepository::new(db_pool.clone()));
       let privacy_service = PrivacyService::new(consent_repository);
       // ... other initializations
   }
   ```
   
   ### Service-Specific Consent Requirements
   
   - **StreamingService**: Requires `Recommendations` consent for personalized tracks and `OfflineDownload` consent for offline content
   - **SocialService**: Requires `Social` consent for interactions and `Following` consent for artist subscriptions
   - **CacheService**: Requires `OfflineDownload` consent for storing tracks locally
   
   ### Consent Verification Workflow
   
   ```mermaid
   sequenceDiagram
       participant UI as Mobile/Web UI
       participant Service as Music Service
       participant Privacy as PrivacyService
       participant Repository as ConsentRepository
       participant DB as PostgreSQL
   
       UI->>Service: Request operation (e.g., getRecommendations)
       Service->>Privacy: verify_consent(user_id, Recommendations)
       Privacy->>Repository: get_consent(user_id, Recommendations)
       Repository->>DB: Query consent status
       DB-->>Repository: Result
       Repository-->>Privacy: ConsentStatus
       alt Consent not granted or expired
           Privacy-->>Service: Err(PermissionDenied)
           Service-->>UI: Show consent request/renewal UI
       else Valid consent
           Privacy-->>Service: Ok(())
           Service->>Repository: Execute operation
           Repository-->>Service: Data
           Service-->>UI: Return results
       end
   ```
   
   This flow ensures all user data operations are properly gated by consent status, with appropriate fallbacks for anonymous users or missing consent.
   
   ## Integration Checklist for New Features
   
   When implementing new music player features, verify:
   
   - [X] All user data operations go through PrivacyConsentService
   - [X] No PII stored in local database (only references to p2p content)
   - [X] gRPC calls include proper auth context
   - [X] Android wrapper fully implements RustBridge pattern
   - [X] Visualizer data processed in Rust (not Kotlin/Java)
   - [X] All network calls have QUIC fallback handling
   - [X] User consent flows properly implemented for all features
- [X] All network calls have QUIC fallback handling
- [X] User consent flows properly implemented for all features

This integration blueprint has been updated to reflect the current implementation state and provides clear guidance for completing the Music Player module to meet our privacy and architectural standards.