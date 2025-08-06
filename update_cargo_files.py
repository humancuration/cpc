#!/usr/bin/env python3
"""
Script to update Cargo.toml files to use workspace dependencies
"""

import os
import re
from pathlib import Path

# Workspace dependencies from main Cargo.toml
WORKSPACE_DEPS = {
    # Core Technologies
    "bevy": "0.16.1",
    "p2panda": "0.4.0",
    "p2panda-auth": "0.4.0",
    "p2panda-blobs": "0.4.0",
    "p2panda-core": "0.4.0",
    "p2panda-discovery": "0.4.0",
    "p2panda-encryption": "0.4.0",
    "p2panda-net": "0.4.0",
    "p2panda-store": "0.4.0",
    "p2panda-stream": "0.4.0",
    "p2panda-sync": "0.4.0",
    
    # UI and Desktop Framework
    "tauri": "2.7.0",
    "yew": "0.21.0",
    "yew-router": "0.18.0",
    "stylist": "0.13.0",
    "wry": "0.52.1",
    
    # Graphics and Rendering
    "glow": "0.16.0",
    "ash": "0.38.0",
    
    # Web Assembly
    "wasm-bindgen": "0.2.100",
    "wasm-bindgen-futures": "0.4.50",
    "web-sys": "0.3.77",
    "js-sys": "0.3.77",
    "gloo-timers": "0.3.0",
    
    # Data Visualization
    "plotters": "0.3.7",
    
    # Web Server Framework
    "axum": "0.8.4",
    
    # Database
    "sqlx": "0.8.6",
    "sqlx-cli": "0.8.6",
    "tokio-postgres": "0.7.13",
    "diesel": "2.2.12",
    "sled": "0.34.7",
    
    # Redis and Connection Pooling
    "redis": "0.32.4",
    "bb8": "0.9.0",
    "bb8-redis": "0.24.0",
    
    # Logging
    "tracing": "0.1.41",
    "tracing-subscriber": "0.3.18",
    
    # Media Processing
    "rodio": "0.21",
    "pdf": "0.9.0",
    "gstreamer": "0.24.0",
    
    # GraphQL for Public API
    "async-graphql": "7.0.17",
    "async-graphql-axum": "7.0.17",
    "graphql-parser": "0.4.1",
    "graphql_client": "0.14.0",
    "graphql_query_derive": "0.14.0",
    "graphql_client_codegen": "0.14.0",
    
    # gRPC for Internal API
    "tonic": "0.14.0",
    "prost": "0.14.1",
    "prost-types": "0.14.1",
    
    # Parser Generation
    "pest": "2.8.1",
    "pest_derive": "2.8.1",
    "pest_meta": "2.8.1",
    "pest_generator": "2.8.1",
    
    # Common dependencies
    "tokio": "1.47.0",
    "serde": "1.0.219",
    "serde_json": "1.0.141",
    "chrono": "0.4.41",
    "uuid": "1.17.0",
    "anyhow": "1.0.98",
    "thiserror": "2.0.12",
    "async-trait": "0.1.88",
    "rust_decimal": "1.37.2",
    "rust_decimal_macros": "1.37.2",
    "async-stream": "0.3.6",
    "futures-util": "0.3.31",
    "log": "0.4.27",
    "wasm-logger": "0.2.0",
    "rand": "0.8.5",
    
    # WebSocket support
    "tokio-tungstenite": "0.20.1",
    
    # Authentication and Security
    "oauth2": "5.0.0",
    "jsonwebtoken": "9.3.1",
    "argon2": "0.5.3",
    "aes-gcm": "0.10.3",
    "chacha20poly1305": "0.10.1",
    "rustls": "0.23.31",
    "ed25519-dalek": "2.2.0",
}

def update_cargo_toml(file_path):
    """Update a single Cargo.toml file to use workspace dependencies"""
    print(f"Updating {file_path}")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    lines = content.split('\n')
    updated_lines = []
    in_dependencies = False
    
    for line in lines:
        # Check if we're in a dependencies section
        if line.strip().startswith('[dependencies]') or line.strip().startswith('[dev-dependencies]') or line.strip().startswith('[build-dependencies]'):
            in_dependencies = True
            updated_lines.append(line)
            continue
        elif line.strip().startswith('[') and in_dependencies:
            in_dependencies = False
            updated_lines.append(line)
            continue
        
        if in_dependencies and '=' in line and not line.strip().startswith('#'):
            # Parse dependency line
            dep_match = re.match(r'^(\s*)([a-zA-Z0-9_-]+)\s*=\s*(.+)$', line)
            if dep_match:
                indent, dep_name, dep_value = dep_match.groups()
                
                # Check if this dependency is in our workspace
                if dep_name in WORKSPACE_DEPS:
                    # Check if it's already using workspace = true
                    if 'workspace = true' not in dep_value:
                        # Parse the existing dependency to preserve features and other options
                        if dep_value.strip().startswith('{'):
                            # Complex dependency specification
                            # Try to preserve features while using workspace
                            features_match = re.search(r'features\s*=\s*(\[[^\]]*\])', dep_value)
                            optional_match = re.search(r'optional\s*=\s*(true|false)', dep_value)
                            
                            new_dep = '{ workspace = true'
                            if features_match:
                                new_dep += f', features = {features_match.group(1)}'
                            if optional_match:
                                new_dep += f', optional = {optional_match.group(1)}'
                            new_dep += ' }'
                            
                            updated_lines.append(f'{indent}{dep_name} = {new_dep}')
                        else:
                            # Simple version specification
                            updated_lines.append(f'{indent}{dep_name} = {{ workspace = true }}')
                    else:
                        updated_lines.append(line)
                else:
                    updated_lines.append(line)
            else:
                updated_lines.append(line)
        else:
            updated_lines.append(line)
    
    # Write back the updated content
    with open(file_path, 'w') as f:
        f.write('\n'.join(updated_lines))

def main():
    """Find and update all Cargo.toml files"""
    root_dir = Path('.')
    
    # Find all Cargo.toml files except the root one
    cargo_files = []
    for cargo_file in root_dir.rglob('Cargo.toml'):
        if cargo_file != Path('./Cargo.toml'):  # Skip root Cargo.toml
            cargo_files.append(cargo_file)
    
    print(f"Found {len(cargo_files)} Cargo.toml files to update")
    
    for cargo_file in cargo_files:
        try:
            update_cargo_toml(cargo_file)
        except Exception as e:
            print(f"Error updating {cargo_file}: {e}")

if __name__ == '__main__':
    main()