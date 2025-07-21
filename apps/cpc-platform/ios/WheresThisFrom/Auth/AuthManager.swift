<![CDATA[
import Foundation
import Security

protocol AuthManager {
    func getAccessToken() -> String?
    func getRefreshToken() -> String?
    func saveTokens(accessToken: String, refreshToken: String)
    func refreshToken() async -> Bool
}

class AuthManagerImpl: AuthManager {
    static let shared = AuthManagerImpl()
    private let serviceName = "com.wtf.auth"
    
    private init() {}
    
    func getAccessToken() -> String? {
        return getToken(forKey: "access_token")
    }
    
    func getRefreshToken() -> String? {
        return getToken(forKey: "refresh_token")
    }
    
    func saveTokens(accessToken: String, refreshToken: String) {
        saveToken(accessToken, forKey: "access_token")
        saveToken(refreshToken, forKey: "refresh_token")
    }
    
    func refreshToken() async -> Bool {
        guard let refreshToken = getRefreshToken() else { return false }
        
        do {
            // Call refresh token endpoint
            // let newTokens = try await apiService.refreshToken(refreshToken)
            // saveTokens(accessToken: newTokens.accessToken, refreshToken: newTokens.refreshToken)
            return true
        } catch {
            return false
        }
    }
    
    private func saveToken(_ token: String, forKey key: String) {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: serviceName,
            kSecAttrAccount as String: key,
            kSecValueData as String: token.data(using: .utf8)!
        ]
        
        SecItemDelete(query as CFDictionary)
        SecItemAdd(query as CFDictionary, nil)
    }
    
    private func getToken(forKey key: String) -> String? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: serviceName,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        
        var dataTypeRef: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &dataTypeRef)
        
        guard status == errSecSuccess, 
              let data = dataTypeRef as? Data,
              let token = String(data: data, encoding: .utf8) else {
            return nil
        }
        
        return token
    }
}
]]>