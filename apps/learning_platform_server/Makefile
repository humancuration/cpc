# Makefile for learning_platform_server

# Variables
DOCKER_COMPOSE = docker-compose
CARGO = cargo
VERSION = 0.1.0

# Default target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build          - Build the project"
	@echo "  run            - Run the server"
	@echo "  test           - Run all tests"
	@echo "  test-unit      - Run unit tests"
	@echo "  test-int       - Run integration tests"
	@echo "  test-doc       - Run documentation tests"
	@echo "  bench          - Run benchmarks"
	@echo "  lint           - Format, check, and run clippy"
	@echo "  fmt            - Format code"
	@echo "  fmt-check      - Check code formatting"
	@echo "  clippy         - Run clippy"
	@echo "  clippy-fix     - Run clippy with fixes"
	@echo "  docs           - Generate and open documentation"
	@echo "  docs-gen       - Generate documentation"
	@echo "  docs-open      - Open documentation in browser"
	@echo "  health         - Check service health"
	@echo "  dev            - Run all development tasks"
	@echo "  dev-setup      - Setup development environment"
	@echo "  package        - Create distribution package"
	@echo "  docker-build   - Build Docker image"
	@echo "  docker-up      - Start services with Docker Compose"
	@echo "  docker-down    - Stop services with Docker Compose"
	@echo "  docker-logs    - View logs from Docker containers"
	@echo "  migrate        - Run database migrations"
	@echo "  clean          - Clean build artifacts"
	@echo "  client         - Run example client"

# Build the project
.PHONY: build
build:
	$(CARGO) build

# Run the server
.PHONY: run
run:
	$(CARGO) run

# Run all tests
.PHONY: test
test:
	./scripts/test.sh --all

# Run unit tests
.PHONY: test-unit
test-unit:
	./scripts/test.sh --unit

# Run integration tests
.PHONY: test-int
test-int:
	./scripts/test.sh --integration

# Run documentation tests
.PHONY: test-doc
test-doc:
	./scripts/test.sh --doc

# Run benchmarks
.PHONY: bench
bench:
	./scripts/test.sh --bench

# Format, check, and run clippy
.PHONY: lint
lint:
	./scripts/lint.sh --all

# Format code
.PHONY: fmt
fmt:
	./scripts/lint.sh --format

# Check code formatting
.PHONY: fmt-check
fmt-check:
	./scripts/lint.sh --check

# Run clippy
.PHONY: clippy
clippy:
	./scripts/lint.sh --clippy

# Run clippy with fixes
.PHONY: clippy-fix
clippy-fix:
	./scripts/lint.sh --fix

# Generate and open documentation
.PHONY: docs
docs:
	./scripts/docs.sh --all

# Generate documentation
.PHONY: docs-gen
docs-gen:
	./scripts/docs.sh --generate

# Open documentation in browser
.PHONY: docs-open
docs-open:
	./scripts/docs.sh --open

# Check service health
.PHONY: health
health:
	./scripts/health.sh

# Run all development tasks
.PHONY: dev
dev:
	./scripts/dev.sh --all

# Setup development environment
.PHONY: dev-setup
dev-setup:
	./scripts/dev.sh --setup

# Create distribution package
.PHONY: package
package:
	./scripts/package.sh --version $(VERSION)

# Build Docker image
.PHONY: docker-build
docker-build:
	$(DOCKER_COMPOSE) build

# Start services with Docker Compose
.PHONY: docker-up
docker-up:
	$(DOCKER_COMPOSE) up -d

# Stop services with Docker Compose
.PHONY: docker-down
docker-down:
	$(DOCKER_COMPOSE) down

# View logs from Docker containers
.PHONY: docker-logs
docker-logs:
	$(DOCKER_COMPOSE) logs -f

# Run database migrations
.PHONY: migrate
migrate:
	$(CARGO) run --bin migrate

# Clean build artifacts
.PHONY: clean
clean:
	$(CARGO) clean

# Run example client
.PHONY: client
client:
	$(CARGO) run --example client