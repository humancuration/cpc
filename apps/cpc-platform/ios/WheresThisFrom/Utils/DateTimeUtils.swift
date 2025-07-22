import Foundation

/// Utility for handling datetime in accordance with the standard defined in
/// packages/cpc-core/docs/DATETIME_STANDARD.md
struct DateTimeUtils {
    static let formatter: DateFormatter = {
        let formatter = DateFormatter()
        formatter.dateFormat = "yyyy-MM-dd'T'HH:mm:ss.SSSX"
        formatter.timeZone = TimeZone(secondsFromGMT: 0)
        formatter.locale = Locale(identifier: "en_US_POSIX")
        return formatter
    }()
    
    /// Returns the current datetime in the standard format
    static func currentDateTime() -> String {
        return formatter.string(from: Date())
    }
    
    /// Parses a datetime string in the standard format to a Date object
    static func parseDateTime(_ datetime: String) -> Date? {
        return formatter.date(from: datetime)
    }
}