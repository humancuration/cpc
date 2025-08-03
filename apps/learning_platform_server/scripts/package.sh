#!/bin/bash
#!/bin/bash

# Learning Platform Server Packaging Script

# Exit on any error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

# Check if required tools are installed
check_requirements() {
    print_header "Checking Requirements"
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Rust first."
        exit 1
    fi
    
    print_info "Requirements checked."
}

# Build the project in release mode
build_release() {
    print_header "Building Release"
    
    print_info "Building project in release mode..."
    cargo build --release
}

# Create distribution package
create_package() {
    local version=${1:-"0.1.0"}
    local target_dir="dist"
    local package_name="learning-platform-server-$version"
    
    print_header "Creating Package"
    
    # Create target directory
    rm -rf "$target_dir"
    mkdir -p "$target_dir/$package_name"
    
    # Copy binaries
    print_info "Copying binaries..."
    cp "target/release/learning_platform_server" "$target_dir/$package_name/"
    cp "target/release/migrate" "$target_dir/$package_name/"
    
    # Copy configuration files
    print_info "Copying configuration files..."
    cp .env "$target_dir/$package_name/" 2>/dev/null || print_warn ".env file not found"
    cp docker-compose.yml "$target_dir/$package_name/" 2>/dev/null || print_warn "docker-compose.yml file not found"
    
    # Copy documentation
    print_info "Copying documentation..."
    cp -r docs "$target_dir/$package_name/" 2>/dev/null || print_warn "docs directory not found"
    cp README.md "$target_dir/$package_name/" 2>/dev/null || print_warn "README.md file not found"
    cp API.md "$target_dir/$package_name/" 2>/dev/null || print_warn "API.md file not found"
    
    # Copy scripts
    print_info "Copying scripts..."
    cp -r scripts "$target_dir/$package_name/" 2>/dev/null || print_warn "scripts directory not found"
    
    # Copy migrations
    print_info "Copying migrations..."
    cp -r migrations "$target_dir/$package_name/" 2>/dev/null || print_warn "migrations directory not found"
    
    # Make scripts executable
    print_info "Setting executable permissions..."
    find "$target_dir/$package_name/scripts" -name "*.sh" -exec chmod +x {} \; 2>/dev/null || true
    
    # Create package
    print_info "Creating package archive..."
    cd "$target_dir"
    tar -czf "$package_name.tar.gz" "$package_name"
    cd ..
    
    print_info "Package created: $target_dir/$package_name.tar.gz"
}

# Main function
main() {
    local version="0.1.0"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -v|--version)
                version="$2"
                shift 2
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  -v, --version VERSION  Package version (default: 0.1.0)"
                echo "  -h, --help            Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    print_header "Learning Platform Server Packaging"
    
    check_requirements
    build_release
    create_package "$version"
    
    print_header "Packaging Completed"
}

# Run main function
main "$@"