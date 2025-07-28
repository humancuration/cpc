package coop.cpc.platform.impact.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import coop.cpc.platform.impact.model.ImpactReport
import coop.cpc.platform.impact.repository.ImpactRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import java.util.UUID
import javax.inject.Inject

@HiltViewModel
class ImpactViewModel @Inject constructor(
    private val repository: ImpactRepository
) : ViewModel() {

    private val _reportState = MutableStateFlow<ImpactReport?>(null)
    val reportState: StateFlow<ImpactReport?> = _reportState

    private val _isLoading = MutableStateFlow(false)
    val isLoading: StateFlow<Boolean> = _isLoading

    private val _errorState = MutableStateFlow<Throwable?>(null)
    val errorState: StateFlow<Throwable?> = _errorState

    fun loadReport(userId: UUID) {
        _isLoading.value = true
        _errorState.value = null
        
        viewModelScope.launch {
            try {
                val report = repository.getImpactReport(userId)
                _reportState.value = report
            } catch (e: Exception) {
                _errorState.value = e
            } finally {
                _isLoading.value = false
            }
        }
    }
}