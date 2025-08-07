# Makefile for Shtairir Demo Applications

# Default target
.PHONY: help
help:
	@echo "Shtairir Demo Applications Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  check        - Verify all examples compile and run"
	@echo "  test         - Run all tests"
	@echo "  clean        - Clean build artifacts"
	@echo "  docs         - Generate documentation"
	@echo "  help         - Show this help message"

# Check all examples
.PHONY: check
check:
	@echo "🔍 Checking all Shtairir demo applications..."
	@if [ -f "check_examples.sh" ]; then \
		bash check_examples.sh; \
	elif [ -f "check_examples.ps1" ]; then \
		pwsh check_examples.ps1; \
	else \
		echo "❌ No verification script found"; \
		exit 1; \
	fi

# Run all tests
.PHONY: test
test:
	@echo "🧪 Running all tests..."
	@cargo test

# Clean build artifacts
.PHONY: clean
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean

# Generate documentation
.PHONY: docs
docs:
	@echo "📚 Generating documentation..."
	@cargo doc --no-deps

# Run data processing demo
.PHONY: run-data-processing
run-data-processing:
	@echo "🏃 Running data processing demo..."
	@cargo run -p shtairir_demos_data_processing

# Run user profiles demo
.PHONY: run-user-profiles
run-user-profiles:
	@echo "🏃 Running user profiles demo..."
	@cargo run -p shtairir_demos_user_profiles

# Run ML features demo
.PHONY: run-ml-features
run-ml-features:
	@echo "🏃 Running ML features demo..."
	@cargo run -p shtairir_demos_ml_features

# Run all demos
.PHONY: run-all
run-all: run-data-processing run-user-profiles run-ml-features