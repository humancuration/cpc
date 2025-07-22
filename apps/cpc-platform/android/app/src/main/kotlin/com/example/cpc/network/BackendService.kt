interface BackendService {
    @POST("/api/update/check")
    suspend fun checkForUpdates(@Body request: UpdateCheckRequest): UpdateCheckResponse

    @POST("/publish")
    @Headers("Content-Type: application/msgpack") 
    suspend fun publishProject(
        @Header("Authorization") token: String,
        @Body projectData: ByteArray
    ): PublishResponse

    @GET("/auth/refresh")
    suspend fun refreshToken(@Header("Authorization") refreshToken: String): AuthResponse
}

data class UpdateCheckRequest(val currentVersion: String)
data class UpdateCheckResponse(val updateAvailable: Boolean, val latestVersion: String)
data class PublishResponse(val success: Boolean, val projectId: String)
data class AuthResponse(val accessToken: String, val refreshToken: String)