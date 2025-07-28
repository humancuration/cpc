package com.cpc.social.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.cpc.social.models.Post
import com.cpc.social.repository.SocialRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class TimelineViewModel : ViewModel() {
    private val repository = SocialRepository()
    
    private val _posts = MutableStateFlow<List<Post>>(emptyList())
    val posts: StateFlow<List<Post>> = _posts.asStateFlow()
    
    private val _isLoading = MutableStateFlow(false)
    val isLoading: StateFlow<Boolean> = _isLoading.asStateFlow()
    
    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error.asStateFlow()

    init {
        loadTimeline()
    }

    fun loadTimeline(limit: Int = 20, offset: Int = 0) {
        viewModelScope.launch {
            _isLoading.value = true
            _error.value = null
            try {
                val timeline = repository.getTimeline(limit, offset)
                _posts.value = if (offset == 0) timeline else _posts.value + timeline
            } catch (e: Exception) {
                _error.value = e.message ?: "Failed to load timeline"
            } finally {
                _isLoading.value = false
            }
        }
    }

    fun refreshTimeline() {
        loadTimeline()
    }

    fun loadMorePosts() {
        if (!_isLoading.value) {
            loadTimeline(limit = 20, offset = _posts.value.size)
        }
    }
}