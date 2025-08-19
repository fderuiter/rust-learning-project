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

echo "Installing trunk..."
wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C ~/.cargo/bin

echo "Installing wasm-bindgen-cli..."
cargo install wasm-bindgen-cli

echo "Environment setup complete."
