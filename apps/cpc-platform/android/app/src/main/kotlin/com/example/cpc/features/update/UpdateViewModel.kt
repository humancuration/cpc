package com.example.cpc.features.update

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.cpc.network.BackendService
import com.example.cpc.network.UpdateCheckRequest
import com.example.cpc.network.UpdateCheckResponse
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class UpdateViewModel @Inject constructor(
    private val backendService: BackendService
) : ViewModel() {
    private val _updateState = MutableStateFlow<UpdateState>(UpdateState.Idle)
    val updateState: StateFlow<UpdateState> = _updateState

    fun checkUpdates(currentVersion: String) {
        _updateState.value = UpdateState.Loading
        viewModelScope.launch {
            try {
                val response = backendService.checkForUpdates(UpdateCheckRequest(currentVersion))
                _updateState.value = UpdateState.Success(response)
            } catch (e: Exception) {
                _updateState.value = UpdateState.Error(e.message ?: "Update check failed")
            }
        }
    }
}

sealed class UpdateState {
    object Idle : UpdateState()
    object Loading : UpdateState()
    data class Success(val response: UpdateCheckResponse) : UpdateState()
    data class Error(val message: String) : UpdateState()
}