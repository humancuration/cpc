package com.cpc.musicplayer

import java.util.UUID

/**
 * Rust<->Kotlin interface for the complete Music Player functionality
 */
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

// Data classes for bridge communication
data class PlaySession(
    val sessionId: String,
    val trackId: String,
    val positionMs: Int
)

data class Track(
    val id: String,
    val title: String,
    val artistId: String,
    val durationMs: Int,
    val albumId: String? = null,
    val coverArtUrl: String? = null
)

data class DownloadStatus(
    val trackId: String,
    val status: String, // "pending", "downloading", "completed", "failed"
    val progress: Float = 0.0f,
    val offlineUrl: String? = null
)

data class ConsentStatus(
    val consentType: String,
    val granted: Boolean,
    val expiresAt: Long? // Unix timestamp in milliseconds
)

data class ConsentRequestResult(
    val consentType: String,
    val granted: Boolean,
    val newToken: String? = null
)