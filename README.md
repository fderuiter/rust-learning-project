# rust-learning-project

[![Crates.io](https://img.shields.io/crates/v/rust-learning-project.svg)](https://crates.io/crates/rust-learning-project)
[![Docs.rs](https://docs.rs/rust-learning-project/badge.svg)](https://docs.rs/rust-learning-project)
[![CI](https://github.com/fderuiter/rust-learning-project/workflows/CI/badge.svg)](https://github.com/fderuiter/rust-learning-project/actions)

## Installation

### Development Environment Setup

For a quick setup of your development environment, run the provided script. This will ensure you have the necessary WebAssembly build target installed.

```bash
./setup_dev.sh
```

If you prefer a manual setup, follow the instructions below.

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install rust-learning-project`

### WebAssembly Target

* Install the WebAssembly build target, which is required for compiling to Wasm:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

## Development

To start the development server, run:

```bash
trunk serve
```

This will build the application, start a server, and watch for any file changes. You can view the application at `http://localhost:8080`.

## Project Layout

-   `src/`: Contains the Rust source code for the application logic.
    -   `face_detection.rs`: Face detection using TensorFlow.
    -   `image_processing.rs`: Image filtering using `photon-rs`.
-   `main.js`: The main JavaScript entry point for the application. It sets up the Three.js scene and interacts with the Wasm module.
-   `index.html`: The main HTML file for the application.
-   `Cargo.toml`: The Rust package manager configuration file.
-   `rust-toolchain.toml`: Specifies the Rust toolchain version.
-   `ARCHITECTURE.md`: Documents the architectural decisions for the project.
-   `docs/`: Contains additional documentation.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
