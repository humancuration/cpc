package com.cpc.social.models

import kotlinx.serialization.Serializable
import java.util.UUID
import java.time.Instant

@Serializable
data class Post(
    val id: String = UUID.randomUUID().toString(),
    val authorId: String,
    val content: String,
    val visibility: Visibility,
    val cooperativeId: String? = null,
    val createdAt: String = Instant.now().toString(),
    val updatedAt: String = Instant.now().toString(),
    val mediaItems: List<MediaItem> = emptyList()
) {
    companion object {
        fun fromJson(json: String): Post {
            return kotlinx.serialization.json.Json.decodeFromString(json)
        }
    }
}