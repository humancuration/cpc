# Flood Fill Algorithm Design for Magic Wand Tool

## Requirements
1. Handle both contiguous and non-contiguous selection modes
2. Support adjustable tolerance levels (0.0-1.0)
3. Optimized performance using scanline algorithm
4. Integrate with existing SelectionService and undo/redo system
5. Work within Bevy ECS architecture
6. Maintain UUID-based selection tracking
7. Support both bitmap and vector layers (future-proof)

## Algorithm Selection
We'll implement a scanline-based flood fill algorithm for efficiency:

```pseudocode
function floodFill(start_x, start_y, tolerance, contiguous):
    target_color = get_pixel(start_x, start_y)
    queue = new Queue()
    queue.enqueue((start_x, start_y))
    visited = new 2D boolean array[width][height]
    
    while queue not empty:
        (x, y) = queue.dequeue()
        
        // Process current scanline
        left_x = find_left_bound(x, y, target_color, tolerance)
        right_x = find_right_bound(x, y, target_color, tolerance)
        
        for current_x from left_x to right_x:
            mark_pixel(current_x, y)
            
            // Check neighbors above and below
            if contiguous:
                check_neighbor(current_x, y-1) // above
                check_neighbor(current_x, y+1) // below

function check_neighbor(x, y):
    if pixel within bounds and not visited and color_match(target_color, pixel_color, tolerance):
        queue.enqueue((x, y))
        visited[x][y] = true
```

## Data Structures
1. **Selection Mask**: 
   - `Vec<u8>` where 0=unselected, 255=fully selected
   - Pre-allocated to layer dimensions

2. **Processing Queue**:
   - `VecDeque<(u32, u32)>` for pixel coordinates
   - More efficient than recursion for large areas

3. **Visited Matrix**:
   - `Vec<bool>` 1D array flattened to 2D coordinates
   - Optimized memory layout: index = y * width + x

## Tolerance Calculation
```pseudocode
function color_match(target: [u8;4], candidate: [u8;4], tolerance: f32) -> bool:
    // Convert to f32 in [0,1] range
    t_r = target[0] as f32 / 255.0
    t_g = target[1] as f32 / 255.0
    t_b = target[2] as f32 / 255.0
    
    c_r = candidate[0] as f32 / 255.0
    c_g = candidate[1] as f32 / 255.0
    c_b = candidate[2] as f32 / 255.0

    // Calculate Euclidean distance in normalized RGB space
    distance = sqrt((t_r - c_r)^2 + (t_g - c_g)^2 + (t_b - c_b)^2)
    
    return distance <= tolerance
```

## Performance Optimizations
1. **Scanline Algorithm**: Processes horizontal runs of pixels
2. **Bit Packing**: Store visited flags in bits (8 flags/byte)
3. **Thread Pool**: Use Rayon for parallel processing of large images
4. **Early Exit**: Skip fully opaque/transparent areas
5. **SIMD Acceleration**: Use packed_simd for color comparisons

## Integration Points
### Modified SelectionArea Creation
```rust
impl SelectionArea {
    pub fn new_magic_wand(
        layer: &Layer,
        start_x: u32,
        start_y: u32,
        tolerance: f32,
        contiguous: bool,
        layer_id: Option<Uuid>,
    ) -> Self {
        // Implement flood fill here
    }
}
```

### Updated SelectionService
```rust
impl SelectionService {
    pub fn create_magic_wand_selection(
        project: &mut Project,
        layer_id: Uuid,
        x: u32,
        y: u32,
        tolerance: f32,
        contiguous: bool,
    ) -> Result<(), String> {
        // Pass contiguous parameter
    }
}
```

### MagicWandTool Enhancement
```rust
impl MagicWandTool {
    // Add this method to support mode switching
    pub fn set_contiguous(&mut self, contiguous: bool) {
        self.contiguous = contiguous;
    }
}
```

## Undo/Redo Handling
- Continue using existing `Action::SelectionCreated` variant
- Selection ID generated before processing for immediate undo support

## Future Improvements
1. **GPU Acceleration**: Implement WebGL shader for browser version
2. **Multi-threaded Processing**: Use Rayon for parallel flood fill
3. **Color Space Support**: Add HSL/HSV comparison modes
4. **Soft Selection**: Support partial selection (anti-aliased edges)
5. **Layer Compositing**: Consider visibility of lower layers

## Testing Plan
1. Unit tests for:
   - Color matching tolerance
   - Contiguous vs non-contiguous results
   - Edge case handling (image boundaries)
   - Performance benchmarks
2. Integration tests with SelectionService
3. Visual regression tests