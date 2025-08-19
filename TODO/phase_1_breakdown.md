# Phase 1: Implementation Breakdown

The following plan expands each task from `phase_1.md` into concrete implementation steps with required tests and documentation updates.

## 1.1 Environment and Toolchain Configuration

### 1.1.1 Install Rust Toolchain
- **Implementation**
  - [x] Install rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
  - [x] Create `rust-toolchain.toml` specifying the required Rust channel.
  - [x] Reload the shell so `cargo` and `rustc` are on `PATH`.
- **Tests**
  - [x] Run `rustc --version` and `cargo --version` to confirm versions match `rust-toolchain.toml`.
- **Documentation**
  - [x] Record installation steps and pinned Rust version in `docs/setup.md`.

### 1.1.2 Install WebAssembly Build Target
- **Implementation**
  - [x] Add the target: `rustup target add wasm32-unknown-unknown`.
- **Tests**
  - [x] Verify with `rustup target list --installed | grep wasm32-unknown-unknown`.
- **Documentation**
  - [x] Note the added target and verification command in `docs/setup.md`.

### 1.1.3 Establish Code Repository and Version Control
- **Implementation**
  - [x] Initialize git in the project root and create the `main` branch.
  - [x] Create `.gitignore` excluding build artifacts and editor files.
  - [x] Define a branching strategy such as GitFlow.
  - [x] Configure CI to run `cargo fmt -- --check` and `cargo test` on every pull request.
- **Tests**
  - [x] Ensure CI runs `cargo fmt -- --check` and `cargo test` successfully on an initial pull request.
- **Documentation**
  - [x] Update `CONTRIBUTING.md` with branching strategy, commit message style, and CI expectations.

## 1.2 Build System and Bundler Selection

### 1.2.1 Analysis of Bundler Options
- **Implementation**
  - [x] Build a wasm-pack prototype noting JavaScript integration steps.
  - [x] Build a Trunk prototype using `trunk serve` and record build speed and developer experience.
- **Tests**
  - [x] Confirm each prototype compiles and serves a basic page without errors.
- **Documentation**
  - [x] Summarize prototype findings in `ARCHITECTURE.md`.

### 1.2.2 Architectural Decision and Justification
- **Implementation**
  - [x] Select Trunk as the bundler based on prototype results.
- **Tests**
  - [x] Smoke-test `trunk serve` to ensure the build pipeline works end to end.
- **Documentation**
  - [x] Record the decision and reasoning in `ARCHITECTURE.md`.

### 1.2.3 Initial Trunk Configuration
- **Implementation**
  - [x] Install tools: `cargo install trunk wasm-bindgen-cli`.
  - [x] Create `index.html` with a `<canvas>` element and `<link data-trunk rel="rust" />` tag.
  - [x] Run the development server with `trunk serve`.
- **Tests**
  - [x] Visit `http://localhost:8080` and confirm a blank page is served without console errors.
- **Documentation**
  - [x] Add Trunk setup instructions to `README.md`.

## 1.3 Rendering Architecture Selection

### 1.3.1 Analysis of Rendering Options
- **Implementation**
  - [x] Prototype Option A: Rust/Wasm for logic with Three.js for rendering.
  - [x] Prototype Option B: Pure Rust with `web-sys` WebGL calls.
- **Tests**
  - [x] Ensure both prototypes compile and render a simple object in the browser.
- **Documentation**
  - [x] Record performance metrics and trade-offs in `ARCHITECTURE.md`.

### 1.3.2 Architectural Decision and Justification
- **Implementation**
  - [x] Choose the two-stack architecture (Rust logic + Three.js rendering).
- **Tests**
  - [x] Verify a sample Three.js scene can call a Rust function compiled to Wasm.
- **Documentation**
  - [x] Summarize the decision and rationale in `ARCHITECTURE.md`.

### 1.3.3 Initial Project Scaffolding
- **Implementation**
  - [x] Update `index.html` with `<canvas id="main-canvas">`.
  - [x] Create `main.js` to set up a basic Three.js renderer, camera, and scene.
  - [x] Run `trunk serve` to view the empty scene.
- **Tests**
  - [x] Visually confirm the scene renders and no console errors appear.
- **Documentation**
  - [x] Document project layout and initial Three.js setup in `README.md`.

## 1.4 "Hello, Wasm!" – Initial Integration Test

### 1.4.1 Create a Simple Rust Wasm Function
- **Implementation**
  - [x] Add `wasm-bindgen = "0.2"` to `Cargo.toml`.
  - [x] Implement an `add(a: i32, b: i32) -> i32` function in `src/main.rs` annotated with `#[wasm_bindgen]`.
  - [x] Build using `trunk build` or `trunk serve`.
- **Tests**
  - [x] Write a Rust unit test asserting `add(2,3) == 5`.
  - [x] Add a wasm-bindgen test to ensure the function exports correctly.
- **Documentation**
  - [x] Include the example function and build instructions in `docs/wasm.md`.

### 1.4.2 Call the Wasm Function from JavaScript
- **Implementation**
  - [x] Import the generated module in `main.js` and log the result of `add(2,3)`.
- **Tests**
  - [x] Manual browser test verifying the expected value appears in the console.
- **Documentation**
  - [x] Document JavaScript/Wasm integration steps in `docs/wasm.md`.
