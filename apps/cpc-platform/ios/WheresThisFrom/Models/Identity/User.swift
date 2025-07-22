import Foundation

struct User: Codable {
    let id: String
    let walletAddress: String
    let email: String?
    let name: String?
    let avatarUrl: String?
    let createdAt: Date
    let updatedAt: Date
    let lastLogin: Date?
    let role: String
    let cooperativeScore: CooperativeScore
    
    enum CodingKeys: String, CodingKey {
        case id
        case walletAddress
        case email
        case name
        case avatarUrl
        case createdAt
        case updatedAt
        case lastLogin
        case role
        case cooperativeScore
    }
}

struct CooperativeScore: Codable {
    // TODO: Implement cooperative score properties
}