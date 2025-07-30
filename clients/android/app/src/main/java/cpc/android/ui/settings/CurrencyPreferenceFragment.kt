package cpc.android.ui.settings

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ArrayAdapter
import android.widget.Spinner
import android.widget.Toast
import androidx.fragment.app.Fragment
import cpc.android.R
import cpc.android.databinding.FragmentCurrencyPreferenceBinding
import cpc.android.features.userpreferences.UserPreferencesManager
import java.util.*

/**
 * Fragment for currency preference selection
 * 
 * This fragment allows users to select their preferred currency from a dropdown list.
 * It provides immediate UI feedback when selection changes and shows sync status.
 */
class CurrencyPreferenceFragment : Fragment() {
    
    private var _binding: FragmentCurrencyPreferenceBinding? = null
    private val binding get() = _binding!!
    
    private lateinit var currencySpinner: Spinner
    private lateinit var userPreferencesManager: UserPreferencesManager
    
    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        _binding = FragmentCurrencyPreferenceBinding.inflate(inflater, container, false)
        return binding.root
    }
    
    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        
        // Initialize components
        userPreferencesManager = UserPreferencesManager(requireContext())
        currencySpinner = binding.currencySpinner
        
        // Setup currency dropdown
        setupCurrencyDropdown()
        
        // Setup sync status indicator
        setupSyncStatusIndicator()
        
        // Load current preference
        loadCurrentPreference()
        
        // Setup save button
        binding.saveButton.setOnClickListener {
            saveCurrencyPreference()
        }
    }
    
    private fun setupCurrencyDropdown() {
        // Get all supported currencies from shared Rust logic
        val currencies = userPreferencesManager.getAllSupportedCurrencies()
        
        // Create adapter for spinner
        val adapter = ArrayAdapter(
            requireContext(),
            android.R.layout.simple_spinner_item,
            currencies
        ).apply {
            setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item)
        }
        
        currencySpinner.adapter = adapter
    }
    
    private fun setupSyncStatusIndicator() {
        // Setup visual feedback for sync status
        // This would show cloud icon with sync state
        binding.syncStatusIcon.setImageResource(R.drawable.ic_cloud_off)
    }
    
    private fun loadCurrentPreference() {
        try {
            val currentCurrency = userPreferencesManager.getPreferredCurrency()
            val currencies = userPreferencesManager.getAllSupportedCurrencies()
            val position = currencies.indexOf(currentCurrency)
            
            if (position >= 0) {
                currencySpinner.setSelection(position)
            }
        } catch (e: Exception) {
            Toast.makeText(
                requireContext(),
                "Failed to load currency preference: ${e.message}",
                Toast.LENGTH_SHORT
            ).show()
        }
    }
    
    private fun saveCurrencyPreference() {
        try {
            val selectedCurrency = currencySpinner.selectedItem as String
            userPreferencesManager.setPreferredCurrency(selectedCurrency)
            
            // Update UI immediately
            updateSyncStatus(true)
            
            Toast.makeText(
                requireContext(),
                "Currency preference saved",
                Toast.LENGTH_SHORT
            ).show()
        } catch (e: Exception) {
            Toast.makeText(
                requireContext(),
                "Failed to save currency preference: ${e.message}",
                Toast.LENGTH_SHORT
            ).show()
            
            // Update sync status to show error
            updateSyncStatus(false)
        }
    }
    
    private fun updateSyncStatus(synced: Boolean) {
        if (synced) {
            binding.syncStatusIcon.setImageResource(R.drawable.ic_cloud_done)
        } else {
            binding.syncStatusIcon.setImageResource(R.drawable.ic_cloud_off)
        }
    }
    
    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}