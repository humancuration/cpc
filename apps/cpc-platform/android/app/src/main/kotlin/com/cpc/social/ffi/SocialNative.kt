package com.cpc.social.ffi

import com.cpc.social.models.Post
import com.cpc.social.models.MediaItem
import com.cpc.social.models.Relationship
import com.cpc.social.models.Visibility
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

object SocialNative {
    init {
        RustBridge
    }

    // Native methods to call Rust code
    external fun createPostNative(postJson: String): String
    external fun getTimelineNative(userId: String, limit: Int, offset: Int): String
    external fun getPostNative(postId: String): String
    external fun createRelationshipNative(followerId: String, followedId: String): String
    external fun getFollowersNative(userId: String): String
    external fun getFollowingNative(userId: String): String

    // Helper methods with Kotlin models
    fun createPost(post: Post): Post {
        val postJson = Json.encodeToString(post)
        val resultJson = createPostNative(postJson)
        return Json.decodeFromString(resultJson)
    }

    fun getTimeline(userId: String, limit: Int = 20, offset: Int = 0): List<Post> {
        val resultJson = getTimelineNative(userId, limit, offset)
        return Json.decodeFromString(resultJson)
    }

    fun getPost(postId: String): Post {
        val resultJson = getPostNative(postId)
        return Json.decodeFromString(resultJson)
    }

    fun createRelationship(followerId: String, followedId: String): Relationship {
        val resultJson = createRelationshipNative(followerId, followedId)
        return Json.decodeFromString(resultJson)
    }

    fun getFollowers(userId: String): List<Relationship> {
        val resultJson = getFollowersNative(userId)
        return Json.decodeFromString(resultJson)
    }

    fun getFollowing(userId: String): List<Relationship> {
        val resultJson = getFollowingNative(userId)
        return Json.decodeFromString(resultJson)
    }
}