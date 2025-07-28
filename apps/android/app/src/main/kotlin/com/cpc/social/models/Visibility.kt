package com.cpc.social.models

import kotlinx.serialization.Serializable

@Serializable
enum class Visibility {
    PUBLIC,
    COOPERATIVE,
    PRIVATE
}