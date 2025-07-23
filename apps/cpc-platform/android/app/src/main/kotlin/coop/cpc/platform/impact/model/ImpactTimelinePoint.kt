package coop.cpc.platform.impact.model

import java.time.Instant

data class ImpactTimelinePoint(
    val timestamp: Instant,
    val value: Double,
    val category: ImpactCategory
)