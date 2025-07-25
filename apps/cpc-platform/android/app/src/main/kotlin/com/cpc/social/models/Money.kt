package com.cpc.social.models

data class Money(
    val amount: Double,
    val currency: String
) {
    init {
        require(amount >= 0) { "Money amount cannot be negative" }
        require(currency.isNotEmpty()) { "Currency must be specified" }
    }
}