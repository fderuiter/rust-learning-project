#!/usr/bin/env bash
set -euo pipefail

# Optional versions:
#   TRUNK_VERSION=0.21.14
#   WASM_BINDGEN_VERSION=0.2.99
# Usage:
#   TRUNK_VERSION=0.21.14 ./setup_dev.sh
#   ./setup_dev.sh

need() { command -v "$1" >/dev/null 2>&1; }

if ! need rustup; then
  echo "rustup not found. Install: https://www.rust-lang.org/tools/install"
  exit 1
fi

# Ensure toolchain/components/targets (rust-toolchain.toml will also auto-sync)
echo "Syncing Rust toolchain…"
rustup show >/dev/null
rustup component add rustfmt clippy || true
rustup target add wasm32-unknown-unknown || true

# Ensure cargo-binstall (optional, speeds up installs with prebuilt bins)
if ! need cargo-binstall; then
  echo "Installing cargo-binstall (optional)…"
  cargo install --locked cargo-binstall || true
fi

install_crate() {
  local crate="$1" ver_env="$2" ver="${!ver_env:-}"
  if need cargo-binstall; then
    if [[ -n "$ver" ]]; then
      cargo binstall -y "${crate}@${ver}" && return 0
    else
      cargo binstall -y "${crate}" && return 0
    fi
  fi
  if [[ -n "$ver" ]]; then
    cargo install --locked "$crate" --version "$ver"
  else
    cargo install --locked "$crate"
  fi
}

echo "Installing trunk…"
install_crate trunk TRUNK_VERSION

echo "Installing wasm-bindgen-cli…"
install_crate wasm-bindgen-cli WASM_BINDGEN_VERSION

echo "Verifying…"
trunk --version
wasm-bindgen --version
rustc -V
cargo -V

echo "Environment setup complete."
