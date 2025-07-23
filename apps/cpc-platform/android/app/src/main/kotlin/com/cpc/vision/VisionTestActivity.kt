package com.cpc.vision

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

class VisionTestActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        setContent {
            VisionTestScreen(
                onStartRecognition = {
                    startActivity(Intent(this, ImageRecognitionActivity::class.java))
                }
            )
        }
    }
}

@Composable
fun VisionTestScreen(onStartRecognition: () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Text(
            text = "CPC Vision Test",
            style = MaterialTheme.typography.headlineMedium
        )
        
        Spacer(modifier = Modifier.height(32.dp))
        
        Button(
            onClick = onStartRecognition,
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("Start Image Recognition")
        }
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Text(
            text = "This will open the camera and start real-time object detection",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
    }
}