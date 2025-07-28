package coop.cpc.impact

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import coop.cpc.GetPublicKeyQuery
import coop.cpc.utils.GraphQLClient
import coop.cpc.verification.SignatureVerifier
import coop.cpc.verification.VerificationResult
import com.apollographql.apollo3.api.ApolloResponse
import com.apollographql.apollo3.exception.ApolloException
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

class ImpactVerificationViewModel(application: Application) : AndroidViewModel(application) {
    private val _verificationState = MutableStateFlow<VerificationState>(VerificationState.Idle)
    val verificationState: StateFlow<VerificationState> = _verificationState

    fun verifyReport(reportJson: String) {
        viewModelScope.launch {
            _verificationState.value = VerificationState.Loading
            try {
                val publicKey = fetchPublicKey()
                val verifier = SignatureVerifier()
                val resultPtr = verifier.verifyImpactReport(reportJson, publicKey)
                val result = verifier.toResult(resultPtr)
                verifier.freeVerificationResult(resultPtr)

                _verificationState.value = when (result) {
                    is VerificationResult.Valid -> VerificationState.Valid
                    is VerificationResult.Invalid -> VerificationState.Invalid(result.error)
                }
            } catch (e: Exception) {
                _verificationState.value = VerificationState.Error(e.message ?: "Unknown error")
            }
        }
    }

    private suspend fun fetchPublicKey(): String {
        val apolloClient = GraphQLClient.getApolloClient(getApplication())
        try {
            val response: ApolloResponse<GetPublicKeyQuery.Data> =
                apolloClient.query(GetPublicKeyQuery()).execute()
            
            if (response.hasErrors()) {
                throw Exception(response.errors?.firstOrNull()?.message ?: "GraphQL error")
            }
            
            return response.data?.impactPublicKey ?: throw Exception("No public key returned")
        } catch (e: ApolloException) {
            throw Exception("Network error: ${e.message}")
        }
    }
}

sealed class VerificationState {
    object Idle : VerificationState()
    object Loading : VerificationState()
    object Valid : VerificationState()
    data class Invalid(val error: String) : VerificationState()
    data class Error(val message: String) : VerificationState()
}