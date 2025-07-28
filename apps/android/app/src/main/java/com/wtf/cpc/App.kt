package com.wtf.cpc

import android.app.Application
import com.wtf.cpc.network.ApiClient
import com.wtf.cpc.report.ReportHandler
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import android.util.Log

class App : Application() {
    val verifier = ImpactVerifier()
    val reportHandler = ReportHandler(verifier)
    var publicKey: String? = null
        private set

    override fun onCreate() {
        super.onCreate()
        fetchPublicKey()
    }

    private fun fetchPublicKey() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                publicKey = ApiClient.getPublicKey()
                Log.d("App", "Public key fetched successfully")
            } catch (e: Exception) {
                Log.e("App", "Failed to fetch public key", e)
                // Handle error appropriately (e.g., retry mechanism)
            }
        }
    }
}