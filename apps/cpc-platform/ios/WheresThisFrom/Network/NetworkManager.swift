<![CDATA[
import Foundation

class NetworkManager {
    static let shared = NetworkManager()
    private let interceptor: RequestInterceptor
    
    private init() {
        self.interceptor = JWTInterceptor(authManager: AuthManagerImpl.shared)
    }
    
    func createRequest(url: URL, method: String, body: Data? = nil) -> URLRequest {
        var request = URLRequest(url: url)
        request.httpMethod = method
        request.httpBody = body
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        
        // Apply JWT interceptor
        return interceptor.intercept(request)
    }
    
    func handleUnauthorized(response: URLResponse) async -> Bool {
        guard let httpResponse = response as? HTTPURLResponse,
              httpResponse.statusCode == 401 else {
            return false
        }
        
        return await AuthManagerImpl.shared.refreshToken()
    }
}
]]>