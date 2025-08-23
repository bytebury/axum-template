ARG RUN_ID

# ---------- Stage 1: Build Rust ----------
FROM rust:1.88-bullseye AS builder
ARG RUN_ID

# Install SQLite headers and OpenSSL dev for sqlx with TLS
RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config libssl-dev

WORKDIR /app

# Optimize Docker caching by fetching dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source and rebuild
COPY . .
RUN cargo build --release

# --------- Stage 2: Build TailwindCSS ---------
FROM node:20-bullseye AS tailwind
ARG RUN_ID

WORKDIR /build

COPY public ./public
COPY templates ./templates
COPY package.json package-lock.json ./

RUN npm ci
RUN npx tailwindcss -i "./public/styles/tailwind.css" -o "./public/styles/main-${RUN_ID}.min.css" --minify


# Rename all .js files in ./public to include RUN_ID. E.g. main.js -> main-12345.js
RUN find ./public -name "*.js" -type f -exec bash -c 'mv "$0" "${0%.js}-'"${RUN_ID}"'.js"' {} \;

# ---------- Stage 3: Runtime ----------
FROM debian:bullseye-slim
ARG RUN_ID

WORKDIR /app

# Install runtime dependencies: SQLite, certs, and OpenSSL 1.1
RUN apt-get update && apt-get install -y libsqlite3-0 ca-certificates libssl1.1 && apt-get clean

# Copy compiled binary and assets
COPY --from=builder /app/target/release/axum-template ./app
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/migrations ./migrations
COPY --from=tailwind /build/public ./public

CMD ["./app"]
