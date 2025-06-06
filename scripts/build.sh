#!/usr/bin/env bash
set -euo pipefail

# Build the WebAssembly frontend
(
  cd web
  wasm-pack build --release --target web --out-dir ../static/pkg
)

# Build the Rust backend
cargo build --release
