import Foundation
import Combine

class FeedViewModel: ObservableObject {
    @Published var posts: [Post] = []
    @Published var stories: [Story] = []
    @Published var isLoading = false
    @Published var errorMessage: String?
    
    private let networkManager: NetworkManager
    
    init(networkManager: NetworkManager = NetworkManager(baseURL: URL(string: "https://api.example.com")!)) {
        self.networkManager = networkManager
    }
    
    func fetchFeed() async {
        DispatchQueue.main.async {
            self.isLoading = true
            self.errorMessage = nil
        }
        
        do {
            // Fetch posts
            let postsResponse: FeedResponse<Post> = try await networkManager.request(
                "feed/posts",
                method: "GET",
                requiresAuth: true
            )
            
            // Fetch stories
            let storiesResponse: FeedResponse<Story> = try await networkManager.request(
                "feed/stories",
                method: "GET",
                requiresAuth: true
            )
            
            DispatchQueue.main.async {
                self.posts = postsResponse.items
                self.stories = storiesResponse.items
                self.isLoading = false
            }
        } catch {
            DispatchQueue.main.async {
                self.isLoading = false
                self.errorMessage = self.handleError(error)
            }
        }
    }
    
    private func handleError(_ error: Error) -> String {
        if let networkError = error as? NetworkError {
            return networkError.localizedDescription
        } else {
            return "Failed to load feed. Please try again later."
        }
    }
}

// MARK: - Data Models
struct Post: Identifiable, Codable {
    let id: String
    let userId: String
    let content: String
    let mediaUrl: String?
    let likes: Int
    let comments: Int
    let createdAt: Date
}

struct Story: Identifiable, Codable {
    let id: String
    let userId: String
    let mediaUrl: String
    let createdAt: Date
    let expiresAt: Date
}

struct FeedResponse<T: Codable>: Codable {
    let items: [T]
}

// MARK: - Mock Data for Previews
extension FeedViewModel {
    static var mock: FeedViewModel {
        let viewModel = FeedViewModel()
        viewModel.posts = [
            Post(
                id: "post1",
                userId: "user123",
                content: "Check out this amazing artwork I created!",
                mediaUrl: "https://example.com/artwork.jpg",
                likes: 42,
                comments: 5,
                createdAt: Date()
            ),
            Post(
                id: "post2",
                userId: "user456",
                content: "Just joined the cooperative! Excited to contribute",
                mediaUrl: nil,
                likes: 18,
                comments: 2,
                createdAt: Date().addingTimeInterval(-3600)
            )
        ]
        
        viewModel.stories = [
            Story(
                id: "story1",
                userId: "user123",
                mediaUrl: "https://example.com/story1.jpg",
                createdAt: Date(),
                expiresAt: Date().addingTimeInterval(86400)
            ),
            Story(
                id: "story2",
                userId: "user789",
                mediaUrl: "https://example.com/story2.mp4",
                createdAt: Date().addingTimeInterval(-1800),
                expiresAt: Date().addingTimeInterval(84600)
            )
        ]
        return viewModel
    }
}