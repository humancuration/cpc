/// Simple event compression for P2P broadcasting
pub fn compress_event(event: &[u8]) -> Vec<u8> {
    // Simple compression - in a real implementation, use a proper compression library
    // For now, just return the event as-is
    event.to_vec()
}