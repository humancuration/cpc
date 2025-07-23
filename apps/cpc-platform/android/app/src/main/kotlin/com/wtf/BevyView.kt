// DEPRECATED: This file has been moved and refactored to coop.cpc.platform.BevyView.kt
package com.wtf

import android.content.Context
import android.view.SurfaceHolder
import android.view.SurfaceView

class BevyView(context: Context) : SurfaceView(context), SurfaceHolder.Callback {
    init {
        holder.addCallback(this)
    }

    override fun surfaceCreated(holder: SurfaceHolder) {
        // Initialize Bevy when surface is created
        initializeBevy(holder.surface)
    }

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
        // Handle surface size changes
        resizeBevy(width, height)
    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {
        // Clean up Bevy resources
        destroyBevy()
    }

    private external fun initializeBevy(surface: Any)
    private external fun resizeBevy(width: Int, height: Int)
    private external fun destroyBevy()
    
    fun pause() {
        androidPauseBevyThread()
    }
    
    fun resume() {
        androidResumeBevyThread()
    }
    
    private external fun androidPauseBevyThread()
    private external fun androidResumeBevyThread()

    companion object {
        init {
            System.loadLibrary("cpc_platform")
        }
    }
}