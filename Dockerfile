# Stage 1: Build the application using latest Rust
FROM rust:latest as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/winter-heart

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy full source and build
COPY . .
RUN cargo build --release

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/winter-heart/target/release/winter-heart /usr/local/bin/winter-heart

ENV DISCORD_TOKEN=""
ENV GUILD_ID=""

ENTRYPOINT ["winter-heart"]

