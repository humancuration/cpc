package com.cpc.social.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.cpc.social.models.Post
import com.cpc.social.repository.SocialRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class PostViewModel : ViewModel() {
    private val repository = SocialRepository()
    
    private val _post = MutableStateFlow<Post?>(null)
    val post: StateFlow<Post?> = _post.asStateFlow()
    
    private val _isLoading = MutableStateFlow(false)
    val isLoading: StateFlow<Boolean> = _isLoading.asStateFlow()
    
    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error.asStateFlow()

    fun loadPost(postId: String) {
        viewModelScope.launch {
            _isLoading.value = true
            _error.value = null
            try {
                val loadedPost = repository.getPost(postId)
                _post.value = loadedPost
            } catch (e: Exception) {
                _error.value = e.message ?: "Failed to load post"
            } finally {
                _isLoading.value = false
            }
        }
    }

    fun createPost(content: String, visibility: String, cooperativeId: String? = null) {
        viewModelScope.launch {
            _isLoading.value = true
            _error.value = null
            try {
                val newPost = repository.createPost(content, visibility, cooperativeId)
                _post.value = newPost
            } catch (e: Exception) {
                _error.value = e.message ?: "Failed to create post"
            } finally {
                _isLoading.value = false
            }
        }
    }
}