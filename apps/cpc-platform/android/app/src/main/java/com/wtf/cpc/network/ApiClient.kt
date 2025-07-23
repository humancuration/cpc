package com.wtf.cpc.network

import com.apollographql.apollo3.ApolloClient
import com.apollographql.apollo3.api.ApolloResponse
import com.apollographql.apollo3.exception.ApolloException
import com.wtf.cpc.GetPublicKeyQuery
import android.util.Log

object ApiClient {
    private const val BASE_URL = "https://api.your-backend.com/graphql" // Update with your backend URL
    private val apolloClient = ApolloClient.Builder()
        .serverUrl(BASE_URL)
        .build()

    suspend fun getPublicKey(): String {
        return try {
            val response: ApolloResponse<GetPublicKeyQuery.Data> = apolloClient.query(GetPublicKeyQuery()).execute()
            response.data?.impactPublicKey ?: throw ApiException("Public key not found in response")
        } catch (e: ApolloException) {
            Log.e("ApiClient", "GraphQL error", e)
            throw ApiException("Failed to fetch public key: ${e.message}")
        }
    }
}

class ApiException(message: String) : Exception(message)