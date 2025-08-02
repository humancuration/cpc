#!/bin/bash
#!/bin/bash

# Learning Platform Server Startup Script

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
    
    if ! command -v docker &> /dev/null; then
        print_warn "Docker is not installed. Database must be running separately."
    fi
    
    print_info "All requirements met."
}

# Start database with Docker (if available)
start_database() {
    if command -v docker &> /dev/null; then
        print_info "Starting database with Docker..."
        docker-compose up -d db
        
        # Wait for database to be ready
        print_info "Waiting for database to be ready..."
        sleep 10
    else
        print_warn "Docker not available. Make sure PostgreSQL is running separately."
    fi
}

# Run database migrations
run_migrations() {
    print_info "Running database migrations..."
    cargo run --bin migrate
}

# Start the server
start_server() {
    print_info "Starting Learning Platform Server..."
    cargo run
}

# Main function
main() {
    print_info "Starting Learning Platform Server..."
    
    check_requirements
    start_database
    run_migrations
    start_server
}

# Run main function
main "$@"