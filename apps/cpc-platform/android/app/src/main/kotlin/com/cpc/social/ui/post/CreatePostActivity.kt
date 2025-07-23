package com.cpc.social.ui.post

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.cpc.social.viewmodel.PostViewModel

class CreatePostActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            CreatePostScreen()
        }
    }
}

@Composable
fun CreatePostScreen(
    viewModel: PostViewModel = viewModel()
) {
    var content by remember { mutableStateOf("") }
    var visibility by remember { mutableStateOf("PUBLIC") }
    var cooperativeId by remember { mutableStateOf("") }
    
    val isLoading by viewModel.isLoading.collectAsState()
    val error by viewModel.error.collectAsState()
    val post by viewModel.post.collectAsState()

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        Text(
            text = "Create Post",
            style = MaterialTheme.typography.headlineMedium,
            modifier = Modifier.padding(bottom = 16.dp)
        )

        OutlinedTextField(
            value = content,
            onValueChange = { content = it },
            label = { Text("Content") },
            modifier = Modifier
                .fillMaxWidth()
                .height(200.dp),
            maxLines = 10
        )

        Spacer(modifier = Modifier.height(16.dp))

        OutlinedTextField(
            value = visibility,
            onValueChange = { visibility = it },
            label = { Text("Visibility (PUBLIC, COOPERATIVE, PRIVATE)") },
            modifier = Modifier.fillMaxWidth()
        )

        Spacer(modifier = Modifier.height(16.dp))

        OutlinedTextField(
            value = cooperativeId,
            onValueChange = { cooperativeId = it },
            label = { Text("Cooperative ID (optional)") },
            modifier = Modifier.fillMaxWidth()
        )

        Spacer(modifier = Modifier.height(24.dp))

        Button(
            onClick = {
                viewModel.createPost(
                    content = content,
                    visibility = visibility,
                    cooperativeId = cooperativeId.ifBlank { null }
                )
            },
            modifier = Modifier.fillMaxWidth(),
            enabled = content.isNotBlank() && !isLoading
        ) {
            if (isLoading) {
                CircularProgressIndicator(
                    modifier = Modifier.size(24.dp),
                    strokeWidth = 2.dp
                )
            } else {
                Text("Create Post")
            }
        }

        error?.let { errorMessage ->
            Spacer(modifier = Modifier.height(16.dp))
            Card(
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.errorContainer
                )
            ) {
                Text(
                    text = errorMessage,
                    color = MaterialTheme.colorScheme.onErrorContainer,
                    modifier = Modifier.padding(16.dp)
                )
            }
        }

        post?.let { createdPost ->
            Spacer(modifier = Modifier.height(16.dp))
            Card {
                Text(
                    text = "Post created successfully!",
                    modifier = Modifier.padding(16.dp)
                )
            }
        }
    }
}