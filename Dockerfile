# Use official Rust image as base
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Copy all project files
COPY . .

# Install system dependencies if needed
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config

# Build the project in release mode
RUN cargo build --release

# Create a slim runtime image
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Create db directory
RUN mkdir -p /app/db

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/api_entry ./

# Copy .env file if it exists
COPY .env* ./

# Ensure correct permissions
RUN chmod +x /app/api_entry

# Expose the port your app runs on (adjust as needed)
EXPOSE 8080

# Set the volume for persistent database storage
VOLUME ["/app/db"]

# Command to run the application
CMD ["./api_entry"]
