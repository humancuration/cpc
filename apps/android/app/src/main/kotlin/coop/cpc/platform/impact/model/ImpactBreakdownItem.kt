package coop.cpc.platform.impact.model

import java.util.UUID

data class ImpactBreakdownItem(
    val itemId: UUID,
    val name: String,
    val category: ImpactCategory,
    val value: Double,
    val ethicalScore: Double
)