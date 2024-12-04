# Use official Rust image
FROM rust:latest

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

# Set the DATABASE_URL for runtime
ENV DATABASE_URL=sqlite://db/local.db

# Expose the port your app runs on (adjust as needed)
EXPOSE 8080

# Run the application directly
CMD ["cargo", "run", "--release"]
