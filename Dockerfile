# Build stage
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Copy all project files
COPY . .

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    sqlite3 \
    libsqlite3-dev

# Create db directory with full permissions
RUN mkdir -p /app/db && \
    chmod 777 /app/db

# Build the project
RUN cargo build --release

# Production stage (identical to builder)
FROM rust:latest

# Set working directory
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Create db directory with full permissions
RUN mkdir -p /app/db && \
    chmod 777 /app/db

# Copy files from builder stage
COPY --from=builder /app /app

# Set the DATABASE_URL for runtime
ENV DATABASE_URL=sqlite://db/local.db

# Expose the port your app runs on (adjust as needed)
EXPOSE 8080

# Run the application
CMD ["./target/release/api_entry"]
