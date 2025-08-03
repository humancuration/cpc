#!/bin/bash
#!/bin/bash

# Learning Platform Server Test Script

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

# Run unit tests
run_unit_tests() {
    print_info "Running unit tests..."
    cargo test --lib
}

# Run integration tests
run_integration_tests() {
    print_info "Running integration tests..."
    
    # Check if DATABASE_URL is set
    if [ -z "$DATABASE_URL" ]; then
        print_warn "DATABASE_URL not set. Using default: postgresql://localhost/learning_platform"
        export DATABASE_URL="postgresql://localhost/learning_platform"
    fi
    
    cargo test --test "*"
}

# Run documentation tests
run_doc_tests() {
    print_info "Running documentation tests..."
    cargo test --doc
}

# Run all tests
run_all_tests() {
    print_info "Running all tests..."
    
    # Check if DATABASE_URL is set
    if [ -z "$DATABASE_URL" ]; then
        print_warn "DATABASE_URL not set. Using default: postgresql://localhost/learning_platform"
        export DATABASE_URL="postgresql://localhost/learning_platform"
    fi
    
    cargo test
}

# Run benchmarks
run_benchmarks() {
    print_info "Running benchmarks..."
    cargo bench
}

# Main function
main() {
    local test_type="all"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --unit)
                test_type="unit"
                shift
                ;;
            --integration)
                test_type="integration"
                shift
                ;;
            --doc)
                test_type="doc"
                shift
                ;;
            --bench)
                test_type="bench"
                shift
                ;;
            --all)
                test_type="all"
                shift
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  --unit         Run unit tests only"
                echo "  --integration  Run integration tests only"
                echo "  --doc          Run documentation tests only"
                echo "  --bench        Run benchmarks only"
                echo "  --all          Run all tests (default)"
                echo "  -h, --help     Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    print_info "Starting Learning Platform Server Tests..."
    
    check_requirements
    
    case $test_type in
        "unit")
            run_unit_tests
            ;;
        "integration")
            run_integration_tests
            ;;
        "doc")
            run_doc_tests
            ;;
        "bench")
            run_benchmarks
            ;;
        "all")
            run_all_tests
            ;;
    esac
    
    print_info "Tests completed successfully!"
}

# Run main function
main "$@"