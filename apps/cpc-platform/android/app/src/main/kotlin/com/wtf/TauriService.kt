package com.wtf

import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.IBinder
import androidx.core.app.NotificationCompat
import com.wtf.R

class TauriService : Service() {
    override fun onBind(intent: Intent): IBinder? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        // Initialize Tauri framework with Android context
        initializeTauri(applicationContext)
        
        startForeground(
            1,
            NotificationCompat.Builder(this, "tauri_channel")
                .setContentTitle("CPC Platform")
                .setContentText("Running in background")
                .setSmallIcon(R.mipmap.ic_launcher)
                .build()
        )
        return START_STICKY
    }

    private external fun initializeTauri(context: Context)
    
    companion object {
        init {
            System.loadLibrary("cpc_platform")
        }
    }
}