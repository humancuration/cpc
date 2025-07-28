package com.cpc.vision.models

import kotlinx.serialization.Serializable

@Serializable
data class RecognitionResult(
    val items: List<RecognitionItem>,
    val processingTimeMs: Long,
    val imageWidth: Int,
    val imageHeight: Int
)

@Serializable
data class RecognitionItem(
    val label: String,
    val confidence: Float,
    val boundingBox: BoundingBox? = null
)

@Serializable
data class BoundingBox(
    val left: Float,
    val top: Float,
    val right: Float,
    val bottom: Float
)