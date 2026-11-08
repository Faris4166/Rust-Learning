# ---------- Stage 1: build ----------
FROM rust:1.99 AS builder
WORKDIR /app

COPY helloworld/Cargo.toml helloworld/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY helloworld/src ./src
RUN touch src/main.rs && cargo build --release

# ---------- Stage 2: runtime ----------
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust_learning /usr/local/bin/rust_learning
CMD ["rust_learning"]
