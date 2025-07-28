package coop.cpc.platform

import android.content.Context
import android.view.SurfaceHolder
import android.view.SurfaceView

class BevyView(context: Context) : SurfaceView(context), SurfaceHolder.Callback {
    init {
        holder.addCallback(this)
    }

    override fun surfaceCreated(holder: SurfaceHolder) {
        NativeBridge.initializeBevy(holder.surface)
    }

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
        NativeBridge.resizeBevy(width, height)
    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {
        NativeBridge.destroyBevy()
    }

    fun pause() {
        NativeBridge.pauseBevy()
    }

    fun resume() {
        NativeBridge.resumeBevy()
    }
}