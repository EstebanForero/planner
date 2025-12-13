FROM rust:latest AS builder

WORKDIR /app

# Install native dependencies for building crates that rely on OpenSSL (e.g. sqlx)
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENV SQLX_OFFLINE=true

COPY . .

RUN cargo build --release --bin api_entry

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api_entry /usr/local/bin/api_entry
COPY --from=builder /app/general_repository/migrations /app/migrations

ENV RUST_LOG=info

EXPOSE 8080

CMD ["api_entry"]
