package com.cpc.social.models

import kotlinx.serialization.Serializable
import java.util.UUID
import java.time.Instant

@Serializable
data class MediaItem(
    val id: String = UUID.randomUUID().toString(),
    val postId: String,
    val url: String,
    val mediaType: MediaType,
    val createdAt: String = Instant.now().toString()
) {
    companion object {
        fun fromJson(json: String): MediaItem {
            return kotlinx.serialization.json.Json.decodeFromString(json)
        }
    }
}

@Serializable
enum class MediaType {
    IMAGE,
    VIDEO,
    AUDIO,
    UNKNOWN
}