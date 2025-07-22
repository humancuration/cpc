import Foundation
import Combine

class UserProfileViewModel: ObservableObject {
    @Published var user: User?
    @Published var isLoading = false
    @Published var errorMessage: String?
    
    private let networkManager: NetworkManager
    
    init(networkManager: NetworkManager = NetworkManager(baseURL: URL(string: "https://api.example.com")!)) {
        self.networkManager = networkManager
    }
    
    func fetchUserProfile(userId: String) async {
        DispatchQueue.main.async {
            self.isLoading = true
            self.errorMessage = nil
        }
        
        do {
            let user: User = try await networkManager.request(
                "users/\(userId)",
                method: "GET",
                requiresAuth: true
            )
            
            DispatchQueue.main.async {
                self.user = user
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
            return "Failed to load profile. Please try again later."
        }
    }
}

// MARK: - Mock Data for Previews
extension UserProfileViewModel {
    static var mock: UserProfileViewModel {
        let viewModel = UserProfileViewModel()
        viewModel.user = User(
            id: "user123",
            walletAddress: "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
            email: "user@example.com",
            name: "John Doe",
            avatarUrl: "https://example.com/avatar.jpg",
            createdAt: Date(),
            updatedAt: Date(),
            lastLogin: Date(),
            role: "member",
            cooperativeScore: CooperativeScore()
        )
        return viewModel
    }
}