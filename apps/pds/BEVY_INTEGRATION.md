# Bevy Engine Integration for CPC Desktop

This document describes the Bevy engine integration for the CPC desktop application, which mirrors the Android BevyView functionality.

## Overview

The Bevy integration provides a 3D rendering layer within the Tauri-based desktop application, similar to the Android BevyView. It includes:

- **Bevy App Initialization**: Rust-based Bevy application setup
- **Tauri Commands**: Communication bridge between JavaScript and Bevy
- **Lifecycle Management**: Pause/resume/stop functionality
- **Window Integration**: Proper handling of window resize and focus events

## Architecture

### Rust Backend (`src-tauri/src/`)
- `bevy_integration.rs`: Core Bevy app initialization and management
- `bevy_commands.rs`: Tauri commands for Bevy control
- Updated `lib.rs`: Integration with existing Tauri state management

### Frontend Components (`frontend/src/`)
- `lib/BevyView`: Main Bevy rendering component
- `components/BevyExperience`: Full-page Bevy experience
- Updated `App`: Navigation integration

## Usage

### Basic Usage
```javascript
import BevyView from '$lib/BevyView';

// In your component
<BevyView width="800" height="600" show={true} />
```

### Tauri Commands
```javascript
// Initialize Bevy
await invoke('initialize_bevy');

// Send message to Bevy
await invoke('send_to_bevy', { message: 'hello' });

// Control Bevy lifecycle
await invoke('control_bevy', { action: 'pause' });
await invoke('control_bevy', { action: 'resume' });
await invoke('control_bevy', { action: 'stop' });

// Check status
const isRunning = await invoke('is_bevy_running');
const status = await invoke('get_bevy_status');
```

## Features

### Current Features
- ✅ 3D cube rendering with rotation
- ✅ Window resize handling
- ✅ Focus/pause management
- ✅ Tauri command integration
- ✅ Basic error handling

### Android Parity
- ✅ Similar lifecycle management (pause/resume/stop)
- ✅ Message passing interface
- ✅ Window integration
- ✅ Navigation toggle support

### Future Enhancements
- Camera controls (WASD movement)
- Touch/mouse interaction
- Custom scene loading
- Performance metrics
- Advanced rendering features

## Development

### Running
```bash
# Terminal 1: Start Tauri dev server
cd apps/pds
npm run tauri dev

# Terminal 2: Start frontend dev server
cd apps/pds/frontend
npm run dev
```

### Testing
- Navigate to the "Bevy" tab in the desktop app
- The 3D cube should appear and rotate automatically
- Use pause/resume/stop buttons to test lifecycle management
- Resize window to test responsive behavior

## Troubleshooting

### Common Issues
1. **Bevy not initializing**: Check console for Rust errors
2. **Black screen**: Ensure proper window initialization
3. **Performance issues**: Adjust Bevy settings in `bevy_integration.rs`

### Debug Commands
```javascript
// Check Bevy status
console.log(await invoke('get_bevy_status'));

// Test message passing
await invoke('send_to_bevy', { message: 'test_message' });
```

## API Reference

### Bevy Commands
- `initialize_bevy()`: Initialize the Bevy application
- `send_to_bevy(message: string)`: Send custom message to Bevy
- `control_bevy(action: string)`: Control lifecycle (pause/resume/stop)
- `is_bevy_running()`: Check if Bevy is running
- `get_bevy_status()`: Get detailed status string

### BevyView Component Props
- `width`: Canvas width (string or number)
- `height`: Canvas height (string or number)
- `show`: Boolean to show/hide the view

## Notes

This implementation follows the Android BevyView pattern while leveraging Tauri's native capabilities for desktop environments. The integration is designed to be modular and can be extended for more complex 3D experiences.