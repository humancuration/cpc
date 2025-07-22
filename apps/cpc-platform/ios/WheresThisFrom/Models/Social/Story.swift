import Foundation

struct Story: Codable {
    let post: Post
    let expirationTimestamp: Date
    let metadata: [String: String]
    let viewedBy: [String]
    
    enum CodingKeys: String, CodingKey {
        case post
        case expirationTimestamp
        case metadata
        case viewedBy
    }
}