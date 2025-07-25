package com.cpc.serialization

import com.cpc.android.JniBridge
import com.cpc.social.models.Post
import com.cpc.social.models.Comment
import org.junit.Test
import org.junit.Assert.*
import java.util.UUID

class PostSerializationTest {

    @Test
    fun `basic post serialization round-trip`() {
        val post = Post(
            id = UUID.randomUUID().toString(),
            content = "Test post content",
            authorId = "user123",
            likes = 10,
            comments = listOf(
                Comment(
                    id = UUID.randomUUID().toString(),
                    postId = "post123",
                    authorId = "user456",
                    content = "Test comment"
                )
            )
        )

        // Serialize to protobuf bytes
        val bytes = JniBridge.serializePost(post)

        // Deserialize through JNI
        val deserializedPost = JniBridge.deserializePost(bytes)
        
        // Verify
        assertEquals(originalPost.id, deserializedPost.id)
        assertEquals(originalPost.content, deserializedPost.content)
        assertEquals(originalPost.authorId, deserializedPost.authorId)
        assertEquals(originalPost.likes, deserializedPost.likes)
        assertEquals(originalPost.comments.size, deserializedPost.comments.size)
        originalPost.comments.forEachIndexed { i, comment ->
            assertEquals(comment.id, deserializedPost.comments[i].id)
            assertEquals(comment.postId, deserializedPost.comments[i].postId)
            assertEquals(comment.authorId, deserializedPost.comments[i].authorId)
            assertEquals(comment.content, deserializedPost.comments[i].content)
        }
    }

    @Test
    fun `post with empty content`() {
        val post = Post(
            id = UUID.randomUUID().toString(),
            content = "",
            authorId = "user123",
            likes = 0,
            comments = emptyList()
        )

        val bytes = JniBridge.serializePost(post)
        assertNotNull(bytes)
        assertTrue(bytes.isNotEmpty())
    }

    @Test
    fun `post with maximum content length`() {
        val longContent = "a".repeat(10000)
        val post = Post(
            id = UUID.randomUUID().toString(),
            content = longContent,
            authorId = "user123",
            likes = 100,
            comments = emptyList()
        )

        val bytes = JniBridge.serializePost(post)
        assertNotNull(bytes)
        assertTrue(bytes.isNotEmpty())
    }

    @Test
    fun `post deserialization with multiple comments`() {
        val comments = listOf(
            Comment(
                id = UUID.randomUUID().toString(),
                postId = "post123",
                authorId = "user456",
                content = "First comment"
            ),
            Comment(
                id = UUID.randomUUID().toString(),
                postId = "post123",
                authorId = "user789",
                content = "Second comment"
            )
        )
        
        val post = Post(
            id = UUID.randomUUID().toString(),
            content = "Post with multiple comments",
            authorId = "user123",
            likes = 5,
            comments = comments
        )

        val bytes = JniBridge.serializePost(post)
        val deserializedPost = JniBridge.deserializePost(bytes)
        
        assertEquals(post.id, deserializedPost.id)
        assertEquals(post.content, deserializedPost.content)
        assertEquals(post.authorId, deserializedPost.authorId)
        assertEquals(post.likes, deserializedPost.likes)
        assertEquals(post.comments.size, deserializedPost.comments.size)
        post.comments.forEachIndexed { i, comment ->
            assertEquals(comment.id, deserializedPost.comments[i].id)
            assertEquals(comment.postId, deserializedPost.comments[i].postId)
            assertEquals(comment.authorId, deserializedPost.comments[i].authorId)
            assertEquals(comment.content, deserializedPost.comments[i].content)
        }
    }
}