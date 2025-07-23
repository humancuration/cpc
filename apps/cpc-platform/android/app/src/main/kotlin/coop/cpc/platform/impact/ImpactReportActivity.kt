package coop.cpc.platform.impact

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import coop.cpc.platform.impact.ui.ImpactReportScreen
import java.util.UUID

class ImpactReportActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // In a real app, we would get the user ID from intent or shared preferences
        val userId = UUID.randomUUID() // Placeholder - replace with actual user ID
        
        setContent {
            ImpactReportScreen(userId = userId)
        }
    }
}