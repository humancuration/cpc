package com.cpc.musicplayer

import android.content.Context
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.util.AttributeSet
import android.view.View

/**
 * Custom canvas for visualizer rendering
 */
class VisualizerView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyleAttr: Int = 0
) : View(context, attrs, defStyleAttr) {
    
    private val paint = Paint().apply {
        color = Color.BLUE
        strokeWidth = 5f
        style = Paint.Style.STROKE
    }
    
    private var waveformPoints: FloatArray = floatArrayOf()
    
    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)
        
        // Draw waveform data
        if (waveformPoints.isNotEmpty()) {
            val width = width.toFloat()
            val height = height.toFloat()
            val pointCount = waveformPoints.size
            
            // Convert waveform data to points on the canvas
            val points = FloatArray(pointCount * 2)
            for (i in waveformPoints.indices) {
                val x = (i.toFloat() / pointCount) * width
                val y = (1 - waveformPoints[i]) * height
                points[i * 2] = x
                points[i * 2 + 1] = y
            }
            
            canvas.drawLines(points, paint)
        }
    }
    
    /**
     * Update the visualizer with new waveform data
     */
    fun updateWaveformData(waveform: FloatArray) {
        waveformPoints = waveform
        invalidate() // Trigger a redraw
    }
}