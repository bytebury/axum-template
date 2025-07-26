# ---------- Stage 1: Build Rust ----------
FROM rust:1.88-bullseye AS builder

# Install SQLite headers for sqlx
RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config

WORKDIR /app

# Copy manifest and fetch dependencies
# for optimizing the cache layer and producing quicker builds
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source and rebuild
COPY . .
RUN cargo build --release

# ---------- Stage 3: Runtime ----------
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (SQLite + TLS certs)
RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates && apt-get clean

# Copy compiled binary and assets
COPY --from=builder /app/target/release/axum-template ./app
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/migrations ./migrations

CMD ["./app"]
