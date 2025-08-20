# Testing Strategy

This project employs a two-tier testing strategy to ensure correctness, robustness, and stability, from core Rust logic to browser-based interactions.

## Tier 1: Native Rust Tests (`cargo test`)

Tier 1 focuses on testing pure Rust logic in a native environment. These tests are fast, can be run frequently during development, and are ideal for validating algorithms, data structures, and business logic.

### Unit Tests

-   **Location**: Defined in a `mod tests { ... }` block at the bottom of each source file (e.g., `src/physics.rs`).
-   **Purpose**: To test individual functions and components in isolation. They have access to private functions and types within their parent module.
-   **Convention**: Test functions are annotated with `#[test]`.

### Integration Tests

-   **Location**: Stored in the `/tests` directory (e.g., `tests/physics.rs`).
-   **Purpose**: To test the public API of the crate as a whole. Each file in `/tests` is compiled as a separate crate, ensuring that only public items are tested.
-   **Convention**: Test functions are annotated with `#[test]`.

### How to Run

All native Rust tests can be executed with a single command:

```bash
cargo test --all-features --workspace
```

## Tier 2: WebAssembly & Browser Tests (`wasm-pack test`)

Tier 2 tests validate the WebAssembly (Wasm) interface and its integration with the browser environment. These are crucial for ensuring that the Rust code interoperates correctly with JavaScript and browser APIs.

### Wasm Integration Tests

-   **Location**: Stored in the `/tests` directory, typically in files like `tests/wasm.rs`.
-   **Purpose**: To test functions exposed to JavaScript via `wasm-bindgen`. These tests run in a real browser environment (headless or headed), allowing for checks on JS/Wasm data marshalling and behavior.
-   **Framework**: We use `wasm-bindgen-test`, which provides the `#[wasm_bindgen_test]` macro.

### How to Run

Wasm tests are executed using `wasm-pack`:

```bash
# Run tests in headless Firefox (default)
wasm-pack test --headless

# Run tests in headless Chrome
wasm-pack test --headless --chrome
```

## End-to-End (E2E) and Visual Regression Tests

*This section is a placeholder for future E2E test implementation using a framework like Playwright or Cypress. E2E tests will simulate full user workflows, such as uploading an image, dragging a vertex, and verifying the visual outcome.*
