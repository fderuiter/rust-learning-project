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

## Physics Engine

**Decision**: We will implement a custom spring-mass system for the physics simulation.

**Justification**:
- **Simplicity**: A spring-mass system is a relatively simple and intuitive model for simulating soft-body dynamics, which is exactly what we need for the deformable face effect.
- **Performance**: This model is computationally inexpensive, making it well-suited for real-time applications in the browser.
- **Control**: A custom implementation gives us full control over the physics parameters (stiffness, damping, etc.), allowing us to fine-tune the behavior to achieve the desired aesthetic.

**Alternatives Considered**:
- **Using a pre-built physics engine (e.g., Rapier, Box2D)**: While these engines are powerful and feature-rich, they are also more complex than what our project requires. Integrating a full physics engine would add unnecessary overhead and complexity for the simple soft-body simulation we need.
