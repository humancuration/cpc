@Module
@InstallIn(SingletonComponent::class)
object NetworkModule {
    private const val BASE_URL = "http://localhost:8080/"

    @Provides
    @Singleton
    fun provideBackendService(): BackendService {
        return Retrofit.Builder()
            .baseUrl(BASE_URL)
            .addConverterFactory(MessagePackConverterFactory.create())
            .addConverterFactory(MoshiConverterFactory.create())
            .build()
            .create(BackendService::class.java)
    }

    @Provides
    @Singleton
    fun provideAuthManager(@ApplicationContext context: Context): AuthManager {
        return AuthManager(context.dataStore)
    }
}