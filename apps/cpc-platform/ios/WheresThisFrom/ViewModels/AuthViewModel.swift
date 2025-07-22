import Foundation
import Combine

class AuthViewModel: ObservableObject {
    @Published var isAuthenticated = false
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var currentUser: User?
    
    private let networkManager: NetworkManager
    private let authManager: AuthManager
    
    init(
        networkManager: NetworkManager = NetworkManager(baseURL: URL(string: "https://api.example.com")!),
        authManager: AuthManager = .shared
    ) {
        self.networkManager = networkManager
        self.authManager = authManager
        self.isAuthenticated = authManager.accessToken != nil
    }
    
    func login(email: String, password: String) async {
        await handleAuthRequest(endpoint: "auth/login", parameters: [
            "email": email,
            "password": password
        ])
    }
    
    func register(email: String, password: String, walletAddress: String) async {
        await handleAuthRequest(endpoint: "auth/register", parameters: [
            "email": email,
            "password": password,
            "walletAddress": walletAddress
        ])
    }
    
    private func handleAuthRequest(endpoint: String, parameters: [String: Any]) async {
        DispatchQueue.main.async {
            self.isLoading = true
            self.errorMessage = nil
        }
        
        do {
            let response: AuthResponse = try await networkManager.request(
                endpoint,
                method: "POST",
                parameters: parameters,
                requiresAuth: false
            )
            
            authManager.saveTokens(
                accessToken: response.accessToken,
                refreshToken: response.refreshToken,
                expiresAt: response.expiresAt
            )
            
            let user: User = try await networkManager.request(
                "users/me",
                method: "GET",
                requiresAuth: true
            )
            
            DispatchQueue.main.async {
                self.currentUser = user
                self.isAuthenticated = true
                self.isLoading = false
            }
        } catch {
            DispatchQueue.main.async {
                self.isLoading = false
                self.errorMessage = self.handleError(error)
                self.isAuthenticated = false
            }
        }
    }
    
    func logout() {
        authManager.clearTokens()
        DispatchQueue.main.async {
            self.isAuthenticated = false
            self.currentUser = nil
        }
    }
    
    func checkAuthStatus() async {
        guard authManager.accessToken != nil else { return }
        
        do {
            let user: User = try await networkManager.request(
                "users/me",
                method: "GET",
                requiresAuth: true
            )
            
            DispatchQueue.main.async {
                self.currentUser = user
                self.isAuthenticated = true
            }
        } catch {
            DispatchQueue.main.async {
                self.isAuthenticated = false
                self.currentUser = nil
            }
        }
    }
    
    private func handleError(_ error: Error) -> String {
        if let networkError = error as? NetworkError {
            return networkError.localizedDescription
        } else {
            return "Authentication failed. Please check your credentials."
        }
    }
}

// MARK: - Data Models
struct AuthResponse: Codable {
    let accessToken: String
    let refreshToken: String
    let expiresAt: Date
}

// MARK: - Mock Data for Previews
extension AuthViewModel {
    static var mock: AuthViewModel {
        let viewModel = AuthViewModel()
        viewModel.currentUser = UserProfileViewModel.mock.user
        viewModel.isAuthenticated = true
        return viewModel
    }
}