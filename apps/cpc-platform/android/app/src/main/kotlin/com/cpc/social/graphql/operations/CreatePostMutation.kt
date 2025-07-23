package com.cpc.social.graphql.operations

import kotlinx.serialization.Serializable

@Serializable
data class CreatePostMutation(
    val content: String,
    val visibility: String,
    val cooperativeId: String? = null
)

@Serializable
data class CreatePostResponse(
    val data: CreatePostData
)

@Serializable
data class CreatePostData(
    val createPost: PostData
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
    val mediaItems: List<MediaItemData>
)

@Serializable
data class MediaItemData(
    val id: String,
    val postId: String,
    val url: String,
    val mediaType: String,
    val createdAt: String
)