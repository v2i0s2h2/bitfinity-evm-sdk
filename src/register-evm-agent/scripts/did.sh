#!/usr/bin/env bash

# Set the canister name
CANISTER="register-evm-agent"

# Build the canister
cargo build --manifest-path="Cargo.toml" --target wasm32-unknown-unknown --release

# Generate the DID
cargo run --manifest-path="Cargo.toml" --bin register-evm-agent -- did "$CANISTER"
