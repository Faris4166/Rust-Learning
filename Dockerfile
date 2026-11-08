# ---------- Stage 1: build ----------
FROM rust:1.99 AS builder
WORKDIR /app

# cache dependencies ก่อน (แก้โค้ดแล้ว build ใหม่จะเร็วขึ้นมาก)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# ก๊อปโค้ดจริงแล้ว build
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ---------- Stage 2: runtime ----------
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/myapp /usr/local/bin/myapp
EXPOSE 3000
CMD ["myapp"]