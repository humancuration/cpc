#!/bin/bash
#!/bin/bash

# Learning Platform Server Health Check Script

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
    
    if ! command -v grpcurl &> /dev/null; then
        print_warn "grpcurl is not installed. Installing..."
        # Try to install grpcurl
        if command -v go &> /dev/null; then
            go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
        else
            print_error "Go is not installed. Please install grpcurl manually."
            exit 1
        fi
    fi
    
    print_info "Requirements met."
}

# Check service health
check_health() {
    local server_addr=${1:-"localhost:50051"}
    
    print_info "Checking health of service at $server_addr..."
    
    # Try to call the health check endpoint
    if grpcurl -plaintext "$server_addr" grpc.health.v1.Health/Check; then
        print_info "Service is healthy!"
        return 0
    else
        print_error "Service health check failed!"
        return 1
    fi
}

# Main function
main() {
    local server_addr="localhost:50051"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -a|--address)
                server_addr="$2"
                shift 2
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  -a, --address ADDR  Server address (default: localhost:50051)"
                echo "  -h, --help          Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    print_info "Starting Learning Platform Server Health Check..."
    
    check_requirements
    check_health "$server_addr"
}

# Run main function
main "$@"