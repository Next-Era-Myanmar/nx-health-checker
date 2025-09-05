# Multi-stage Docker build for NX Health Checker
# Stage 1: Build stage with Rust 1.87
FROM rust:1.87-slim AS builder

# Set working directory
WORKDIR /app

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY static ./static
COPY config.env ./

# Download dependencies (this step is cached if the files haven't changed)
RUN cargo fetch

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build the application
RUN cargo build --release

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release /app

# Copy static files and config
COPY --from=builder /app/static /app/static
COPY --from=builder /app/config.env /app/config.env

# Expose port
EXPOSE 3030

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:health_check.db?mode=rwc

# Run the application
CMD ["./nx_health_checker"]
