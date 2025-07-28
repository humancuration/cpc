package coop.cpc.platform.impact.repository

import coop.cpc.platform.network.CpcApolloClient
import coop.cpc.platform.impact.model.ImpactCategory
import coop.cpc.platform.impact.model.ImpactReport
import coop.cpc.platform.impact.model.ImpactTimelinePoint
import coop.cpc.platform.impact.model.ImpactBreakdownItem
import java.time.Instant
import java.util.UUID
import javax.inject.Inject

interface ImpactRepository {
    suspend fun getImpactReport(userId: UUID): ImpactReport
}

class ImpactRepositoryImpl @Inject constructor(
    private val apolloClient: CpcApolloClient
) : ImpactRepository {
    override suspend fun getImpactReport(userId: UUID): ImpactReport {
        val result = apolloClient.query(GetImpactReportQuery(userId.toString())).execute()
        
        if (result.errors != null && result.errors!!.isNotEmpty()) {
            throw Exception("GraphQL error: ${result.errors!!.first().message}")
        }
        
        val data = result.data?.getImpactReport ?: throw Exception("No data returned")
        
        return ImpactReport(
            userId = userId,
            generatedAt = Instant.parse(data.generatedAt),
            overallScore = data.overallScore,
            ethicalDistribution = data.ethicalDistribution.associate {
                ImpactCategory.valueOf(it.category) to it.value
            },
            timeline = data.timeline.map {
                ImpactTimelinePoint(
                    timestamp = Instant.parse(it.timestamp),
                    value = it.value,
                    category = ImpactCategory.valueOf(it.category)
                )
            },
            breakdown = data.breakdown.map {
                ImpactBreakdownItem(
                    itemId = UUID.fromString(it.itemId),
                    name = it.name,
                    category = ImpactCategory.valueOf(it.category),
                    value = it.value,
                    ethicalScore = it.ethicalScore
                )
            },
            signature = data.signature
        )
    }
}