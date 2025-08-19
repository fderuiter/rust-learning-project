# Environment Setup

This document outlines the steps to set up the development environment for this project.

## Rust Toolchain

The project uses a specific version of the Rust toolchain, which is defined in the `rust-toolchain.toml` file.

1.  **Install rustup**: If you don't have it already, install `rustup` by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).
2.  **Navigate to the project directory**: The `rust-toolchain.toml` file will be automatically used by `rustup` to install the correct toolchain version.
3.  **Verify the installation**: Run the following commands to verify that the correct versions of `rustc` and `cargo` are installed:
    ```bash
    rustc --version
    cargo --version
    ```
    The output should match the version specified in `rust-toolchain.toml`.

## WebAssembly Build Target

This project requires the `wasm32-unknown-unknown` build target to compile Rust code to WebAssembly.

1.  **Install the target**: Run the following command to add the target:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
2.  **Verify the installation**: You can verify that the target is installed by running:
    ```bash
    rustup target list --installed | grep wasm32-unknown-unknown
    ```
