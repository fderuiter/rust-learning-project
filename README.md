# Rust Wasm Face Stretcher

[![CI](https://github.com/fderuiter/rust-learning-project/workflows/CI/badge.svg)](https://github.com/fderuiter/rust-learning-project/actions)

A "Mario 64–style" face stretcher application built with Rust and WebAssembly. This project demonstrates how to integrate a Rust-powered physics simulation and rendering logic with a JavaScript frontend using Three.js.

## Project Structure

This project is organized as a Cargo workspace with a multi-crate setup to promote modularity and separation of concerns.

-   `crates/`: Contains all the Rust crates.
    -   `wasm-app`: The main WebAssembly library that interfaces with JavaScript.
    -   `physics`: The soft-body physics simulation.
    -   `mesh`: 3D mesh data structures and logic.
    -   `image-processing`: Image filtering effects.
    -   `face-detection`: (Optional) Face detection logic.
-   `js/`: Frontend JavaScript code.
-   `static/`: Static assets like `index.html` and images.
-   `ARCHITECTURE.md`: A detailed document explaining the project's architecture.

For a more detailed breakdown, please see the [Architecture](ARCHITECTURE.md) document.

## Getting Started

### Prerequisites

-   [Rust toolchain](https://www.rust-lang.org/tools/install)
-   `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
-   [Trunk](https://trunkrs.dev/#install)

For a one-time setup of the development environment, you can run the provided script:
```bash
./setup_dev.sh
```

### Running the Development Server

To build the application and start a local development server, run:
```bash
trunk serve
```
The application will be available at `http://localhost:8080`. The server will automatically rebuild the application when you make changes to the code.

## Building

To build the application for production, run:
```bash
make build-release
```
This will create an optimized build in the `dist/` directory.

## Testing

This project includes both native Rust tests and WebAssembly-specific tests.

### Rust Unit & Integration Tests
To run all the tests for the native Rust crates, use:
```bash
cargo test --all-features --workspace
```

### WebAssembly (Browser) Tests
To run the Wasm integration tests in a headless browser, you'll need to navigate to the `wasm-app` crate:
```bash
cd crates/wasm-app
wasm-pack test --headless
```

## License

This project is dual-licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more details.
