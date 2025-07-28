package com.cpc.vision

import android.graphics.Bitmap

object ImageRecognitionNative {
    init {
        System.loadLibrary("cpc_core")
    }
    
    external fun recognize(bitmap: Bitmap): String
    
    external fun initRecognizer(
        modelPath: String,
        modelType: Int,
        inputWidth: Int,
        inputHeight: Int,
        confidenceThreshold: Float
    ): Long
    
    external fun recognizeWithRecognizer(recognizerPtr: Long, bitmap: Bitmap): String
    
    external fun destroyRecognizer(recognizerPtr: Long)
}