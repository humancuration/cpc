package com.cpc.social.utils

import java.time.Instant
import java.time.ZoneId
import java.time.format.DateTimeFormatter
import java.util.*

object DateUtils {
    private val ISO_FORMATTER = DateTimeFormatter.ISO_INSTANT
    private val DISPLAY_FORMATTER = DateTimeFormatter.ofPattern("MMM dd, yyyy 'at' h:mm a")
    private val TIME_FORMATTER = DateTimeFormatter.ofPattern("h:mm a")

    fun formatIsoDate(isoDate: String): String {
        return try {
            val instant = Instant.parse(isoDate)
            DISPLAY_FORMATTER.withZone(ZoneId.systemDefault()).format(instant)
        } catch (e: Exception) {
            isoDate
        }
    }

    fun formatTimeOnly(isoDate: String): String {
        return try {
            val instant = Instant.parse(isoDate)
            TIME_FORMATTER.withZone(ZoneId.systemDefault()).format(instant)
        } catch (e: Exception) {
            isoDate
        }
    }

    fun getCurrentIsoDate(): String {
        return ISO_FORMATTER.format(Instant.now())
    }

    fun getCurrentTimestamp(): Long {
        return System.currentTimeMillis()
    }

    fun formatRelativeTime(timestamp: Long): String {
        val now = System.currentTimeMillis()
        val diff = now - timestamp
        
        val seconds = diff / 1000
        val minutes = seconds / 60
        val hours = minutes / 60
        val days = hours / 24
        
        return when {
            days > 30 -> "${days / 30}mo ago"
            days > 0 -> "${days}d ago"
            hours > 0 -> "${hours}h ago"
            minutes > 0 -> "${minutes}m ago"
            seconds > 0 -> "${seconds}s ago"
            else -> "now"
        }
    }
}