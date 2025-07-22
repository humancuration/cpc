package com.example.cpc.features.publish

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.cpc.network.BackendService
import com.example.cpc.network.PublishResponse
import com.example.cpc.auth.AuthManager
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class PublishViewModel @Inject constructor(
    private val backendService: BackendService,
    private val authManager: AuthManager
) : ViewModel() {
    private val _publishState = MutableStateFlow<PublishState>(PublishState.Idle)
    val publishState: StateFlow<PublishState> = _publishState

    fun publishProject(projectData: ByteArray) {
        _publishState.value = PublishState.Loading
        viewModelScope.launch {
            try {
                val token = authManager.getToken() ?: throw Exception("Not authenticated")
                val response = backendService.publishProject("Bearer $token", projectData)
                _publishState.value = PublishState.Success(response)
            } catch (e: Exception) {
                _publishState.value = PublishState.Error(e.message ?: "Publish failed")
            }
        }
    }
}

sealed class PublishState {
    object Idle : PublishState()
    object Loading : PublishState()
    data class Success(val response: PublishResponse) : PublishState()
    data class Error(val message: String) : PublishState()
}