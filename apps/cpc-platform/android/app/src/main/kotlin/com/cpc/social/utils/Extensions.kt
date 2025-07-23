package com.cpc.social.utils

import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter

fun String.toFormattedDate(): String {
    return try {
        val instant = Instant.parse(this)
        val formatter = DateTimeFormatter.ofPattern("MMM dd, yyyy 'at' h:mm a")
            .withZone(ZoneId.systemDefault())
        formatter.format(instant)
    } catch (e: Exception) {
        this
    }
}

fun String.toRelativeTime(): String {
    return try {
        val instant = Instant.parse(this)
        val now = Instant.now()
        val seconds = java.time.Duration.between(instant, now).seconds
        
        when {
            seconds < 60 -> "just now"
            seconds < 3600 -> "${seconds / 60}m ago"
            seconds < 86400 -> "${seconds / 3600}h ago"
            seconds < 2592000 -> "${seconds / 86400}d ago"
            else -> toFormattedDate()
        }
    } catch (e: Exception) {
        this
    }
}