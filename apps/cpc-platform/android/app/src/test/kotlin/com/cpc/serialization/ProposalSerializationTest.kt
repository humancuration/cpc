package com.cpc.serialization

import com.cpc.android.JniBridge
import com.cpc.social.models.Proposal
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class ProposalSerializationTest {

    @Test
    fun `proposal round-trip serialization`() {
        val original = Proposal(
            id = "prop123",
            title = "New Feature",
            description = "Implement new analytics dashboard",
            authorId = "user456"
        )

        val bytes = JniBridge.serializeProposal(original)
        val deserialized = JniBridge.deserializeProposal(bytes)

        assertEquals(original.id, deserialized.id)
        assertEquals(original.title, deserialized.title)
        assertEquals(original.description, deserialized.description)
        assertEquals(original.authorId, deserialized.authorId)
    }

    @Test
    fun `serialize proposal with empty fields`() {
        val proposal = Proposal(
            id = "",
            title = "",
            description = "",
            authorId = ""
        )

        val bytes = JniBridge.serializeProposal(proposal)
        val deserialized = JniBridge.deserializeProposal(bytes)

        assertEquals("", deserialized.id)
        assertEquals("", deserialized.title)
        assertEquals("", deserialized.description)
        assertEquals("", deserialized.authorId)
    }

    @Test
    fun `serialize proposal with max length fields`() {
        val longString = "a".repeat(10000)
        val proposal = Proposal(
            id = longString,
            title = longString,
            description = longString,
            authorId = longString
        )

        val bytes = JniBridge.serializeProposal(proposal)
        val deserialized = JniBridge.deserializeProposal(bytes)

        assertEquals(longString, deserialized.id)
        assertEquals(longString, deserialized.title)
        assertEquals(longString, deserialized.description)
        assertEquals(longString, deserialized.authorId)
    }

    @Test
    fun `deserialize invalid data throws exception`() {
        val invalidBytes = byteArrayOf(0, 1, 2, 3, 4, 5)

        assertFailsWith<Exception> {
            JniBridge.deserializeProposal(invalidBytes)
        }
    }
}