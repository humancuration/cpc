# Presence Indicators Integration Implementation Plan

This document provides a comprehensive plan to complete the presence indicators integration for our document editor application. The plan addresses the current gaps in the implementation based on the Presence Integration Guide and the existing codebase.

## 1. Signaling Service Integration

### Objectives:
- Replace mock RedisSignalingService with actual implementation
- Implement robust error handling and reconnection logic
- Add batched PresenceSummary support

### Steps:
1. **Service Replacement**  
   - Import actual RedisSignalingService from `shared_packages` instead of creating a mock
   - Inject Redis connection parameters via environment variables

2. **Error Handling**  
   - Implement exponential backoff strategy for reconnections
   - Add error boundaries to prevent UI crashes
   - Create notification system for connection status changes

3. **Batched Messages**  
   - Implement batched PresenceSummary support:
     ```rust
     // New message handler
     match message {
         SignalingMessage::PresenceSummary(summary) => 
             handle_presence_summary(summary, ...),
         // ...
     }
     ```
   - Add batching threshold (e.g., batch updates every 200ms)
   - Implement delta encoding for presence updates

## 2. Accurate Cursor Positioning

### Objectives:
- Replace fixed character approximations with actual measurements
- Implement document-to-screen coordinate translation
- Add scrolling synchronization

### Steps:
1. **Measurement System**  
   - Create measurement service that tracks:
     - Character width/height
     - Line height
     - Font metrics
   - Implement reactive updates on font/style changes

2. **Coordinate Translation**  
   - Create `PositionTranslator` utility:
     ```rust
     pub struct PositionTranslator {
         line_height: f64,
         char_width: f64,
         scroll_offset: (f64, f64),
     }
     
     impl PositionTranslator {
         pub fn document_to_screen(&self, line: usize, col: usize) -> (f64, f64) {
             // Translation logic
         }
     }
     ```

3. **Scroll Synchronization**  
   - Add scroll position tracking to editor state
   - Broadcast scroll position via signaling service
   - Update cursor positions based on received scroll data

## 3. State Management Improvements

### Objectives:
- Consolidate presence state into unified structure
- Implement LRU caching
- Simplify state updates

### Steps:
1. **Unified State Structure**  
   ```rust
   pub struct UserPresenceState {
       pub user: PresenceUser,
       pub cursor_position: Option<(usize, usize)>,
       pub last_active: DateTime<Utc>,
       pub is_typing: bool,
   }
   ```

2. **LRU Caching**  
   - Use `lru::LruCache` with capacity setting
   - Implement automatic eviction based on:
     - Last activity time
     - Cache size limits

3. **State Management Hook**  
   - Create `use_presence_state` custom hook to manage:
     - Presence data
     - Cursor positions
     - Activity timestamps
     - Typing states

## 4. Typing Detection

### Objectives:
- Implement typing detection indicators
- Add throttling for typing updates
- Extend signaling protocol

### Steps:
1. **Detection Mechanism**  
   - Add keyboard event listeners to editor
   - Set typing state on keydown, clear on idle timeout

2. **Throttling**  
   - Implement 500ms throttling for typing updates
   - Use `gloo_timers` for timing control

3. **Protocol Extension**  
   - Add TypingIndicator handling:
   ```rust
   match message {
       SignalingMessage::TypingIndicator { user_id, is_typing, .. } => {
           // Update typing state
       }
       // ...
   }
   ```

## 5. Performance Optimizations

### Objectives:
- Implement update batching
- Add cursor virtualization
- Apply memoization
- Enforce rate limiting

### Steps:
1. **Update Batching**  
   - Create `PresenceUpdateBatcher` utility
   - Batch updates by:
     - Time window (100ms)
     - Update count threshold (10 updates)

2. **Cursor Virtualization**  
   - Implement visibility detection:
     ```rust
     fn is_visible(position: (usize, usize), viewport: Rect) -> bool {
         // Check if position within viewport
     }
     ```
   - Only render visible cursors

3. **Memoization**  
   - Apply to all presence components:
     ```rust
     let should_render = use_memo(deps, |deps| {
         // Comparison logic
     });
     ```

4. **Rate Limiting**  
   - Enforce 10Hz (100ms) limit for cursor updates
   - Implement token bucket algorithm

## 6. Comprehensive Testing

### Objectives:
- Validate signaling service integration
- Verify presence expiration logic
- Ensure cursor positioning accuracy
- Measure performance benchmarks

### Test Cases:
1. **Signaling Tests**  
   - Connection loss/recovery scenarios
   - Message serialization/deserialization
   - PresenceSummary batching validation

2. **Expiration Tests**  
   - Verify state transitions:
     - Online → Away (5s)
     - Away → Removed (30s)
   - Test clock skew handling

3. **Cursor Accuracy Tests**  
   - Test coordinate translation with:
     - Various font sizes
     - Different zoom levels
     - Mixed content (text, images, tables)

4. **Performance Tests**  
   - Benchmark with 1000 concurrent users
   - Measure:
     - Memory usage
     - CPU utilization
     - Network throughput

### Implementation Timeline:
1. Phase 1 (2 days): Signaling integration & state management
2. Phase 2 (2 days): Cursor positioning & typing detection
3. Phase 3 (1 day): Performance optimizations
4. Phase 4 (1 day): Comprehensive testing

### Dependencies:
- Real RedisSignalingService implementation
- Updated editor measurement hooks
- LRU cache utility