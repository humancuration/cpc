# Makefile for music player module

# Variables
CARGO = cargo
TARGET = music-player

# Default target
.PHONY: default
default: build

# Build the project
.PHONY: build
build:
	$(CARGO) build

# Build in release mode
.PHONY: release
release:
	$(CARGO) build --release

# Run the project
.PHONY: run
run:
	$(CARGO) run

# Run tests
.PHONY: test
test:
	$(CARGO) test

# Run tests with coverage
.PHONY: coverage
coverage:
	$(CARGO) tarpaulin --out Html

# Format the code
.PHONY: fmt
fmt:
	$(CARGO) fmt

# Check for linting issues
.PHONY: clippy
clippy:
	$(CARGO) clippy -- -D warnings

# Check documentation
.PHONY: doc
doc:
	$(CARGO) doc --no-deps --open

# Clean build artifacts
.PHONY: clean
clean:
	$(CARGO) clean

# Install dependencies
.PHONY: install
install:
	$(CARGO) check

# Run all checks
.PHONY: check
check: fmt clippy test

# Help
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build     - Build the project"
	@echo "  release   - Build in release mode"
	@echo "  run       - Run the project"
	@echo "  test      - Run tests"
	@echo "  coverage  - Run tests with coverage"
	@echo "  fmt       - Format the code"
	@echo "  clippy    - Check for linting issues"
	@echo "  doc       - Check documentation"
	@echo "  clean     - Clean build artifacts"
	@echo "  install   - Install dependencies"
	@echo "  check     - Run all checks"
	@echo "  help      - Show this help"

# Default target
.DEFAULT_GOAL := help