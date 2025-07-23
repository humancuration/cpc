package com.cpc.social.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.cpc.social.models.Post
import com.cpc.social.models.Relationship
import com.cpc.social.repository.SocialRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class SocialViewModel(
    private val repository: SocialRepository
) : ViewModel() {

    private val _posts = MutableStateFlow<List<Post>>(emptyList())
    val posts: StateFlow<List<Post>> = _posts.asStateFlow()

    private val _relationships = MutableStateFlow<List<Relationship>>(emptyList())
    val relationships: StateFlow<List<Relationship>> = _relationships.asStateFlow()

    private val _loading = MutableStateFlow(false)
    val loading: StateFlow<Boolean> = _loading.asStateFlow()

    private val _error = MutableStateFlow<String?>(null)
    val error: StateFlow<String?> = _error.asStateFlow()

    fun loadUserPosts(userId: String) {
        viewModelScope.launch {
            _loading.value = true
            try {
                val posts = repository.getPostsByUser(userId)
                _posts.value = posts
            } catch (e: Exception) {
                _error.value = "Failed to load posts: ${e.message}"
            } finally {
                _loading.value = false
            }
        }
    }

    fun createPost(content: String, visibility: String = "PUBLIC") {
        viewModelScope.launch {
            _loading.value = true
            try {
                val post = repository.createPost(content, visibility)
                _posts.value = listOf(post) + _posts.value
            } catch (e: Exception) {
                _error.value = "Failed to create post: ${e.message}"
            } finally {
                _loading.value = false
            }
        }
    }

    fun followUser(userId: String) {
        viewModelScope.launch {
            _loading.value = true
            try {
                val relationship = repository.followUser(userId)
                _relationships.value = listOf(relationship) + _relationships.value
            } catch (e: Exception) {
                _error.value = "Failed to follow user: ${e.message}"
            } finally {
                _loading.value = false
            }
        }
    }

    fun unfollowUser(userId: String) {
        viewModelScope.launch {
            _loading.value = true
            try {
                val success = repository.unfollowUser(userId)
                if (success) {
                    _relationships.value = _relationships.value.filter { it.followingId != userId }
                }
            } catch (e: Exception) {
                _error.value = "Failed to unfollow user: ${e.message}"
            } finally {
                _loading.value = false
            }
        }
    }

    fun getFollowers(userId: String) {
        viewModelScope.launch {
            _loading.value = true
            try {
                val followers = repository.getFollowers(userId)
                _relationships.value = followers
            } catch (e: Exception) {
                _error.value = "Failed to load followers: ${e.message}"
            } finally {
                _loading.value = false
            }
        }
    }

    fun getFollowing(userId: String) {
        viewModelScope.launch {
            _loading.value = true
            try {
                val following = repository.getFollowing(userId)
                _relationships.value = following
            } catch (e: Exception) {
                _error.value = "Failed to load following: ${e.message}"
            } finally {
                _loading.value = false
            }
        }
    }
}