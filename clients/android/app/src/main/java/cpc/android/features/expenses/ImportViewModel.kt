package cpc.android.features.expenses

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import cpc.android.features.userpreferences.UserPreferencesManager
import kotlinx.coroutines.launch
import java.io.File
import java.util.*

/**
 * ViewModel for expense import functionality
 * 
 * This ViewModel handles the expense import process, including fetching user currency preferences
 * before processing and validating currency before starting the import.
 */
class ImportViewModel(application: Application) : AndroidViewModel(application) {
    
    private val userPreferencesManager = UserPreferencesManager(application)
    
    /**
     * Import expenses from a file
     * 
     * @param selectedFile The file to import from
     * @param onResult Callback for import result
     */
    fun importExpenses(selectedFile: File, onResult: (Result<ImportResult>) -> Unit) {
        viewModelScope.launch {
            try {
                // Fetch currency preference BEFORE processing
                val currency = userPreferencesManager.getPreferredCurrency()
                
                // Validate currency before processing starts
                if (!isValidCurrency(currency)) {
                    onResult(Result.failure(IllegalArgumentException("Invalid currency: $currency")))
                    return@launch
                }
                
                // In a real implementation, this would call the Rust expense import processor
                // For now, we'll simulate the import process
                val result = processExpenseImport(selectedFile, currency)
                
                onResult(Result.success(result))
            } catch (e: Exception) {
                onResult(Result.failure(e))
            }
        }
    }
    
    /**
     * Process expense import with the given currency
     * 
     * @param file The file to import from
     * @param currency The currency to use for all expenses
     * @return Import result
     */
    private fun processExpenseImport(file: File, currency: String): ImportResult {
        // In a real implementation, this would call the Rust expense import processor
        // through JNI, passing the currency parameter
        // For now, we'll simulate a successful import
        
        return ImportResult(
            totalRows = 10,
            successfulImports = 8,
            failedRows = listOf(
                FailedRow(3, "Invalid date format"),
                FailedRow(7, "Missing amount")
            )
        )
    }
    
    /**
     * Validate currency code
     * 
     * @param currency The currency code to validate
     * @return true if valid, false otherwise
     */
    private fun isValidCurrency(currency: String): Boolean {
        val supportedCurrencies = userPreferencesManager.getAllSupportedCurrencies()
        return supportedCurrencies.contains(currency)
    }
}

/**
 * Result of an expense import operation
 */
data class ImportResult(
    val totalRows: Int,
    val successfulImports: Int,
    val failedRows: List<FailedRow>
)

/**
 * Represents a failed row in the import
 */
data class FailedRow(
    val rowNumber: Int,
    val error: String
)