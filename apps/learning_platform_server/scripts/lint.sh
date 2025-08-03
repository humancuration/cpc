#!/bin/bash
#!/bin/bash

# Learning Platform Server Lint Script

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

# Format code
format_code() {
    print_info "Formatting code..."
    cargo fmt
}

# Check code formatting
check_formatting() {
    print_info "Checking code formatting..."
    cargo fmt --check
}

# Run clippy
run_clippy() {
    print_info "Running clippy..."
    cargo clippy -- -D warnings
}

# Run clippy with fixes
run_clippy_fix() {
    print_info "Running clippy with fixes..."
    cargo clippy --fix --allow-dirty --allow-staged
}

# Main function
main() {
    local action="all"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --format)
                action="format"
                shift
                ;;
            --check)
                action="check"
                shift
                ;;
            --clippy)
                action="clippy"
                shift
                ;;
            --fix)
                action="fix"
                shift
                ;;
            --all)
                action="all"
                shift
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  --format       Format code"
                echo "  --check        Check code formatting"
                echo "  --clippy       Run clippy"
                echo "  --fix          Run clippy with fixes"
                echo "  --all          Format, check, and run clippy (default)"
                echo "  -h, --help     Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    print_info "Starting Learning Platform Server Linting..."
    
    check_requirements
    
    case $action in
        "format")
            format_code
            ;;
        "check")
            check_formatting
            ;;
        "clippy")
            run_clippy
            ;;
        "fix")
            run_clippy_fix
            ;;
        "all")
            format_code
            check_formatting
            run_clippy
            ;;
    esac
    
    print_info "Linting completed successfully!"
}

# Run main function
main "$@"