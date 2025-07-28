package com.cpc.social.models

import com.cpc.social.models.Money
import com.cpc.social.models.WarehouseLocation

data class Product(
    val id: String,
    val name: String,
    val brand: String? = null,
    val description: String,
    val barcode: String? = null,
    val carbonFootprint: Double? = null,
    val packagingType: String? = null,
    val nutritionalInfo: String? = null,
    val manufacturer: String? = null,
    val materialCost: Double? = null,
    val laborCost: Double? = null,
    val supplier: String? = null,
    val currentStock: Int? = null,
    val reorderLevel: Int? = null,
    val supplyChain: String? = null,
    val cost: Money? = null,
    val location: WarehouseLocation? = null
)