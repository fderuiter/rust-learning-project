# Phase 5: Implementation Breakdown

The following plan expands each task from `phase_5.md` into concrete implementation steps with completion checkboxes, tests, and documentation updates.

## 5.1 Testing Strategy Foundation

### 5.1.1 Document Testing Approach
- **Implementation**
  - [x] Create `docs/testing.md` outlining the two-tier strategy and naming conventions for test files.
  - [x] Add a section to `README.md` summarizing how to run native and Wasm tests.
- **Tests**
  - [x] Ensure `docs/testing.md` includes verified command snippets for both tiers.
- **Documentation**
  - [x] Cross-link `docs/testing.md` from `CONTRIBUTING.md`.

### 5.1.2 Configure Test Tooling
- **Implementation**
  - [x] Add `wasm-bindgen-test` and `wasm-pack` as dev-dependencies in `Cargo.toml`.
  - [x] Install `wasm-pack` via `cargo install wasm-pack` for local development.
  - [x] Set up a `Makefile` target `test-all` that runs `cargo test` followed by `wasm-pack test`.
- **Tests**
  - [x] Run `wasm-pack --version` to verify installation.
  - [x] Execute `make test-all` locally to ensure both test suites run sequentially.
- **Documentation**
  - [x] Document the `make test-all` workflow in `docs/testing.md`.

### 5.1.3 Continuous Integration Pipeline
- **Implementation**
  - [x] Extend CI configuration to execute `cargo test` on all pushes.
  - [x] Add a separate CI job invoking `wasm-pack test --headless --firefox`.
  - [x] Configure caching for the cargo registry and build artifacts to speed up CI.
- **Tests**
  - [x] Confirm that both CI jobs pass on a test pull request.
- **Documentation**
  - [x] Note CI environments, browsers used, and caching strategy in `docs/testing.md`.

## 5.2 Tier 1: Native Rust Unit Tests

### 5.2.1 Organize Unit Test Modules
- **Implementation**
  - [x] For each core module, create an inline `mod tests` section with focused test functions.
  - [x] Use helper functions or fixtures for shared setup code.
- **Tests**
  - [x] Run `cargo test` and ensure all modules compile and execute their tests.
- **Documentation**
  - [x] Document patterns for arranging unit tests and fixtures in `docs/testing.md`.

### 5.2.2 Property and Edge-Case Testing
- **Implementation**
  - [x] Add `proptest` as a dev-dependency for property-based tests.
  - [x] Write tests exploring edge cases for spring forces, integration limits, and vector math.
- **Tests**
  - [x] Execute `cargo test` with the property tests enabled and observe failures for invalid invariants.
- **Documentation**
  - [x] Explain when to use property tests vs. example-based tests in `docs/testing.md`.

### 5.2.3 Code Coverage Metrics
- **Implementation**
  - [x] Integrate `cargo tarpaulin` for Linux-based coverage reports.
  - [x] Add a `make coverage` target that runs `cargo tarpaulin --ignore-tests`.
- **Tests**
  - [x] Run `make coverage` and verify that coverage exceeds the 90% threshold for physics modules.
- **Documentation**
  - [x] Record coverage commands and badge setup instructions in `docs/testing.md`.

## 5.3 Tier 2: Wasm Integration and E2E Tests

### 5.3.1 Set Up wasm-bindgen-test Harness
- **Implementation**
  - [x] Create test files under `tests/` using the `#[wasm_bindgen_test]` macro.
  - [x] Export public functions from `FaceController` for testing via `wasm_bindgen`.
- **Tests**
  - [x] Run `wasm-pack test --headless --firefox` to execute the harness.
- **Documentation**
  - [x] Document the harness setup and example tests in `docs/testing.md`.

### 5.3.2 Browser End-to-End Interaction Tests
- **Implementation**
  - [ ] Choose a browser automation framework such as Playwright.
  - [ ] Write a script that loads the Web page, injects a mesh, performs a drag interaction, and verifies vertex movement.
- **Tests**
  - [ ] Execute the automation script in CI using `wasm-pack test --headless` combined with the framework's CLI.
  - [ ] Ensure the script captures screenshots or console logs on failure.
- **Documentation**
  - [ ] Add an end-to-end testing section in `docs/testing.md` with instructions for running the automation locally.

### 5.3.3 Cross-Browser Validation
- **Implementation**
  - [ ] Configure `wasm-pack test` to run against both Firefox and Chrome in CI.
  - [ ] Investigate any discrepancies in behavior and add polyfills if necessary.
- **Tests**
  - [ ] Verify that both `wasm-pack test --headless --firefox` and `--chrome` pass.
- **Documentation**
  - [ ] Document supported browsers and known issues in `docs/testing.md`.

### 5.3.4 Performance and Memory Profiling
- **Implementation**
  - [ ] Use browser devtools to record memory usage and frame rates during automated tests.
  - [ ] Set thresholds for acceptable performance metrics.
- **Tests**
  - [ ] Include assertions in the automation script ensuring frame rate stays above target and memory does not leak.
- **Documentation**
  - [ ] Record profiling methodology and thresholds in `docs/testing.md`.
