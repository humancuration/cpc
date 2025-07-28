package coop.cpc.platform.impact.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import coop.cpc.platform.network.GraphQLClient
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import java.util.UUID

class ImpactReportViewModel : ViewModel() {
    sealed class UiState {
        object Loading : UiState()
        data class Success(val report: ImpactReport) : UiState()
        data class Error(val message: String) : UiState()
    }

    private val _uiState = MutableStateFlow<UiState>(UiState.Loading)
    val uiState: StateFlow<UiState> = _uiState

    fun loadReport(userId: UUID) {
        _uiState.value = UiState.Loading
        
        viewModelScope.launch {
            try {
                val response = GraphQLClient.query(
                    query = """
                    query GetImpactReport(${'$'}userId: UUID!) {
                        impact_report(user_id: ${'$'}userId) {
                            user_id
                            generated_at
                            overall_score
                            ethical_distribution {
                                category
                                percentage
                            }
                            timeline {
                                timestamp
                                value
                                category
                            }
                            breakdown {
                                item_id
                                name
                                category
                                value
                                ethical_score
                            }
                            signature
                        }
                    }
                    """,
                    variables = mapOf("userId" to userId.toString())
                )
                
                val report = parseImpactReport(response)
                _uiState.value = UiState.Success(report)
            } catch (e: Exception) {
                _uiState.value = UiState.Error(e.message ?: "Unknown error")
            }
        }
    }
    
    private fun parseImpactReport(response: Map<String, Any>): ImpactReport {
        val data = response["data"] as Map<*, *>
        val reportData = data["impact_report"] as Map<*, *>
        
        return ImpactReport(
            userId = UUID.fromString(reportData["user_id"] as String),
            generatedAt = Instant.parse(reportData["generated_at"] as String),
            overallScore = reportData["overall_score"] as Double,
            ethicalDistribution = (reportData["ethical_distribution"] as List<*>).map {
                val entry = it as Map<*, *>
                ImpactCategory.valueOf(entry["category"] as String) to entry["percentage"] as Double
            }.toMap(),
            timeline = (reportData["timeline"] as List<*>).map {
                val point = it as Map<*, *>
                ImpactTimelinePoint(
                    timestamp = Instant.parse(point["timestamp"] as String),
                    value = point["value"] as Double,
                    category = ImpactCategory.valueOf(point["category"] as String)
                )
            },
            breakdown = (reportData["breakdown"] as List<*>).map {
                val item = it as Map<*, *>
                ImpactBreakdownItem(
                    itemId = UUID.fromString(item["item_id"] as String),
                    name = item["name"] as String,
                    category = ImpactCategory.valueOf(item["category"] as String),
                    value = item["value"] as Double,
                    ethicalScore = item["ethical_score"] as Double
                )
            },
            signature = reportData["signature"] as String
        )
    }
}