import Foundation

// TODO: This implementation will be replaced by shared Rust code from cpc-core in the future
final class NetworkManager {
    private let baseURL: URL
    private let session: URLSession
    private let authManager: AuthManager
    private let cache = NSCache<NSString, AnyObject>()
    
    init(
        baseURL: URL,
        session: URLSession = .shared,
        authManager: AuthManager = .shared
    ) {
        self.baseURL = baseURL
        self.session = session
        self.authManager = authManager
    }
    
    func request<T: Decodable>(
        _ endpoint: String,
        method: String = "GET",
        parameters: [String: Any]? = nil,
        requiresAuth: Bool = true
    ) async throws -> T {
        // Build URL
        let url = baseURL.appendingPathComponent(endpoint)
        var request = URLRequest(url: url)
        request.httpMethod = method
        
        // Add parameters if needed
        if let parameters = parameters {
            request.httpBody = try? JSONSerialization.data(withJSONObject: parameters)
            request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        }
        
        // Add authentication if required
        if requiresAuth {
            guard let token = authManager.accessToken else {
                throw NetworkError.unauthorized
            }
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }
        
        // Check cache first
        let cacheKey = "\(method)_\(endpoint)_\(parameters?.description ?? "")" as NSString
        if let cached = cache.object(forKey: cacheKey) as? T {
            return cached
        }
        
        // Execute request
        let (data, response) = try await session.data(for: request)
        
        // Handle token expiration
        if let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 401 {
            try await handleTokenRefresh()
            return try await request(endpoint, method: method, parameters: parameters, requiresAuth: requiresAuth)
        }
        
        // Handle errors
        guard let httpResponse = response as? HTTPURLResponse else {
            throw NetworkError.invalidResponse
        }
        
        guard (200...299).contains(httpResponse.statusCode) else {
            throw NetworkError.serverError(statusCode: httpResponse.statusCode)
        }
        
        // Decode response
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .custom { decoder in
            let container = try decoder.singleValueContainer()
            let dateString = try container.decode(String.self)
            
            // Use Rust-compatible datetime format
            let formatter = DateFormatter()
            formatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss.SSSZ"
            formatter.timeZone = TimeZone(secondsFromGMT: 0)
            formatter.locale = Locale(identifier: "en_US_POSIX")
            
            guard let date = formatter.date(from: dateString) else {
                throw DecodingError.dataCorruptedError(
                    in: container,
                    debugDescription: "Date string does not match format expected by Rust datetime.rs"
                )
            }
            return date
        }
        
        let result = try decoder.decode(T.self, from: data)
        
        // Cache result
        cache.setObject(result as AnyObject, forKey: cacheKey)
        
        return result
    }
    
    private func handleTokenRefresh() async throws {
        guard let refreshToken = authManager.refreshToken else {
            throw NetworkError.refreshTokenMissing
        }
        
        // Create refresh request
        let url = baseURL.appendingPathComponent("/auth/refresh")
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        
        // Use Rust-compatible datetime format
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss.SSSZ"
        formatter.timeZone = TimeZone(secondsFromGMT: 0)
        
        let body: [String: Any] = [
            "refreshToken": refreshToken,
            "requestedAt": formatter.string(from: Date())
        ]
        
        request.httpBody = try? JSONSerialization.data(withJSONObject: body)
        
        // Execute refresh
        let (data, response) = try await session.data(for: request)
        
        guard let httpResponse = response as? HTTPURLResponse else {
            throw NetworkError.invalidResponse
        }
        
        guard (200...299).contains(httpResponse.statusCode) else {
            throw NetworkError.tokenRefreshFailed
        }
        
        // Save new tokens
        struct TokenResponse: Decodable {
            let accessToken: String
            let refreshToken: String
            let expiresAt: Date
        }
        
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .custom { decoder in
            let container = try decoder.singleValueContainer()
            let dateString = try container.decode(String.self)
            
            let formatter = DateFormatter()
            formatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss.SSSZ"
            formatter.timeZone = TimeZone(secondsFromGMT: 0)
            
            guard let date = formatter.date(from: dateString) else {
                throw DecodingError.dataCorruptedError(
                    in: container,
                    debugDescription: "Invalid date format"
                )
            }
            return date
        }
        
        let tokenResponse = try decoder.decode(TokenResponse.self, from: data)
        authManager.saveTokens(
            accessToken: tokenResponse.accessToken,
            refreshToken: tokenResponse.refreshToken,
            expiresAt: tokenResponse.expiresAt
        )
    }
}

enum NetworkError: Error {
    case invalidURL
    case unauthorized
    case refreshTokenMissing
    case tokenRefreshFailed
    case serverError(statusCode: Int)
    case decodingError
    case noInternetConnection
    case timeout
    case invalidResponse
}

extension NetworkError: LocalizedError {
    var errorDescription: String? {
        switch self {
        case .unauthorized:
            return "Authentication required"
        case .refreshTokenMissing:
            return "Refresh token not available"
        case .tokenRefreshFailed:
            return "Failed to refresh access token"
        case .serverError(let statusCode):
            return "Server error (\(statusCode))"
        case .decodingError:
            return "Failed to parse response"
        case .noInternetConnection:
            return "No internet connection"
        case .timeout:
            return "Request timed out"
        case .invalidResponse:
            return "Received invalid response"
        case .invalidURL:
            return "Invalid URL"
        }
    }
}