package coop.cpc.platform.impact.ui

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import coop.cpc.platform.impact.model.ImpactBreakdownItem
import coop.cpc.platform.impact.model.ImpactReport

@Composable
fun BreakdownView(report: ImpactReport) {
    LazyColumn(
        modifier = Modifier
            .padding(16.dp)
            .fillMaxSize()
    ) {
        item {
            Text(
                text = "Detailed Breakdown",
                style = MaterialTheme.typography.headlineSmall,
                modifier = Modifier.padding(bottom = 16.dp)
            )
        }
        
        items(report.breakdown) { item ->
            BreakdownItemCard(item = item)
        }
    }
}

@Composable
fun BreakdownItemCard(item: ImpactBreakdownItem) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 8.dp),
        elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(
                    text = item.item_name,
                    fontWeight = FontWeight.Bold,
                    fontSize = 18.sp
                )
                Text(
                    text = "${item.contribution.toInt()}% contribution",
                    fontSize = 16.sp
                )
            }
            
            Spacer(modifier = Modifier.height(8.dp))
            
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text("Impact Score:")
                Text(
                    text = "%.1f".format(item.impact_score),
                    fontWeight = FontWeight.Bold,
                    color = when {
                        item.impact_score > 7.5 -> Color.Green
                        item.impact_score > 5.0 -> Color(0xFFFFA500) // Orange
                        else -> Color.Red
                    }
                )
            }
        }
    }
}