# Use Rust official image as base
FROM rust:1.70 as builder

# Create working directory
WORKDIR /usr/src/learning_platform_server

# Copy project files
COPY . .

# Build the application
RUN cargo build --release

# Use Debian slim image for runtime
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 app
USER app

# Create working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /usr/src/learning_platform_server/target/release/learning_platform_server .

# Expose port
EXPOSE 50051

# Run the application
CMD ["./learning_platform_server"]