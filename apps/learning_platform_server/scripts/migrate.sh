#!/bin/bash
#!/bin/bash

# Learning Platform Server Migration Script

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

# Run database migrations
run_migrations() {
    print_info "Running database migrations..."
    
    # Check if DATABASE_URL is set
    if [ -z "$DATABASE_URL" ]; then
        print_warn "DATABASE_URL not set. Using default: postgresql://localhost/learning_platform"
        export DATABASE_URL="postgresql://localhost/learning_platform"
    fi
    
    # Run migrations
    cargo run --bin migrate
}

# Main function
main() {
    print_info "Running Learning Platform Server Migrations..."
    
    check_requirements
    run_migrations
    
    print_info "Migrations completed successfully!"
}

# Run main function
main "$@"