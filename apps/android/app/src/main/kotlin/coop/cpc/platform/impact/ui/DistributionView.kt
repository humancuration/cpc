package coop.cpc.platform.impact.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import coop.cpc.platform.impact.model.ImpactCategory
import coop.cpc.platform.impact.model.ImpactReport

@Composable
fun DistributionView(report: ImpactReport) {
    Column(
        modifier = Modifier
            .padding(16.dp)
            .fillMaxSize(),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = "Ethical Distribution",
            style = MaterialTheme.typography.headlineSmall,
            modifier = Modifier.padding(bottom = 16.dp)
        )
        
        // Placeholder for pie chart - in real app use a chart library
        Box(
            modifier = Modifier
                .size(200.dp)
                .background(Color.LightGray, CircleShape),
            contentAlignment = Alignment.Center
        ) {
            Text("Pie Chart Visualization")
        }
        
        Spacer(modifier = Modifier.height(24.dp))
        
        // Distribution stats
        report.distribution.forEach { category ->
            DistributionRow(category = category)
        }
    }
}

@Composable
fun DistributionRow(category: ImpactCategory) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 8.dp),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Text(
            text = category.category,
            fontSize = 16.sp
        )
        Text(
            text = "${(category.weight * 100).toInt()}%",
            fontWeight = FontWeight.Bold,
            fontSize = 16.sp
        )
    }
}