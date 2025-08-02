# Makefile for learning_platform_server

# Variables
DOCKER_COMPOSE = docker-compose
CARGO = cargo

# Default target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build          - Build the project"
	@echo "  run            - Run the server"
	@echo "  test           - Run tests"
	@echo "  docker-build   - Build Docker image"
	@echo "  docker-up      - Start services with Docker Compose"
	@echo "  docker-down    - Stop services with Docker Compose"
	@echo "  docker-logs    - View logs from Docker containers"
	@echo "  clean          - Clean build artifacts"

# Build the project
.PHONY: build
build:
	$(CARGO) build

# Run the server
.PHONY: run
run:
	$(CARGO) run

# Run tests
.PHONY: test
test:
	$(CARGO) test

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

# Clean build artifacts
.PHONY: clean
clean:
	$(CARGO) clean

# Format code
.PHONY: fmt
fmt:
	$(CARGO) fmt

# Check code formatting
.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --check

# Run clippy
.PHONY: clippy
clippy:
	$(CARGO) clippy

# Run example client
.PHONY: client
client:
	$(CARGO) run --example client