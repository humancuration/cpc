package coop.cpc.utils

import android.content.Context
import com.apollographql.apollo3.ApolloClient
import okhttp3.OkHttpClient

object GraphQLClient {
    private const val BASE_URL = "http://10.0.2.2:4000/graphql" // For Android emulator

    fun getApolloClient(context: Context): ApolloClient {
        val okHttpClient = OkHttpClient.Builder()
            .build()

        return ApolloClient.Builder()
            .serverUrl(BASE_URL)
            .okHttpClient(okHttpClient)
            .build()
    }
}