#!/bin/bash
set -e

# Check if rustup is installed
if ! command -v rustup &> /dev/null
then
    echo "rustup could not be found. Please install Rust from https://www.rust-lang.org/tools/install"
    exit 1
fi

echo "Ensuring wasm32-unknown-unknown target is installed..."
rustup target add wasm32-unknown-unknown

echo "Environment setup complete."
