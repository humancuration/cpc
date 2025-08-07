# Shtairir Visual Editor Makefile

# Variables
PACKAGE_NAME = shtairir_editor
DIST_DIR = dist
PKG_DIR = pkg

# Default target
.PHONY: help
help:
	@echo "Shtairir Visual Editor - Build Commands"
	@echo ""
	@echo "Available commands:"
	@echo "  build-web     - Build for web target"
	@echo "  build-dev     - Build for web target (development mode)"
	@echo "  serve         - Serve locally for development"
	@echo "  clean         - Clean build artifacts"
	@echo "  help          - Show this help message"

# Build for web target
.PHONY: build-web
build-web:
	@echo "Building for web target..."
	wasm-pack build --target web --out-dir $(PKG_DIR)
	@echo "Build complete. Files are in $(PKG_DIR)/"

# Build for web target (development mode)
.PHONY: build-dev
build-dev:
	@echo "Building for web target (development mode)..."
	wasm-pack build --target web --dev --out-dir $(PKG_DIR)
	@echo "Development build complete. Files are in $(PKG_DIR)/"

# Serve locally for development
.PHONY: serve
serve: build-dev
	@echo "Serving files locally..."
	@echo "Open http://localhost:8000 in your browser"
	python -m http.server 8000

# Clean build artifacts
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	rm -rf $(PKG_DIR) $(DIST_DIR)
	cargo clean
	@echo "Clean complete."

# Install wasm-pack if not present
.PHONY: install-wasm-pack
install-wasm-pack:
	@which wasm-pack > /dev/null || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Setup development environment
.PHONY: setup
setup: install-wasm-pack
	@echo "Development environment setup complete."

# Run tests
.PHONY: test
test:
	@echo "Running tests..."
	cargo test

# Check code formatting
.PHONY: fmt
fmt:
	@echo "Formatting code..."
	cargo fmt

# Check for linting errors
.PHONY: lint
lint:
	@echo "Running clippy lints..."
	cargo clippy -- -D warnings

# Build documentation
.PHONY: docs
docs:
	@echo "Building documentation..."
	cargo doc --no-deps
	@echo "Documentation built. Open target/doc/$(PACKAGE_NAME)/index.html to view."

# Publish to crates.io (requires credentials)
.PHONY: publish
publish:
	@echo "Publishing to crates.io..."
	cargo publish