package com.cpc.social.repository

import com.cpc.social.ffi.SocialNative
import com.cpc.social.graphql.SocialGraphQLClient
import com.cpc.social.models.Post
import com.cpc.social.models.Relationship
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

class SocialRepository(
    private val graphQLClient: SocialGraphQLClient,
    private val socialNative: SocialNative
) {

    suspend fun createPost(content: String, visibility: String = "PUBLIC"): Post {
        return withContext(Dispatchers.IO) {
            // Use Rust networking layer for offline-first approach
            val postId = socialNative.createPost(content, visibility)
            Post(
                id = postId,
                content = content,
                authorId = "current_user", // Get from auth
                visibility = visibility,
                createdAt = System.currentTimeMillis(),
                updatedAt = System.currentTimeMillis()
            )
        }
    }

    suspend fun getPostsByUser(userId: String): List<Post> {
        return withContext(Dispatchers.IO) {
            // Use GraphQL client to fetch posts
            graphQLClient.getPostsByUser(userId)
        }
    }

    suspend fun followUser(userId: String): Relationship {
        return withContext(Dispatchers.IO) {
            val relationshipId = socialNative.followUser(userId)
            Relationship(
                id = relationshipId,
                followerId = "current_user", // Get from auth
                followingId = userId,
                createdAt = System.currentTimeMillis()
            )
        }
    }

    suspend fun unfollowUser(userId: String): Boolean {
        return withContext(Dispatchers.IO) {
            socialNative.unfollowUser(userId)
        }
    }

    suspend fun getFollowers(userId: String): List<Relationship> {
        return withContext(Dispatchers.IO) {
            graphQLClient.getFollowers(userId)
        }
    }

    suspend fun getFollowing(userId: String): List<Relationship> {
        return withContext(Dispatchers.IO) {
            graphQLClient.getFollowing(userId)
        }
    }

    suspend fun getTimeline(): List<Post> {
        return withContext(Dispatchers.IO) {
            graphQLClient.getTimeline()
        }
    }
}