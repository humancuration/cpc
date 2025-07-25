package com.cpc.social.models

data class WarehouseLocation(
    val id: String,
    val name: String
) {
    init {
        require(id.isNotBlank()) { "WarehouseLocation id cannot be blank" }
    }
}