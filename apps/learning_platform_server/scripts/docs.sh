#!/bin/bash
#!/bin/bash

# Learning Platform Server Documentation Generation Script

# Exit on any error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
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

# Check if required tools are installed
check_requirements() {
    print_info "Checking requirements..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Rust first."
        exit 1
    fi
    
    print_info "Requirements met."
}

# Generate Rust documentation
generate_rust_docs() {
    print_info "Generating Rust documentation..."
    cargo doc --no-deps
}

# Open documentation in browser
open_docs() {
    print_info "Opening documentation in browser..."
    if command -v xdg-open &> /dev/null; then
        xdg-open target/doc/learning_platform_server/index.html
    elif command -v open &> /dev/null; then
        open target/doc/learning_platform_server/index.html
    else
        print_warn "Could not detect browser opener. Documentation is available at target/doc/learning_platform_server/index.html"
    fi
}

# Main function
main() {
    local action="generate"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --generate)
                action="generate"
                shift
                ;;
            --open)
                action="open"
                shift
                ;;
            --all)
                action="all"
                shift
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  --generate     Generate Rust documentation (default)"
                echo "  --open         Open documentation in browser"
                echo "  --all          Generate and open documentation"
                echo "  -h, --help     Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    print_info "Starting Learning Platform Server Documentation Generation..."
    
    check_requirements
    
    case $action in
        "generate")
            generate_rust_docs
            ;;
        "open")
            open_docs
            ;;
        "all")
            generate_rust_docs
            open_docs
            ;;
    esac
    
    print_info "Documentation generation completed successfully!"
}

# Run main function
main "$@"