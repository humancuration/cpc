package com.cpc.social.ffi

import android.content.Context
import java.io.File

object RustBridge {
    init {
        System.loadLibrary("cpc_core")
    }

    fun loadLibrary(context: Context) {
        val libraryDir = File(context.applicationInfo.nativeLibraryDir)
        val libFile = File(libraryDir, "libcpc_core.so")
        if (libFile.exists()) {
            System.load(libFile.absolutePath)
        } else {
            System.loadLibrary("cpc_core")
        }
    }
}