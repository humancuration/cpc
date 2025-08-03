#!/bin/bash

# Learning Platform Server Security Check Script

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

# Run cargo-audit to check for known vulnerabilities
run_cargo_audit() {
    print_header "Running Cargo Audit"
    
    if ! command -v cargo-audit &> /dev/null; then
        print_warn "cargo-audit is not installed. Installing..."
        cargo install cargo-audit
    fi
    
    print_info "Checking for known vulnerabilities..."
    if cargo audit; then
        print_info "No known vulnerabilities found!"
    else
        print_error "Vulnerabilities found! Please review the output above."
        return 1
    fi
}

# Run cargo-deny to check licensing and advisories
run_cargo_deny() {
    print_header "Running Cargo Deny"
    
    if ! command -v cargo-deny &> /dev/null; then
        print_warn "cargo-deny is not installed. Installing..."
        cargo install cargo-deny
    fi
    
    print_info "Checking licenses and advisories..."
    if cargo deny check; then
        print_info "License and advisory checks passed!"
    else
        print_error "License or advisory issues found! Please review the output above."
        return 1
    fi
}

# Check for outdated dependencies
check_outdated_deps() {
    print_header "Checking for Outdated Dependencies"
    
    print_info "Checking for outdated dependencies..."
    if cargo update --dry-run; then
        print_info "Dependencies are up to date!"
    else
        print_warn "Some dependencies may be outdated. Consider running 'cargo update'."
    fi
}

# Main function
main() {
    local checks="all"
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --audit)
                checks="audit"
                shift
                ;;
            --deny)
                checks="deny"
                shift
                ;;
            --outdated)
                checks="outdated"
                shift
                ;;
            --all)
                checks="all"
                shift
                ;;
            -h|--help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  --audit     Run cargo-audit"
                echo "  --deny      Run cargo-deny"
                echo "  --outdated  Check for outdated dependencies"
                echo "  --all       Run all security checks (default)"
                echo "  -h, --help  Show this help message"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    print_header "Learning Platform Server Security Check"
    
    check_requirements
    
    case $checks in
        "audit")
            run_cargo_audit
            ;;
        "deny")
            run_cargo_deny
            ;;
        "outdated")
            check_outdated_deps
            ;;
        "all")
            run_cargo_audit
            run_cargo_deny
            check_outdated_deps
            ;;
    esac
    
    print_header "Security Check Completed"
}

# Run main function
main "$@"