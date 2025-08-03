# Makefile for the Live Streaming module

# Variables
CARGO = cargo
RUSTC = rustc
APP_NAME = live-streaming

# Default target
.PHONY: default
default: build

# Build the application
.PHONY: build
build:
	$(CARGO) build

# Build the application in release mode
.PHONY: release
release:
	$(CARGO) build --release

# Run the application
.PHONY: run
run:
	$(CARGO) run

# Run the application in release mode
.PHONY: run-release
run-release:
	$(CARGO) run --release

# Run tests
.PHONY: test
test:
	$(CARGO) test

# Run tests with coverage
.PHONY: test-coverage
test-coverage:
	$(CARGO) tarpaulin --out Html

# Format the code
.PHONY: fmt
fmt:
	$(CARGO) fmt

# Check code formatting
.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --check

# Run clippy for linting
.PHONY: lint
lint:
	$(CARGO) clippy --all-targets --all-features

# Run clippy with fixes
.PHONY: lint-fix
lint-fix:
	$(CARGO) clippy --all-targets --all-features --fix

# Generate documentation
.PHONY: docs
docs:
	$(CARGO) doc --open

# Clean build artifacts
.PHONY: clean
clean:
	$(CARGO) clean

# Install dependencies
.PHONY: install
install:
	$(CARGO) check

# Build Tauri application
.PHONY: tauri-build
tauri-build:
	$(CARGO) tauri build

# Develop Tauri application
.PHONY: tauri-dev
tauri-dev:
	$(CARGO) tauri dev

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build         - Build the application"
	@echo "  release       - Build the application in release mode"
	@echo "  run           - Run the application"
	@echo "  run-release   - Run the application in release mode"
	@echo "  test          - Run tests"
	@echo "  fmt           - Format the code"
	@echo "  fmt-check     - Check code formatting"
	@echo "  lint          - Run clippy for linting"
	@echo "  docs          - Generate documentation"
	@echo "  clean         - Clean build artifacts"
	@echo "  install       - Install dependencies"
	@echo "  tauri-build   - Build Tauri application"
	@echo "  tauri-dev     - Develop Tauri application"
	@echo "  help          - Show this help message"