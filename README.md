# Rust Wasm Face Stretcher

[![CI](https://github.com/fderuiter/rust-learning-project/workflows/CI/badge.svg)](https://github.com/fderuiter/rust-learning-project/actions)

A "Mario 64–style" face stretcher application built with Rust and WebAssembly. This project demonstrates how to integrate a Rust-powered physics simulation and rendering logic with a JavaScript frontend using Three.js.

<!-- Add a screenshot or GIF of the application in action here -->
<!-- e.g., ![Face Stretcher Demo](docs/demo.gif) -->

## Purpose and Features

This project serves as a comprehensive example of building a web application with Rust and WebAssembly. It showcases several key concepts and technologies, including:

- **Soft-body physics**: A Verlet integration-based physics engine simulates a "squishy" face mesh.
- **3D rendering**: The frontend is built with Three.js, a popular WebGL library.
- **Rust and Wasm integration**: The core logic is written in Rust and compiled to WebAssembly, which communicates with the JavaScript frontend.
- **Modular architecture**: The Rust code is organized into a multi-crate workspace, promoting code reuse and separation of concerns.
- **Image processing**: The application can apply filters like grayscale and sepia to textures.
- **Face detection**: (Optional) The application can detect faces in uploaded images to automatically place control points.

## Project Structure

This project is organized as a Cargo workspace with a multi-crate setup to promote modularity and separation of concerns.

-   `crates/`: Contains all the Rust crates.
    -   `wasm-app`: The main WebAssembly library that exposes the application's core logic to JavaScript. It integrates the other crates and provides a simple API for the frontend.
    -   `physics`: A simple soft-body physics engine that uses Verlet integration to simulate the movement of the face mesh.
    -   `mesh`: Contains the data structures and logic for representing a 3D mesh, including vertices and indices.
    -   `image-processing`: A crate for applying image filtering effects, such as grayscale and sepia, to the mesh's texture.
    -   `face-detection`: An optional crate that uses a pre-trained TensorFlow model to detect faces in images.
-   `js/`: Frontend JavaScript code that handles rendering with Three.js, user input, and communication with the Wasm module.
-   `static/`: Static assets like `index.html`, the 3D model, and other resources.
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

## Usage

Once the application is running, you can interact with it as follows:

1.  **Load the default model**: The application starts with a default face model.
2.  **Drag the vertices**: Click and drag any part of the face to stretch and deform it.
3.  **Upload an image**: Click the "Upload Image" button to select an image from your computer. The image will be applied as a texture to the face mesh.
4.  **Face detection**: If a face is detected in the uploaded image, control points will be placed on the eyes, nose, and mouth, allowing you to manipulate these features.
5.  **Apply filters**: Use the "Grayscale" and "Sepia" buttons to apply image filters to the texture.

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
