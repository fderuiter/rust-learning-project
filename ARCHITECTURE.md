# Architecture

This document outlines the architectural decisions made for this project.

## High-Level Overview

The application is composed of three main parts:

1.  **Frontend**: A web interface built with HTML, CSS, and JavaScript, located in the `static` and `js` directories. It uses the [Three.js](https://threejs.org/) library to render the 3D scene.
2.  **WASM Application**: The `wasm-app` crate acts as the main bridge between the Rust logic and the JavaScript frontend. It's compiled to WebAssembly (WASM) and exposes the necessary functions to be called from JS.
3.  **Core Logic Crates**: The core business logic of the application is organized into a Cargo workspace with several independent crates, each responsible for a specific domain:
    *   `mesh`: Handles 3D mesh data structures and manipulation.
    *   `physics`: Runs the soft-body physics simulation.
    *   `image-processing`: Provides image filtering effects.
    *   `face-detection`: (Optional feature) Detects faces in images.

The following diagram illustrates the interaction between these components:

```mermaid
graph TD;
    subgraph Browser
        A[Frontend (js/main.js)]
    end

    subgraph "Rust Workspace (crates/)"
        B[wasm-app]
        C[physics]
        D[mesh]
        E[image-processing]
        F[face-detection]
    end

    A -- Calls WASM functions --> B;
    B -- Uses --> C;
    B -- Uses --> D;
    B -- Uses --> E;
    B -- Uses --> F;
    C -- Uses --> D;
```

## Project Layout

The repository is structured as a Cargo workspace to promote modularity and separation of concerns.

```
.
├── Cargo.toml          # The root workspace manifest
├── crates/             # Contains all the Rust crates
│   ├── wasm-app/       # The main WASM library crate and JS interface
│   ├── physics/        # Physics simulation logic
│   ├── mesh/           # Mesh manipulation logic
│   ├── image-processing/ # Image processing filters
│   └── face-detection/ # Face detection logic (optional feature)
├── js/                 # JavaScript source files
│   └── main.js
├── static/             # Static assets for the web application
│   ├── index.html
│   └── assets/
├── package.json
├── trunk.toml          # Trunk configuration file
...
```

## Build System and Bundler

**Decision**: We will use [Trunk](https://trunkrs.dev/) as the build system and bundler for this project.

**Justification**:
- **Simplicity**: Trunk provides a simple, zero-config experience for building and bundling Rust Wasm applications.
- **Developer Experience**: It offers a fast development server with auto-reloading, which improves developer productivity.
- **Integration**: Trunk seamlessly handles the integration between Rust and JavaScript.

## Rendering Architecture

**Decision**: We will use a two-stack architecture with Rust/Wasm for application logic and Three.js for rendering.

**Justification**:
- **Performance**: Three.js is a highly optimized library for 3D graphics in the browser.
- **Ecosystem**: It has a large and active community and extensive documentation.
- **Productivity**: Using a mature rendering library like Three.js allows us to focus on the application logic in Rust.

## Physics Engine

**Decision**: We will use a custom spring-mass system for the physics simulation.

**Justification**:
- **Simplicity**: A spring-mass system is a relatively simple model for the deformable face effect.
- **Performance**: This model is computationally inexpensive and suitable for real-time applications.
- **Control**: A custom implementation gives us full control over the physics parameters.
