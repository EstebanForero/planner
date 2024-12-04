# Use official Rust image as base
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Copy all project files
COPY . .

# Set the DATABASE_URL for the build process
ENV DATABASE_URL=sqlite://db/local.db

# Install system dependencies if needed
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    sqlite3 \
    libsqlite3-dev

# Build the project in release mode
RUN cargo build --release

# Create a slim runtime image
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Create db directory
RUN mkdir -p /app/db

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/api_entry ./

# Copy .env file if it exists
COPY .env* ./

# Set the DATABASE_URL for runtime
ENV DATABASE_URL=sqlite://db/local.db

# Ensure correct permissions
RUN chmod +x /app/api_entry

# Expose the port your app runs on (adjust as needed)
EXPOSE 8080

# Command to run the application
CMD ["./api_entry"]
