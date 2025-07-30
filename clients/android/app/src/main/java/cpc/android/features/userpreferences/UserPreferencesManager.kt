package cpc.android.features.userpreferences

import android.content.Context
import java.util.*

/**
 * Manager for user preferences, interfacing with Rust FFI layer
 * 
 * This class provides a Kotlin interface to the shared Rust user preferences logic.
 * It handles currency preferences and sync status.
 */
class UserPreferencesManager(private val context: Context) {
    
    companion object {
        // Load the Rust library
        init {
            System.loadLibrary("cpc_android")
        }
    }
    
    /**
     * Get the user's preferred currency
     * 
     * @return The currency code (e.g., "USD", "EUR")
     */
    external fun getPreferredCurrency(): String
    
    /**
     * Set the user's preferred currency
     * 
     * @param currencyCode The currency code to set
     * @return true if successful, false otherwise
     */
    external fun setPreferredCurrency(currencyCode: String): Boolean
    
    /**
     * Get all supported currencies
     * 
     * @return List of supported currency codes
     */
    fun getAllSupportedCurrencies(): List<String> {
        // In a real implementation, this would call a Rust function
        // For now, we'll return a static list of supported currencies
        return listOf(
            "USD", "EUR", "GBP", "JPY", "CAD", "AUD", "CHF", "CNY", "SEK", "NZD",
            "MXN", "SGD", "HKD", "NOK", "KRW", "TRY", "RUB", "INR", "BRL", "ZAR", "DABLOONS"
        )
    }
}