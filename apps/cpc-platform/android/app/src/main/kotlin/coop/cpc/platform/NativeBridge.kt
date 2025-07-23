package coop.cpc.platform

import android.view.Surface

object NativeBridge {
    init {
        System.loadLibrary("cpc_platform")
    }

    // JNI functions from bevy_jni.rs
    external fun initializeBevy(surface: Surface)
    external fun resizeBevy(width: Int, height: Int)
    external fun destroyBevy()
    external fun sendGameEvent(eventType: String, data: String)
    external fun sendTextureToEngine(name: String, data: ByteArray, width: Int, height: Int)

    // Lifecycle functions for MainActivity
    external fun androidOnCreate()
    external fun androidOnPause()
    external fun androidOnResume()

    // Lifecycle functions for BevyView
    external fun pauseBevy()
    external fun resumeBevy()
}