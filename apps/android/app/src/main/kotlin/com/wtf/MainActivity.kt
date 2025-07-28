// DEPRECATED: This file has been moved and refactored to coop.cpc.platform.MainActivity.kt
package com.wtf

import android.content.Intent
import android.os.Bundle
import android.view.View
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.FrameLayout
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    private lateinit var webView: WebView
    private lateinit var bevyView: BevyView
    
    private external fun androidOnCreate()
    private external fun androidOnPause()
    private external fun androidOnResume()

    @SuppressLint("SetJavaScriptEnabled")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Create container layout
        val container = FrameLayout(this).apply {
            layoutParams = FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
        }
        
        // Determine if we're in production build
        val isProduction = BuildConfig.BUILD_TYPE == "release"
        val url = if (isProduction) {
            "file:///android_asset/index.html"
        } else {
            "http://localhost:3000"
        }
        
        // Initialize WebView
        webView = WebView(this).apply {
            settings.javaScriptEnabled = true
            settings.domStorageEnabled = true
            
            // Set WebViewClient and AssetLoader for development
            if (!isProduction) {
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
            } else {
                webViewClient = WebViewClient()
            }
            
            loadUrl(url)
            layoutParams = FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
        }
        
        // Initialize BevyView
        bevyView = BevyView(this).apply {
            layoutParams = FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
            visibility = View.GONE // Hidden by default
        }
        
        // Add views to container
        container.addView(webView)
        container.addView(bevyView)
        setContentView(container)

        // Start Tauri background service
        startService(Intent(this, TauriService::class.java))
        androidOnCreate()
    }

    // Function to toggle between views
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
        androidOnPause()
        bevyView.pause()
    }
    
    override fun onResume() {
        super.onResume()
        androidOnResume()
        bevyView.resume()
    }
}