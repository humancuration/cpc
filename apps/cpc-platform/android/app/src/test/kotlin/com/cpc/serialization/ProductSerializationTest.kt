package com.cpc.serialization

import com.cpc.android.JniBridge
import com.cpc.social.models.Product
import org.junit.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class ProductSerializationTest {

    @Test
    fun `product round-trip serialization`() {
        val cost = Money(19.99, "USD")
        val location = WarehouseLocation("loc-1", "Main Warehouse")
        val original = Product(
            id = "prod123",
            name = "Widget",
            brand = null,
            description = "A useful widget",
            barcode = null,
            carbonFootprint = null,
            packagingType = null,
            nutritionalInfo = null,
            manufacturer = null,
            materialCost = null,
            laborCost = null,
            supplier = null,
            currentStock = null,
            reorderLevel = null,
            supplyChain = null,
            cost = cost,
            location = location
        )

        val bytes = JniBridge.serializeProduct(original)
        val deserialized = JniBridge.deserializeProduct(bytes)

        assertEquals(original.id, deserialized.id)
        assertEquals(original.name, deserialized.name)
        assertEquals(original.description, deserialized.description)
        assertEquals(original.cost?.amount, deserialized.cost?.amount)
        assertEquals(original.location?.id, deserialized.location?.id)
    }

    @Test
    fun `serialize product with empty fields`() {
        val product = Product(
            id = "",
            name = "",
            brand = null,
            description = "",
            barcode = null,
            carbonFootprint = null,
            packagingType = null,
            nutritionalInfo = null,
            manufacturer = null,
            materialCost = null,
            laborCost = null,
            supplier = null,
            currentStock = null,
            reorderLevel = null,
            supplyChain = null,
            cost = null,
            location = null
        )

        val bytes = JniBridge.serializeProduct(product)
        val deserialized = JniBridge.deserializeProduct(bytes)

        assertEquals("", deserialized.id)
        assertEquals("", deserialized.name)
        assertEquals("", deserialized.description)
        assertEquals(null, deserialized.cost)
        assertEquals(null, deserialized.location)
    }

    @Test
    fun `serialize product with max length fields`() {
        val longString = "a".repeat(10000)
        val cost = Money(Double.MAX_VALUE, longString)
        val location = WarehouseLocation(longString, longString)
        val product = Product(
            id = longString,
            name = longString,
            brand = null,
            description = longString,
            barcode = null,
            carbonFootprint = null,
            packagingType = null,
            nutritionalInfo = null,
            manufacturer = null,
            materialCost = null,
            laborCost = null,
            supplier = null,
            currentStock = null,
            reorderLevel = null,
            supplyChain = null,
            cost = cost,
            location = location
        )

        val bytes = JniBridge.serializeProduct(product)
        val deserialized = JniBridge.deserializeProduct(bytes)

        assertEquals(longString, deserialized.id)
        assertEquals(longString, deserialized.name)
        assertEquals(longString, deserialized.description)
        assertEquals(cost.amount, deserialized.cost?.amount)
        assertEquals(cost.currency, deserialized.cost?.currency)
        assertEquals(location.id, deserialized.location?.id)
        assertEquals(location.name, deserialized.location?.name)
    }

    @Test
    fun `deserialize invalid data throws exception`() {
        val invalidBytes = byteArrayOf(0, 1, 2, 3, 4, 5)

        assertFailsWith<Exception> {
            JniBridge.deserializeProduct(invalidBytes)
        }
    }
}