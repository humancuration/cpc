# Shtairir Ecosystem Makefile

# Default target
.PHONY: all
all: build test

# Build all components
.PHONY: build
build:
	cargo build --workspace

# Run tests
.PHONY: test
test:
	cargo test --workspace

# Run tests for shtairir package only
.PHONY: test-shtairir
test-shtairir:
	cargo test -p shtairir

# Run benchmarks
.PHONY: bench
bench:
	cargo bench -p shtairir

# Run example application
.PHONY: run-example
run-example:
	cargo run -p shtairir_example

# Run CLI tool
.PHONY: run-cli
run-cli:
	cargo run -p shtairir_cli -- --script examples/hello_world.sht

# Check code formatting
.PHONY: fmt
fmt:
	cargo fmt --all -- --check

# Run clippy linting
.PHONY: clippy
clippy:
	cargo clippy --workspace -- -D warnings

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Generate documentation
.PHONY: doc
doc:
	cargo doc --workspace --no-deps

# Install CLI tool
.PHONY: install-cli
install-cli:
	cargo install --path apps/shtairir_cli

# Help
.PHONY: help
help:
	@echo "Shtairir Ecosystem Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  all             Build and test everything"
	@echo "  build           Build all components"
	@echo "  test            Run all tests"
	@echo "  test-shtairir   Run tests for shtairir package only"
	@echo "  bench           Run benchmarks"
	@echo "  run-example     Run example application"
	@echo "  run-cli         Run CLI tool with example script"
	@echo "  fmt             Check code formatting"
	@echo "  clippy          Run clippy linting"
	@echo "  clean           Clean build artifacts"
	@echo "  doc             Generate documentation"
	@echo "  install-cli     Install CLI tool"
	@echo "  help            Show this help message"