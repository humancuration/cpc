package com.cpc.social.graphql.operations

import kotlinx.serialization.Serializable

@Serializable
data class TimelineQuery(
    val userId: String,
    val limit: Int = 20,
    val offset: Int = 0
)

@Serializable
data class TimelineResponse(
    val data: TimelineData
)

@Serializable
data class TimelineData(
    val timeline: List<PostData>
)

@Serializable
data class PostData(
    val id: String,
    val authorId: String,
    val content: String,
    val visibility: String,
    val cooperativeId: String?,
    val createdAt: String,
    val updatedAt: String,
    val mediaItems: List<MediaItemData>,
    val author: AuthorData
)

@Serializable
data class AuthorData(
    val id: String,
    val username: String,
    val displayName: String,
    val avatarUrl: String?
)

@Serializable
data class MediaItemData(
    val id: String,
    val postId: String,
    val url: String,
    val mediaType: String,
    val createdAt: String
)