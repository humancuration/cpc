package com.cpc.vision

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.recyclerview.widget.DiffUtil
import androidx.recyclerview.widget.ListAdapter
import androidx.recyclerview.widget.RecyclerView
import com.cpc.vision.models.RecognitionItem

class RecognitionResultAdapter : ListAdapter<RecognitionItem, RecognitionResultAdapter.ViewHolder>(DiffCallback) {

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val view = LayoutInflater.from(parent.context)
            .inflate(R.layout.item_recognition_result, parent, false)
        return ViewHolder(view)
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        holder.bind(getItem(position))
    }

    inner class ViewHolder(itemView: View) : RecyclerView.ViewHolder(itemView) {
        private val labelText: TextView = itemView.findViewById(R.id.labelText)
        private val confidenceText: TextView = itemView.findViewById(R.id.confidenceText)
        private val boundingBoxText: TextView = itemView.findViewById(R.id.boundingBoxText)

        fun bind(item: RecognitionItem) {
            labelText.text = item.label
            confidenceText.text = "%.1f%%".format(item.confidence * 100)
            
            item.boundingBox?.let { box ->
                boundingBoxText.text = "Box: (${box.left.toInt()}, ${box.top.toInt()}) - (${box.right.toInt()}, ${box.bottom.toInt()})"
                boundingBoxText.visibility = View.VISIBLE
            } ?: run {
                boundingBoxText.visibility = View.GONE
            }
        }
    }

    companion object DiffCallback : DiffUtil.ItemCallback<RecognitionItem>() {
        override fun areItemsTheSame(oldItem: RecognitionItem, newItem: RecognitionItem): Boolean {
            return oldItem.label == newItem.label && oldItem.confidence == newItem.confidence
        }

        override fun areContentsTheSame(oldItem: RecognitionItem, newItem: RecognitionItem): Boolean {
            return oldItem == newItem
        }
    }
}