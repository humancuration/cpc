#!/bin/bash
#!/bin/bash

# Learning Platform Server Development Script

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
    
    if ! command -v docker &> /dev/null; then
        print_warn "Docker is not installed. Database must be running separately."
    fi
    
    print_info "Requirements checked."
}

# Start database with Docker (if available)
start_database() {
    print_header "Starting Database"
    
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
    print_header "Running Database Migrations"
    
    print_info "Running database migrations..."
    cargo run --bin migrate
}

# Build the project
build_project() {
    print_header "Building Project"
    
    print_info "Building project..."
    cargo build
}

# Run tests
run_tests() {
    print_header "Running Tests"
    
    print_info "Running tests..."
    ./scripts/test.sh --all
}

# Format and lint code
lint_code() {
    print_header "Linting Code"
    
    print_info "Formatting and linting code..."
    ./scripts/lint.sh --all
}

# Generate documentation
generate_docs() {
    print_header "Generating Documentation"
    
    print_info "Generating documentation..."
    ./scripts/docs.sh --generate
}

# Start the server
start_server() {
    print_header "Starting Server"
    
    print_info "Starting Learning Platform Server..."
    cargo run
}

# Main function
main() {
    local tasks="all"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --setup)
                tasks="setup"
                shift
                ;;
            --build)
                tasks="build"
                shift
                ;;
            --test)
                tasks="test"
                shift
                ;;
            --lint)
                tasks="lint"
                shift
                ;;
            --docs)
                tasks="docs"
                shift
                ;;
            --run)
                tasks="run"
                shift
                ;;
            --all)
                tasks="all"
                shift
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  --setup        Setup development environment"
                echo "  --build        Build the project"
                echo "  --test         Run all tests"
                echo "  --lint         Format and lint code"
                echo "  --docs         Generate documentation"
                echo "  --run          Start the server"
                echo "  --all          Run all tasks (default)"
                echo "  -h, --help     Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    print_header "Learning Platform Server Development Environment"
    
    case $tasks in
        "setup")
            check_requirements
            start_database
            run_migrations
            build_project
            ;;
        "build")
            build_project
            ;;
        "test")
            run_tests
            ;;
        "lint")
            lint_code
            ;;
        "docs")
            generate_docs
            ;;
        "run")
            start_server
            ;;
        "all")
            check_requirements
            start_database
            run_migrations
            build_project
            run_tests
            lint_code
            generate_docs
            start_server
            ;;
    esac
    
    print_header "Development Task Completed"
}

# Run main function
main "$@"