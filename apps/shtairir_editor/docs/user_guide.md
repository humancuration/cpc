# Shtairir Visual Editor User Guide

Welcome to the Shtairir Visual Editor! This guide will help you get started with creating, editing, and managing visual workflows using the Shtairir system.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Interface Overview](#interface-overview)
3. [Creating Workflows](#creating-workflows)
4. [Working with Blocks](#working-with-blocks)
5. [Managing Connections](#managing-connections)
6. [Configuring Nodes](#configuring-nodes)
7. [Workflow Management](#workflow-management)
8. [Advanced Features](#advanced-features)

## Getting Started

### System Requirements

- A modern web browser (Chrome, Firefox, Safari, or Edge)
- Internet connection (for initial load)

### Accessing the Editor

The Shtairir Visual Editor runs directly in your web browser. Simply navigate to the editor URL provided by your CPC platform administrator.

## Interface Overview

The editor interface is divided into several key areas:

1. **Toolbar** - Top-level controls for file operations, editing, and view options
2. **Block Library** - Left sidebar containing available blocks
3. **Canvas** - Central workspace for building workflows
4. **Properties Panel** - Right sidebar showing configuration for selected nodes

### Toolbar

The toolbar contains essential controls:

- **File Menu**: Export/import workflows as TOML
- **Edit Menu**: Undo/redo operations
- **View Menu**: Auto-layout, performance metrics
- **Help Menu**: Documentation and tutorials

### Block Library

The block library shows all available blocks organized by category. You can:

- Search for specific blocks
- Filter by category
- Browse blocks with their documentation

### Canvas

The canvas is where you build your workflows:

- Drag nodes to reposition them
- Click and drag between ports to create connections
- Zoom in/out using the controls or mouse wheel

### Properties Panel

When you select a node, the properties panel shows:

- Node documentation
- Parameter configuration
- Port information

## Creating Workflows

### Starting a New Workflow

1. The editor starts with a blank canvas
2. Browse the block library to find blocks you want to use
3. Drag blocks from the library to the canvas

### Adding Nodes

1. Find a block in the library (use search if needed)
2. Click on the block to add it to the canvas
3. The node will appear at the center of the canvas

### Moving Nodes

1. Click and drag any node to reposition it
2. Nodes can be moved anywhere on the canvas
3. Use the zoom controls to navigate large workflows

## Working with Blocks

### Block Categories

Blocks are organized into categories:

- **Data Processing**: Transform, filter, and manipulate data
- **I/O**: Read from and write to files, databases, networks
- **Math**: Mathematical operations and calculations
- **Logic**: Boolean operations and control flow
- **Visualization**: Charting and graphing capabilities

### Block Information

Each block in the library shows:

- Title and description
- Version information
- Tags for easy identification
- Purity indicator (pure vs effectful)

## Managing Connections

### Creating Connections

1. Click on an output port (right side of a node)
2. Drag to an input port (left side of another node)
3. Release to create the connection

### Connection Validation

The editor automatically validates connections:

- **Green connections**: Valid type compatibility
- **Red connections**: Type mismatch or other issues
- **Dashed lines**: Invalid connections that won't execute

### Connection Policies

Right-click on connections to access advanced options:

- **Adapter Types**: Map, filter, buffer, window operations
- **Backpressure Handling**: Block, drop oldest, drop newest, expand
- **Ordering**: Source order, timestamp, stable key

## Configuring Nodes

### Selecting Nodes

Click on any node to select it. The properties panel will update with:

- Node documentation
- Available parameters
- Port information

### Editing Parameters

Different parameter types have different editors:

- **Text**: Simple text input fields
- **Numbers**: Numeric input with validation
- **Booleans**: Toggle switches
- **Objects/Arrays**: Structured editors

### Node Status

Nodes can display status information:

- **⏳ Pending**: Waiting to execute
- **▶ Running**: Currently executing
- **✓ Completed**: Finished successfully
- **✗ Failed**: Error occurred during execution

## Workflow Management

### Saving Workflows

Workflows are automatically saved in your browser's local storage. To export:

1. Click "Export TOML" in the File menu
2. Save the TOML file to your computer

### Loading Workflows

To load an existing workflow:

1. Click "Import TOML" in the File menu
2. Select your TOML file
3. The workflow will load on the canvas

### Version Control

The editor supports basic versioning:

- Each export includes version information
- Track changes through the undo/redo history
- Compare versions using external tools

## Advanced Features

### Auto-layout

The editor can automatically arrange nodes:

1. Click "Auto-layout" in the View menu
2. Nodes will be arranged in a logical flow
3. Manual adjustments can still be made

### Performance Metrics

Monitor workflow performance:

1. Enable "Performance Metrics" in the View menu
2. See execution time and memory usage
3. Identify bottlenecks in your workflow

### Simulation Mode

Test workflows without full execution:

1. Enable simulation mode from the toolbar
2. Step through node execution manually
3. View intermediate results

### Collaboration

The editor supports collaborative editing:

- Real-time changes from multiple users
- Cursor visibility for other users
- Conflict resolution for simultaneous edits

## Troubleshooting

### Common Issues

**Nodes won't connect**: Check that port types are compatible

**Workflow won't export**: Check for validation errors in the console

**Performance issues**: Try reducing the number of nodes or using auto-layout

### Getting Help

- Check the built-in documentation
- Access tutorials from the Help menu
- Contact your system administrator for platform-specific issues

## Keyboard Shortcuts

- **Ctrl+S**: Export workflow
- **Ctrl+O**: Import workflow
- **Ctrl+Z**: Undo
- **Ctrl+Shift+Z**: Redo
- **Delete**: Remove selected node/connection
- **Space**: Pan canvas
- **Mouse Wheel**: Zoom in/out

## Best Practices

### Workflow Design

1. **Start Simple**: Begin with basic workflows and add complexity gradually
2. **Use Descriptive Names**: Rename nodes to reflect their purpose
3. **Group Related Nodes**: Keep related functionality together
4. **Minimize Cycles**: Avoid circular dependencies when possible

### Performance Optimization

1. **Batch Operations**: Use collection operations instead of individual node processing
2. **Efficient Data Types**: Choose appropriate types for your data
3. **Stream Processing**: Use stream connections for large datasets
4. **Caching**: Reuse computed results when possible

### Documentation

1. **Comment Complex Logic**: Use the notes feature for complex workflows
2. **Version Descriptions**: Update version descriptions with changes
3. **Example Workflows**: Save example workflows for reference

## Conclusion

The Shtairir Visual Editor provides a powerful yet intuitive interface for creating complex workflows without writing code. As you become more familiar with the system, you'll be able to build sophisticated data processing pipelines, machine learning workflows, and other computational graphs.

For advanced features and custom block development, refer to the Shtairir Developer Guide.