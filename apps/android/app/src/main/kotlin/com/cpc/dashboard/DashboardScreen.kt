package com.cpc.dashboard

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.cpc.android.JniBridge
import com.cpc.protos.metrics.AggregatedMetrics

@Composable
fun DashboardScreen() {
    var metrics by remember { mutableStateOf<AggregatedMetrics?>(null) }
    var loading by remember { mutableStateOf(true) }
    var error by remember { mutableStateOf<String?>(null) }
    var dateRange by remember { mutableStateOf("month") }
    var roles by remember { mutableStateOf(listOf<String>()) }

    LaunchedEffect(dateRange, roles) {
        try {
            loading = true
            metrics = JniBridge.getAggregatedMetrics(dateRange, roles.toTypedArray())
            error = null
        } catch (e: Exception) {
            error = e.message
        } finally {
            loading = false
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
            .verticalScroll(rememberScrollState()),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text("Cooperative Dashboard", style = MaterialTheme.typography.headlineMedium)

        Spacer(modifier = Modifier.height(24.dp))

        // Filter controls
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text("Date Range:", modifier = Modifier.padding(end = 8.dp))
            val ranges = listOf("day", "week", "month", "quarter", "year")
            var expanded by remember { mutableStateOf(false) }
            ExposedDropdownMenuBox(
                expanded = expanded,
                onExpandedChange = { expanded = !expanded }
            ) {
                TextField(
                    value = dateRange,
                    onValueChange = {},
                    readOnly = true,
                    trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
                    modifier = Modifier.menuAnchor()
                )
                ExposedDropdownMenu(
                    expanded = expanded,
                    onDismissRequest = { expanded = false }
                ) {
                    ranges.forEach { range ->
                        DropdownMenuItem(
                            text = { Text(range.capitalize()) },
                            onClick = {
                                dateRange = range
                                expanded = false
                            }
                        )
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(16.dp))

        // Metrics display
        when {
            loading -> CircularProgressIndicator()
            error != null -> Text("Error: $error", color = MaterialTheme.colorScheme.error)
            metrics != null -> MetricsDisplay(metrics!!)
        }

        Spacer(modifier = Modifier.height(24.dp))

        // Export PDF button
        Button(onClick = { 
            try {
                val pdfData = JniBridge.exportMetricsToPdf(dateRange, roles.toTypedArray())
                // TODO: Implement PDF save/share functionality
            } catch (e: Exception) {
                error = "PDF export failed: ${e.message}"
            }
        }) {
            Text("Export to PDF")
        }
    }
}

@Composable
fun MetricsDisplay(metrics: AggregatedMetrics) {
    val metricItems = listOf(
        "Total Members" to metrics.totalMembers.toString(),
        "Active Members" to metrics.activeMembers.toString(),
        "Total Products" to metrics.totalProducts.toString(),
        "Total Sales" to "$${"%.2f".format(metrics.totalSales)}",
        "Total Profit" to "$${"%.2f".format(metrics.totalProfit)}",
        "Carbon Saved" to "%.1f kg".format(metrics.totalCarbonSaved),
        "Avg Profit/Member" to "$${"%.2f".format(metrics.avgProfitPerMember)}",
        "Member Engagement" to "%.1f%%".format(metrics.memberEngagement * 100)
    )

    Column {
        metricItems.forEach { (title, value) ->
            Card(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(vertical = 8.dp),
            ) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text(title, style = MaterialTheme.typography.bodyLarge)
                    Text(value, style = MaterialTheme.typography.headlineSmall)
                }
            }
        }
    }
}