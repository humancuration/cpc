package com.cpc.social.models

data class Post(
    val id: String,
    val content: String,
    val authorId: String,
    val likes: Int,
    val comments: List<Comment>
)