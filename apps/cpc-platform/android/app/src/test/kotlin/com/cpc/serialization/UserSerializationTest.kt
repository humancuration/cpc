package com.cpc.serialization

import com.cpc.android.JniBridge
import com.cpc.social.models.User
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class UserSerializationTest {

    @Test
    fun `user round-trip serialization`() {
        val originalUser = User(
            id = "user123",
            name = "Test User",
            email = "test@example.com"
        )

        // Serialize to protobuf bytes
        val bytes = JniBridge.serializeUser(originalUser)

        // Deserialize through JNI
        val deserializedUser = JniBridge.deserializeUser(bytes)

        // Verify
        assertEquals(originalUser.id, deserializedUser.id)
        assertEquals(originalUser.name, deserializedUser.name)
        assertEquals(originalUser.email, deserializedUser.email)
    }

    @Test
    fun `serialize user with empty fields`() {
        val user = User(
            id = "",
            name = "",
            email = ""
        )

        val bytes = JniBridge.serializeUser(user)
        val deserialized = JniBridge.deserializeUser(bytes)

        assertEquals("", deserialized.id)
        assertEquals("", deserialized.name)
        assertEquals("", deserialized.email)
    }

    @Test
    fun `serialize user with max length fields`() {
        val longString = "a".repeat(10000)
        val user = User(
            id = longString,
            name = longString,
            email = longString
        )

        val bytes = JniBridge.serializeUser(user)
        val deserialized = JniBridge.deserializeUser(bytes)

        assertEquals(longString, deserialized.id)
        assertEquals(longString, deserialized.name)
        assertEquals(longString, deserialized.email)
    }

    @Test
    fun `deserialize invalid data throws exception`() {
        val invalidBytes = byteArrayOf(0, 1, 2, 3, 4, 5) // Random bytes

        assertFailsWith<Exception> {
            JniBridge.deserializeUser(invalidBytes)
        }
    }
}