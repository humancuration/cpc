package coop.cpc.platform

import android.annotation.SuppressLint
import android.content.Intent
import android.graphics.BitmapFactory
import android.os.Bundle
import android.view.View
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.Button
import android.widget.FrameLayout
import android.widget.LinearLayout
import android.view.Gravity
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AppCompatActivity
import androidx.webkit.WebViewAssetLoader
import com.wtf.TauriService
import java.io.IOException

class MainActivity : AppCompatActivity() {
    private lateinit var webView: WebView
    private lateinit var bevyView: BevyView

    private val imagePickerLauncher = registerForActivityResult(ActivityResultContracts.GetContent()) { uri ->
        uri?.let {
            try {
                // Get image bytes
                val inputStream = contentResolver.openInputStream(it)
                val imageData = inputStream?.readBytes()
                inputStream?.close()

                // Get image dimensions without loading the whole bitmap into memory
                val options = BitmapFactory.Options().apply { inJustDecodeBounds = true }
                val dimensionsInputStream = contentResolver.openInputStream(it)
                BitmapFactory.decodeStream(dimensionsInputStream, null, options)
                dimensionsInputStream?.close()

                val imageWidth = options.outWidth
                val imageHeight = options.outHeight

                if (imageData != null && imageWidth > 0 && imageHeight > 0) {
                    NativeBridge.sendTextureToEngine("user_selected_image", imageData, imageWidth, imageHeight)
                    // Navigate to impact report after image selection
                    val intent = Intent(this@MainActivity, ImpactReportActivity::class.java)
                    startActivity(intent)
                }
            } catch (e: IOException) {
                e.printStackTrace()
            }
        }
    }

    @SuppressLint("SetJavaScriptEnabled", "WebViewApiAvailability")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Load Rust JNI library for serialization
        System.loadLibrary("cpc_jni")

        val container = FrameLayout(this).apply {
            layoutParams = FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
        }

        val isProduction = false // Simplified for now, replace with BuildConfig logic if needed
        val url = if (isProduction) "https://app.tauri.studio/index.html" else "http://localhost:3000"

        webView = WebView(this).apply {
            settings.javaScriptEnabled = true
            settings.domStorageEnabled = true
            settings.setSupportMultipleWindows(true)

            val assetLoader = WebViewAssetLoader.Builder()
                .setDomain("localhost")
                .addPathHandler("/", WebViewAssetLoader.AssetsPathHandler(this@MainActivity))
                .build()

            webViewClient = object : WebViewClient() {
                override fun shouldInterceptRequest(
                    view: WebView,
                    request: WebResourceRequest
                ): WebResourceResponse? {
                    return assetLoader.shouldInterceptRequest(request.url)
                }
            }
            loadUrl(url)
        }
        
        container.addView(webView, FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT)

        bevyView = BevyView(this).apply {
            visibility = View.GONE
        }
        container.addView(bevyView, FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT)

        val imagePickerButton = Button(this).apply {
            text = "Pick Image"
            setOnClickListener {
                imagePickerLauncher.launch("image/*")
            }
        }
        
        val impactReportButton = Button(this).apply {
            text = "View Impact Report"
            setOnClickListener {
                val intent = Intent(this@MainActivity, ImpactReportActivity::class.java)
                startActivity(intent)
            }
        }
        
        val buttonsLayout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            gravity = Gravity.CENTER_HORIZONTAL
            addView(imagePickerButton)
            addView(impactReportButton)
        }
        
        val buttonsParams = FrameLayout.LayoutParams(
            FrameLayout.LayoutParams.WRAP_CONTENT,
            FrameLayout.LayoutParams.WRAP_CONTENT
        ).apply {
            // Position buttons at bottom center
            gravity = android.view.Gravity.BOTTOM or android.view.Gravity.CENTER_HORIZONTAL
            bottomMargin = 100
        }
        container.addView(buttonsLayout, buttonsParams)
        
        setContentView(container)

        startService(Intent(this, TauriService::class.java))
        NativeBridge.androidOnCreate()
    }

    fun showBevyView(show: Boolean) {
        runOnUiThread {
            bevyView.visibility = if (show) View.VISIBLE else View.GONE
            webView.visibility = if (show) View.GONE else View.VISIBLE

            if (show) {
                bevyView.resume()
            } else {
                bevyView.pause()
            }
        }
    }

    override fun onPause() {
        super.onPause()
        NativeBridge.androidOnPause()
        bevyView.pause()
    }

    override fun onResume() {
        super.onResume()
        NativeBridge.androidOnResume()
        bevyView.resume()
    }
}