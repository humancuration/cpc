<![CDATA[
import Foundation

class NetworkManager {
    static let shared = NetworkManager()
    private let interceptor: RequestInterceptor
    
    private init() {
        self.interceptor = JWTInterceptor(authManager: AuthManagerImpl.shared)
    }
    
    private let jsonDecoder: JSONDecoder = {
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .custom { decoder -> Date in
            let container = try decoder.singleValueContainer()
            let dateString = try container.decode(String.self)
            guard let date = DateTimeUtils.parseDateTime(dateString) else {
                throw DecodingError.dataCorruptedError(
                    in: container,
                    debugDescription: "Date string does not match format expected by formatter."
                )
            }
            return date
        }
        return decoder
    }()
    
    private let jsonEncoder: JSONEncoder = {
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .custom { date, encoder in
            var container = encoder.singleValueContainer()
            let dateString = DateTimeUtils.formatter.string(from: date)
            try container.encode(dateString)
        }
        return encoder
    }()
    
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
    
    func decode<T: Decodable>(_ type: T.Type, from data: Data) throws -> T {
        return try jsonDecoder.decode(type, from: data)
    }
    
    func encode<T: Encodable>(_ value: T) throws -> Data {
        return try jsonEncoder.encode(value)
    }
}
]]>