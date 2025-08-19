# Architecture

This document outlines the architectural decisions made for this project.

## Build System and Bundler

**Decision**: We will use [Trunk](https://trunkrs.dev/) as the build system and bundler for this project.

**Justification**:
- **Simplicity**: Trunk provides a simple, zero-config experience for building and bundling Rust Wasm applications.
- **Developer Experience**: It offers a fast development server with auto-reloading, which improves developer productivity.
- **Integration**: Trunk seamlessly handles the integration between Rust and JavaScript, making it easy to call Wasm functions from JS and vice-versa.

**Alternatives Considered**:
- **wasm-pack**: While a powerful tool, `wasm-pack` requires more manual configuration and setup compared to Trunk. For our project's needs, the simplicity of Trunk was preferred.

## Rendering Architecture

**Decision**: We will use a two-stack architecture with Rust/Wasm for application logic and Three.js for rendering.

**Justification**:
- **Performance**: Three.js is a highly optimized library for 3D graphics in the browser, providing excellent performance.
- **Ecosystem**: It has a large and active community, extensive documentation, and a wealth of examples and resources.
- **Productivity**: Using a mature rendering library like Three.js allows us to focus on the application logic in Rust, rather than dealing with the complexities of WebGL directly.

**Alternatives Considered**:
- **Pure Rust with `web-sys`**: While possible, this approach would require writing a lot of boilerplate code for WebGL interactions, increasing complexity and development time. It would also mean reinventing many of the features that Three.js already provides.
