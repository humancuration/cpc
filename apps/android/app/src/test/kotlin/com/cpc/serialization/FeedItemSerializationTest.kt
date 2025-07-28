package com.cpc.serialization

import com.cpc.android.JniBridge
import com.cpc.social.models.FeedItem
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class FeedItemSerializationTest {

    @Test
    fun `feed item round-trip serialization`() {
        val original = FeedItem(
            id = "feed123",
            type = "post",
            content = "This is a test feed item"
        )

        val bytes = JniBridge.serializeFeedItem(original)
        val deserialized = JniBridge.deserializeFeedItem(bytes)

        assertEquals(original.id, deserialized.id)
        assertEquals(original.type, deserialized.type)
        assertEquals(original.content, deserialized.content)
    }

    @Test
    fun `serialize feed item with empty fields`() {
        val feedItem = FeedItem(
            id = "",
            type = "",
            content = ""
        )

        val bytes = JniBridge.serializeFeedItem(feedItem)
        val deserialized = JniBridge.deserializeFeedItem(bytes)

        assertEquals("", deserialized.id)
        assertEquals("", deserialized.type)
        assertEquals("", deserialized.content)
    }

    @Test
    fun `serialize feed item with max length fields`() {
        val longString = "a".repeat(10000)
        val feedItem = FeedItem(
            id = longString,
            type = longString,
            content = longString
        )

        val bytes = JniBridge.serializeFeedItem(feedItem)
        val deserialized = JniBridge.deserializeFeedItem(bytes)

        assertEquals(longString, deserialized.id)
        assertEquals(longString, deserialized.type)
        assertEquals(longString, deserialized.content)
    }

    @Test
    fun `deserialize invalid data throws exception`() {
        val invalidBytes = byteArrayOf(0, 1, 2, 3, 4, 5)

        assertFailsWith<Exception> {
            JniBridge.deserializeFeedItem(invalidBytes)
        }
    }
}