package com.cpc.android

import com.cpc.social.models.User
import com.cpc.social.models.Post
import com.cpc.social.models.Comment
import com.cpc.social.models.Proposal
import com.cpc.social.models.FeedItem
import com.cpc.social.models.Product

object JniBridge {
    external fun serializeUser(user: User): ByteArray
    external fun deserializeUser(bytes: ByteArray): User
    external fun serializePost(post: Post): ByteArray
    external fun deserializePost(bytes: ByteArray): Post
    external fun deserializeComment(bytes: ByteArray): Comment
    external fun serializeComment(comment: Comment): ByteArray
    external fun serializeProposal(proposal: Proposal): ByteArray
    external fun serializeFeedItem(feedItem: FeedItem): ByteArray
    external fun serializeProduct(product: Product): ByteArray
    
    external fun deserializeProposal(protoBytes: ByteArray): Long
    external fun deserializeFeedItem(protoBytes: ByteArray): Long
    external fun deserializeProduct(protoBytes: ByteArray): Long
    external fun getAggregatedMetrics(timeRange: String, roles: Array<String>): ByteArray
    external fun exportMetricsToPdf(timeRange: String, roles: Array<String>): ByteArray
}