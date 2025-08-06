# Shtairir Visual Editor - Enhanced Web Example

This enhanced web example demonstrates all the features of the Shtairir Visual Editor, providing a comprehensive demo and learning resource for developers.

## Features Demonstrated

### 1. Diverse Node Examples

The example includes multiple types of nodes showcasing different functionality:

#### Math Operations Node
- **Add Node**: Performs addition operations with two number inputs
- **Multiply Node**: Performs multiplication operations
- **Port Types**: All ports use `Number` type for type-safe mathematical operations
- **Parameters**: Editable number inputs with validation

#### String Manipulation Node
- **Concatenate Node**: Combines two strings into one
- **Uppercase Node**: Converts text to uppercase
- **Port Types**: All ports use `String` type for text operations
- **Parameters**: Editable string inputs

#### Boolean Logic Node
- **AND Node**: Logical AND operation with two boolean inputs
- **OR Node**: Logical OR operation
- **Port Types**: All ports use `Boolean` type for logic operations
- **Parameters**: Toggle switches for boolean values

#### Custom Node with Object/Array Ports
- **Object Processing**: Handles complex data structures with key-value pairs
- **Array Processing**: Manages lists of values
- **Mixed Port Types**: Combines Number, String, Object, and Array ports
- **Complex Parameters**: Object and array editors with add/remove functionality

### 2. Connection Validation

The example demonstrates real-time connection validation with visual feedback:

#### Valid Connections
- **Type Matching**: Connections between compatible port types (Number → Number, String → String)
- **Visual Indicators**: Valid connections shown in blue
- **Direction Validation**: Only allows output → input connections
- **Real-time Feedback**: Immediate visual response when creating connections

#### Invalid Connections
- **Type Mismatch**: Connections between incompatible types (Number → String)
- **Visual Indicators**: Invalid connections shown in red with dashed lines
- **Prevention**: System prevents invalid connections from being created
- **Error Demo**: "Add Invalid Connection" button creates a Number → String connection for demonstration

#### Port Type Compatibility
- **Any Type**: Can connect to any port type
- **Specific Types**: Only connect to identical types
- **Custom Types**: Only connect to matching custom types
- **Direction Rules**: Output ports can only connect to input ports

### 3. Parameter Editing

The enhanced NodeEditor provides controls for different value types:

#### Number Input
- **Validation**: Ensures only valid numeric values are accepted
- **Step Control**: Supports decimal values with configurable step size
- **Real-time Updates**: Changes immediately reflected in the node
- **Visual Feedback**: Input styling for focused/error states

#### Text Input Field
- **Multi-line Support**: Handles long text values
- **Character Encoding**: Full Unicode support
- **Placeholder Text**: Helpful hints for expected input
- **Real-time Preview**: Live updates as you type

#### Boolean Toggle
- **Visual Switch**: Animated toggle slider
- **Clear Labels**: "True"/"False" text indicators
- **Accessibility**: Keyboard navigation support
- **Immediate Feedback**: Instant state changes

#### Array Editor
- **Dynamic Lists**: Add/remove array elements
- **Type Validation**: Ensures array elements match expected types
- **Visual Management**: Clear UI for array manipulation
- **Number Arrays**: Specialized for numeric array elements

#### Object Editor
- **Key-Value Pairs**: Add/remove object properties
- **Type Flexibility**: Supports nested numbers and strings
- **Visual Structure**: Clear representation of object hierarchy
- **Real-time Updates**: Changes immediately applied

### 4. Script Conversion

The example demonstrates bidirectional script conversion:

#### Graph to Script
- **Algorithm**: Traverses graph in execution order
- **Identifier Resolution**: Converts connections to identifier references
- **Command Generation**: Creates proper Shtairir commands
- **Formatting**: Clean, readable script output

#### Script to Graph
- **Parsing**: Converts Shtairir script back to visual graph
- **Node Creation**: Generates nodes for each command
- **Default Layout**: Automatic positioning of nodes
- **Port Generation**: Creates appropriate input/output ports

#### Legacy Migration
- **Compatibility**: Updates old graph formats to current version
- **Port Addition**: Adds missing input/output ports to legacy nodes
- **Data Preservation**: Maintains existing node data and connections
- **Error Handling**: Graceful handling of migration issues

### 5. UI Components

#### VisualEditor Component
- **SVG Connections**: Smooth Bezier curves for connection lines
- **Interactive Nodes**: Draggable nodes with visual feedback
- **Port Highlighting**: Visual indication of port types
- **Responsive Design**: Adapts to different screen sizes

#### NodeEditor Component
- **Type-specific Controls**: Different UI for different value types
- **Real-time Updates**: Immediate synchronization with graph
- **Validation**: Input validation for all parameter types
- **Accessibility**: Keyboard navigation and screen reader support

#### Styling System
- **CSS Variables**: Theming support for consistent colors
- **Port Type Colors**: Visual distinction between different port types
- **Responsive Layout**: Works on desktop and mobile devices
- **Hover Effects**: Interactive feedback for all UI elements

## How to Use

### Adding Nodes
1. Click the appropriate button in the "Add Nodes" toolbar section
2. Nodes are automatically positioned to avoid overlap
3. Each node type has appropriate default parameters

### Creating Connections
1. Click on an output port (right side of node, green indicator)
2. Click on an input port (left side of node, blue indicator)
3. Valid connections are shown in blue, invalid in red
4. Use "Add Invalid Connection" to see error handling

### Editing Parameters
1. Click on any node to select it
2. The NodeEditor panel will appear on the right
3. Edit parameters using the appropriate control type
4. Changes are immediately applied to the node

### Script Conversion
1. **Graph to Script**: Click "Graph to Script" to see the generated Shtairir script
2. **Script to Graph**: Click "Script to Graph" to create a graph from sample script
3. **Legacy Migration**: Click "Migrate Legacy" to update old graph formats

### Visual Feedback
- **Valid Connections**: Blue solid lines
- **Invalid Connections**: Red dashed lines
- **Port Types**: Color-coded indicators (blue=Number, purple=String, orange=Boolean, etc.)
- **Node Selection**: Highlighted with shadow effect
- **Hover Effects**: Interactive feedback on all clickable elements

## Code Structure

### Main Components

#### `web_example.rs`
- Main application entry point
- Demo graph creation with diverse node types
- Event handlers for all user interactions
- State management for graph, selected nodes, and script output

#### `lib.rs`
- Core VisualEditor component
- Enhanced connection rendering with validation
- Node positioning and interaction
- SVG-based connection visualization

#### `node_editor.rs`
- Enhanced parameter editing for all value types
- Specialized editors for arrays and objects
- Type-specific input validation
- Real-time synchronization with graph state

#### `port.rs`
- Port type definitions and compatibility checking
- Direction validation (input/output)
- Custom port type support

### Helper Functions

#### Node Creation Functions
- `create_math_node()`: Creates mathematical operation nodes
- `create_string_node()`: Creates string manipulation nodes
- `create_boolean_node()`: Creates logic operation nodes
- `create_custom_node()`: Creates complex data processing nodes

#### Utility Functions
- `create_demo_graph()`: Sets up the initial demonstration graph
- `format_script()`: Converts script AST to readable text
- `get_port_position()`: Calculates port positions for connections

## Best Practices Demonstrated

### 1. Type Safety
- Strict port type compatibility checking
- Input validation for all parameter types
- Clear visual indication of type mismatches

### 2. User Experience
- Intuitive drag-and-drop interface
- Real-time visual feedback
- Responsive design for different screen sizes
- Accessible UI with keyboard navigation

### 3. Code Organization
- Modular component architecture
- Clear separation of concerns
- Reusable helper functions
- Comprehensive error handling

### 4. Performance
- Efficient graph traversal algorithms
- Optimized SVG rendering
- Minimal state updates
- Lazy loading of components

## Extending the Example

### Adding New Node Types
1. Create a new node creation function similar to existing ones
2. Define appropriate input/output ports
3. Add button to toolbar for new node type
4. Update the demo graph if needed

### Customizing Port Types
1. Add new PortType variants in `port.rs`
2. Update compatibility checking logic
3. Add CSS styling for new port types
4. Update legend and documentation

### Enhancing Parameter Editors
1. Create new editor components for specialized types
2. Add validation logic for new types
3. Update the NodeEditor to use new components
4. Add styling for new editor types

## Running the Example

```bash
cd shared_packages/shtairir_editor
cargo run --bin web_example
```

The example will be available in your web browser at the default Yew development server address.

## Dependencies

- Yew framework for web UI
- Serde for serialization
- Shtairir core for AST and script handling
- Web APIs for browser integration

## Learning Resources

This example serves as a comprehensive learning resource for:

1. **Visual Programming**: Understanding node-based visual programming concepts
2. **Type Systems**: Learning about type compatibility and validation
3. **UI Development**: Best practices for complex web interfaces
4. **State Management**: Handling complex application state in Yew
5. **Script Conversion**: Understanding bidirectional conversion between visual and textual representations

The code is extensively commented and follows Rust best practices, making it an excellent reference for developers learning about visual programming interfaces and the Shtairir ecosystem.