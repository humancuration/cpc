package coop.cpc.platform.impact.ui

import androidx.compose.foundation.layout.*
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import coop.cpc.platform.impact.model.ImpactReport
import java.time.Instant
import java.time.LocalDate
import java.time.ZoneId
import java.time.format.DateTimeFormatter

@Composable
fun TimelineView(report: ImpactReport) {
    Column(
        modifier = Modifier
            .padding(16.dp)
            .fillMaxSize(),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = "Impact Over Time",
            style = MaterialTheme.typography.headlineSmall,
            modifier = Modifier.padding(bottom = 16.dp)
        )
        
        // Placeholder for timeline chart
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(300.dp)
                .background(MaterialTheme.colorScheme.surfaceVariant),
            contentAlignment = Alignment.Center
        ) {
            Text("Timeline Visualization")
        }
        
        Spacer(modifier = Modifier.height(24.dp))
        
        // Timeline points
        report.timeline.forEach { point ->
            TimelinePoint(point = point)
        }
    }
}

@Composable
fun TimelinePoint(point: ImpactTimelinePoint) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 8.dp),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Text(
            text = formatDate(point.timestamp),
            fontSize = 16.sp
        )
        Text(
            text = "%.1f".format(point.score),
            fontWeight = FontWeight.Bold,
            fontSize = 16.sp
        )
    }
}

private fun formatDate(timestamp: Long): String {
    val date = LocalDate.ofInstant(
        Instant.ofEpochMilli(timestamp),
        ZoneId.systemDefault()
    )
    val formatter = DateTimeFormatter.ofPattern("MMM yyyy")
    return date.format(formatter)
}