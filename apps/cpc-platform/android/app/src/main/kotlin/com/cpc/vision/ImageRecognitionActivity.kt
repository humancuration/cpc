package com.cpc.vision

import android.Manifest
import android.content.pm.PackageManager
import android.graphics.Bitmap
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.camera.core.*
import androidx.camera.lifecycle.ProcessCameraProvider
import androidx.camera.view.PreviewView
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.content.ContextCompat
import androidx.lifecycle.LifecycleOwner
import com.cpc.vision.models.RecognitionItem
import com.cpc.vision.models.RecognitionResult
import com.google.gson.Gson
import kotlinx.coroutines.*
import java.util.concurrent.Executors
import kotlin.coroutines.resume
import kotlin.coroutines.suspendCoroutine

class ImageRecognitionActivity : ComponentActivity() {
    private lateinit var cameraProvider: ProcessCameraProvider
    private lateinit var preview: Preview
    private lateinit var imageCapture: ImageCapture
    private lateinit var imageAnalysis: ImageAnalysis
    private val executor = Executors.newSingleThreadExecutor()
    private val gson = Gson()
    
    private val requestPermissionLauncher = registerForActivityResult(
        ActivityResultContracts.RequestPermission()
    ) { isGranted ->
        if (isGranted) {
            startCamera()
        } else {
            Toast.makeText(this, "Camera permission required", Toast.LENGTH_SHORT).show()
            finish()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        if (ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.CAMERA
            ) == PackageManager.PERMISSION_GRANTED
        ) {
            startCamera()
        } else {
            requestPermissionLauncher.launch(Manifest.permission.CAMERA)
        }
        
        setContent {
            ImageRecognitionScreen()
        }
    }

    private fun startCamera() {
        val cameraProviderFuture = ProcessCameraProvider.getInstance(this)
        cameraProviderFuture.addListener({
            cameraProvider = cameraProviderFuture.get()
            bindCameraUseCases()
        }, ContextCompat.getMainExecutor(this))
    }

    private fun bindCameraUseCases() {
        val cameraSelector = CameraSelector.DEFAULT_BACK_CAMERA
        
        preview = Preview.Builder()
            .setTargetAspectRatio(AspectRatio.RATIO_16_9)
            .build()
        
        imageCapture = ImageCapture.Builder()
            .setCaptureMode(ImageCapture.CAPTURE_MODE_MINIMIZE_LATENCY)
            .build()
        
        imageAnalysis = ImageAnalysis.Builder()
            .setBackpressureStrategy(ImageAnalysis.STRATEGY_KEEP_ONLY_LATEST)
            .build()
        
        imageAnalysis.setAnalyzer(executor) { imageProxy ->
            processImage(imageProxy)
        }
        
        cameraProvider.unbindAll()
        cameraProvider.bindToLifecycle(
            this as LifecycleOwner,
            cameraSelector,
            preview,
            imageCapture,
            imageAnalysis
        )
    }

    private fun processImage(imageProxy: ImageProxy) {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                val bitmap = imageProxy.toBitmap()
                val result = performRecognition(bitmap)
                
                withContext(Dispatchers.Main) {
                    // Update UI with results
                    RecognitionResultHolder.setResult(result)
                }
            } catch (e: Exception) {
                Log.e("ImageRecognition", "Error processing image", e)
            } finally {
                imageProxy.close()
            }
        }
    }

    private suspend fun performRecognition(bitmap: Bitmap): RecognitionResult {
        return withContext(Dispatchers.IO) {
            try {
                val jsonResult = ImageRecognitionNative.recognize(bitmap)
                gson.fromJson(jsonResult, RecognitionResult::class.java)
            } catch (e: Exception) {
                RecognitionResult(
                    items = emptyList(),
                    processingTimeMs = 0,
                    imageWidth = bitmap.width,
                    imageHeight = bitmap.height
                )
            }
        }
    }

    @Composable
    private fun ImageRecognitionScreen() {
        val results by RecognitionResultHolder.results.collectAsState()
        
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp)
        ) {
            CameraPreview(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(300.dp)
            )
            
            Spacer(modifier = Modifier.height(16.dp))
            
            Text(
                text = "Recognition Results",
                style = MaterialTheme.typography.headlineSmall
            )
            
            Spacer(modifier = Modifier.height(8.dp))
            
            RecognitionResultsList(results)
        }
    }

    @Composable
    private fun CameraPreview(modifier: Modifier = Modifier) {
        val context = LocalContext.current
        
        AndroidView(
            factory = { ctx ->
                PreviewView(ctx).apply {
                    implementationMode = PreviewView.ImplementationMode.COMPATIBLE
                    scaleType = PreviewView.ScaleType.FILL_CENTER
                }
            },
            modifier = modifier,
            update = { previewView ->
                previewView.controller = cameraProvider.bindToLifecycle(
                    context as LifecycleOwner,
                    CameraSelector.DEFAULT_BACK_CAMERA,
                    preview
                )
            }
        )
    }

    @Composable
    private fun RecognitionResultsList(result: RecognitionResult) {
        if (result.items.isEmpty()) {
            Text(
                text = "No objects detected",
                style = MaterialTheme.typography.bodyMedium,
                color = Color.Gray
            )
        } else {
            LazyColumn {
                items(result.items) { item ->
                    RecognitionResultItem(item)
                }
            }
        }
    }

    @Composable
    private fun RecognitionResultItem(item: RecognitionItem) {
        Card(
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 4.dp),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceVariant
            )
        ) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(
                    text = item.label,
                    style = MaterialTheme.typography.bodyLarge
                )
                Text(
                    text = "%.1f%%".format(item.confidence * 100),
                    style = MaterialTheme.typography.bodyMedium,
                    color = Color.Gray
                )
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        executor.shutdown()
    }
}

// Object to hold recognition results for Compose
object RecognitionResultHolder {
    private val _results = mutableStateOf(
        RecognitionResult(
            items = emptyList(),
            processingTimeMs = 0,
            imageWidth = 0,
            imageHeight = 0
        )
    )
    val results: State<RecognitionResult> = _results
    
    fun setResult(result: RecognitionResult) {
        _results.value = result
    }
}

// Extension function to convert ImageProxy to Bitmap
fun ImageProxy.toBitmap(): Bitmap {
    val yBuffer = planes[0].buffer
    val uBuffer = planes[1].buffer
    val vBuffer = planes[2].buffer
    
    val ySize = yBuffer.remaining()
    val uSize = uBuffer.remaining()
    val vSize = vBuffer.remaining()
    
    val nv21 = ByteArray(ySize + uSize + vSize)
    
    yBuffer.get(nv21, 0, ySize)
    vBuffer.get(nv21, ySize, vSize)
    uBuffer.get(nv21, ySize + vSize, uSize)
    
    val yuvImage = android.graphics.YuvImage(
        nv21,
        android.graphics.ImageFormat.NV21,
        width,
        height,
        null
    )
    
    val out = java.io.ByteArrayOutputStream()
    yuvImage.compressToJpeg(android.graphics.Rect(0, 0, width, height), 100, out)
    val yuv = out.toByteArray()
    
    return android.graphics.BitmapFactory.decodeByteArray(yuv, 0, yuv.size)
}