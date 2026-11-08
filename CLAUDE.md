# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

A personal Rust learning repository (`Rust-Learning`). Each exercise or project lives in its own standalone Cargo crate in a subdirectory (e.g. `helloworld/`). There is no Cargo workspace at the root, so run Cargo commands from inside the crate's directory, or pass `--manifest-path <crate>/Cargo.toml`.

Crates use `edition = "2024"`.

## Commands

Run these inside a crate directory such as `helloworld/`:

- Build: `cargo build` (release build: `cargo build --release`)
- Run: `cargo run`
- Test all: `cargo test`
- Run a single test: `cargo test <test_name_substring>` (add `-- --exact` for an exact name match)
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Docker

The root [Dockerfile](Dockerfile) does a multi-stage build: `rust:1.99` builds the binary, and `debian:trixie-slim` runs it. It caches dependencies by building a stub `main.rs` first. It is hardcoded to the `helloworld/` crate, whose package name is `rust_learning`, so the binary is `rust_learning`. If you rename the crate's package or point the image at another crate, update the `COPY` paths and the binary name in the Dockerfile to match.

Build and run from the repo root:

- `docker build -t rust_learning .`
- `docker run --rm rust_learning`
