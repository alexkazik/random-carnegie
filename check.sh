#!/bin/sh
cargo build --target wasm32-unknown-unknown --release
cargo clippy --target wasm32-unknown-unknown --release
cargo +nightly clippy --target wasm32-unknown-unknown --release
cargo +nightly fmt --all
