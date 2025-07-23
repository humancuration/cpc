# Android Development Guide: Social Features Implementation

## Introduction
This guide covers the implementation of social features in the Android app using the MVVM pattern, GraphQL API, and shared Rust logic. The social features include timeline viewing, post creation, and user interactions.

## Implementing Social Features

### MVVM Pattern Implementation
The Model-View-ViewModel pattern is used to separate concerns and maintain clean architecture:

**ViewModel Example: TimelineViewModel**
```kotlin
class TimelineViewModel : ViewModel() {
    private val repository = SocialRepository()
    
    private val _posts = MutableStateFlow<List<Post>>(emptyList())
    val posts: StateFlow<List<Post>> = _posts.asStateFlow()
    
    fun loadTimeline() {
        viewModelScope.launch {
            _posts.value = repository.getTimeline()
        }
    }
}
```

**Best Practices:**
- Use StateFlow for observable state
- Keep business logic in ViewModels
- Use coroutines for asynchronous operations
- Handle errors in ViewModels

### GraphQL Client Integration
The Apollo Android client handles GraphQL operations:

**Client Configuration:**
```kotlin
object SocialGraphQLClient {
    private const val BASE_URL = "http://localhost:3000/graphql"
    
    val apolloClient: ApolloClient = ApolloClient.Builder()
        .serverUrl(BASE_URL)
        .okHttpClient(okHttpClient)
        .build()
}
```

**Query Execution:**
```kotlin
val response = SocialGraphQLClient.apolloClient
    .query(TimelineQuery(userId = currentUserId))
    .execute()
```

### SocialRepository Pattern
The repository abstracts data sources:

**Key Responsibilities:**
- Execute GraphQL queries/mutations
- Handle local caching (future)
- Transform data between layers

**Example Method:**
```kotlin
suspend fun getTimeline(limit: Int = 20, offset: Int = 0): List<Post> {
    val response = SocialGraphQLClient.apolloClient
        .query(TimelineQuery(currentUserId, limit, offset))
        .execute()
    
    return response.data?.timeline?.map { it.toDomainModel() } ?: emptyList()
}
```

### UI Implementation with Jetpack Compose
Jetpack Compose provides reactive UI components:

**Timeline UI Example:**
```kotlin
@Composable
fun TimelineScreen() {
    val viewModel: TimelineViewModel = viewModel()
    val posts by viewModel.posts.collectAsState()
    
    LazyColumn {
        items(posts) { post ->
            PostCard(post)
        }
    }
}

@Composable
fun PostCard(post: Post) {
    Card {
        Column {
            Text(post.content)
            Text("By: ${post.author.username}")
            // Media items display
        }
    }
}
```

**Best Practices:**
- Use state hoisting for reusable components
- Keep composables small and focused
- Use Material3 theming
- Handle loading/error states gracefully

### Troubleshooting Common Issues

**1. Network Request Failures:**
- Check BASE_URL configuration
- Verify network permissions in manifest
- Inspect Apollo client logs

**2. Data Deserialization Errors:**
- Ensure Kotlin data classes match GraphQL schema
- Use default values for nullable fields
- Add @Serializable annotations

**3. UI State Inconsistencies:**
- Verify StateFlow collection in composables
- Check ViewModel initialization
- Ensure proper coroutine scoping

## Cross-Platform Considerations

### Shared Models with cpc-core
The Android app uses Rust models from `cpc-core` via FFI:

**Usage Pattern:**
```kotlin
external fun nativeValidatePostContent(content: String): Boolean

fun createPost(content: String) {
    if (nativeValidatePostContent(content)) {
        // Proceed with creation
    }
}
```

### Networking with cpc-net
Common networking logic is shared via:
- GraphQL schema definitions
- API contracts
- Error handling patterns

### GraphQL API Contracts
The backend and mobile apps share:
- Type definitions (Post, User, etc.)
- Query/mutation signatures
- Pagination patterns

## Next Steps
1. Implement local caching with Room DB
2. Add pagination support to timeline
3. Integrate Rust FFI for content validation
4. Implement user following functionality