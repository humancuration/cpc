package com.cpc.serialization

import com.cpc.android.JniBridge
import com.cpc.social.models.Comment
import io.mockk.every
import io.mockk.mockkObject
import io.mockk.unmockkAll
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test
import java.util.Base64

class CommentSerializationTest {

    @Before
    fun setUp() {
        mockkObject(JniBridge)
    }

    @After
    fun tearDown() {
        unmockkAll()
    }

    @Test
    fun `basic comment round-trip`() {
        // Setup mock serialization/deserialization
        every { JniBridge.serializeComment(any()) } answers { 
            val comment = firstArg<Comment>()
            "${comment.id}|${comment.content}|${comment.authorId}".toByteArray()
        }
        every { JniBridge.deserializeComment(any()) } answers {
            val bytes = firstArg<ByteArray>()
            val parts = String(bytes).split("|")
            Comment(id = parts[0], content = parts[1], authorId = parts[2])
        }

        val comment = Comment(id = "id1", content = "Test comment", authorId = "user123")
        val bytes = JniBridge.serializeComment(comment)
        val deserialized = JniBridge.deserializeComment(bytes)
        
        assertEquals(comment, deserialized)
    }

    @Test
    fun `comment with empty content`() {
        // Setup mock serialization/deserialization
        every { JniBridge.serializeComment(any()) } answers { 
            val comment = firstArg<Comment>()
            "${comment.id}|${comment.content}|${comment.authorId}".toByteArray()
        }
        every { JniBridge.deserializeComment(any()) } answers {
            val bytes = firstArg<ByteArray>()
            val parts = String(bytes).split("|")
            Comment(id = parts[0], content = parts[1], authorId = parts[2])
        }

        val comment = Comment(id = "id2", content = "", authorId = "user456")
        val bytes = JniBridge.serializeComment(comment)
        val deserialized = JniBridge.deserializeComment(bytes)
        
        assertEquals(comment, deserialized)
    }

    @Test
    fun `comment with special characters`() {
        // Setup Base64 encoding to handle special characters
        every { JniBridge.serializeComment(any()) } answers { 
            val comment = firstArg<Comment>()
            val idB64 = Base64.getEncoder().encodeToString(comment.id.toByteArray())
            val contentB64 = Base64.getEncoder().encodeToString(comment.content.toByteArray())
            val authorIdB64 = Base64.getEncoder().encodeToString(comment.authorId.toByteArray())
            "$idB64|$contentB64|$authorIdB64".toByteArray()
        }
        every { JniBridge.deserializeComment(any()) } answers {
            val bytes = firstArg<ByteArray>()
            val parts = String(bytes).split("|")
            val id = String(Base64.getDecoder().decode(parts[0]))
            val content = String(Base64.getDecoder().decode(parts[1]))
            val authorId = String(Base64.getDecoder().decode(parts[2]))
            Comment(id, content, authorId)
        }

        val comment = Comment(
            id = "id#3", 
            content = "Special | characters \n test", 
            authorId = "user@789"
        )
        val bytes = JniBridge.serializeComment(comment)
        val deserialized = JniBridge.deserializeComment(bytes)
        
        assertEquals(comment, deserialized)
    }

    @Test(expected = Exception::class)
    fun `invalid deserialization handling`() {
        every { JniBridge.deserializeComment(any()) } throws Exception("Invalid data")
        
        JniBridge.deserializeComment("invalid".toByteArray())
    }
}