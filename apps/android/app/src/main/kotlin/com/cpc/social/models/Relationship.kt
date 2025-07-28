package com.cpc.social.models

import kotlinx.serialization.Serializable
import java.util.UUID
import java.time.Instant

@Serializable
data class Relationship(
    val id: String = UUID.randomUUID().toString(),
    val followerId: String,
    val followedId: String,
    val createdAt: String = Instant.now().toString()
) {
    companion object {
        fun fromJson(json: String): Relationship {
            return kotlinx.serialization.json.Json.decodeFromString(json)
        }
    }
}