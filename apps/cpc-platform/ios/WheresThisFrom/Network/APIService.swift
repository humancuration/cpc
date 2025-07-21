<![CDATA[
import Foundation

class APIService {
    func fetchUserProfile() async throws -> UserProfile {
        guard let url = URL(string: "https://api.example.com/user/profile") else {
            throw NetworkError.invalidURL
        }
        
        var request = NetworkManager.shared.createRequest(url: url, method: "GET")
        
        do {
            let (data, response) = try await URLSession.shared.data(for: request)
            
            // Check for 401 Unauthorized
            if let response = response as? HTTPURLResponse, response.statusCode == 401 {
                let refreshSuccess = await NetworkManager.shared.handleUnauthorized(response: response)
                if refreshSuccess {
                    // Retry the request with new token
                    request = NetworkManager.shared.createRequest(url: url, method: "GET")
                    let (newData, _) = try await URLSession.shared.data(for: request)
                    return try JSONDecoder().decode(UserProfile.self, from: newData)
                } else {
                    throw NetworkError.unauthorized
                }
            }
            
            return try JSONDecoder().decode(UserProfile.self, from: data)
        } catch {
            throw NetworkError.requestFailed(error)
        }
    }
}

enum NetworkError: Error {
    case invalidURL
    case unauthorized
    case requestFailed(Error)
}

struct UserProfile: Codable {
    let id: String
    let name: String
    let email: String
}
]]>