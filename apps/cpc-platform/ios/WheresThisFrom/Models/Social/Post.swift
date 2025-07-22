import Foundation

struct Post: Codable {
    let id: String
    let content: String
    let mediaUrls: [String]
    let author: User
    let timestamp: Date
    let visibility: Visibility
    let cooperativeId: String?
    
    enum CodingKeys: String, CodingKey {
        case id
        case content
        case mediaUrls
        case author
        case timestamp
        case visibility
        case cooperativeId
    }
}

enum Visibility: String, Codable {
    case public = "PUBLIC"
    case cooperative = "COOPERATIVE"
    case private = "PRIVATE"
}