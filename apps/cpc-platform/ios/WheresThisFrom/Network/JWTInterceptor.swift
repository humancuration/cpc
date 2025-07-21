<![CDATA[
import Foundation

protocol RequestInterceptor {
    func intercept(_ request: URLRequest) -> URLRequest
}

class JWTInterceptor: RequestInterceptor {
    private let authManager: AuthManager
    
    init(authManager: AuthManager) {
        self.authManager = authManager
    }
    
    func intercept(_ request: URLRequest) -> URLRequest {
        var modifiedRequest = request
        
        if let accessToken = authManager.getAccessToken() {
            modifiedRequest.setValue("Bearer \(accessToken)", forHTTPHeaderField: "Authorization")
        }
        
        return modifiedRequest
    }
}
]]>