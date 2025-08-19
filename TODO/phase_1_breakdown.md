# Phase 1: Implementation Breakdown

The following plan expands each task from `phase_1.md` into concrete implementation steps with required tests and documentation updates.

## 1.1 Environment and Toolchain Configuration

### 1.1.1 Install Rust Toolchain
- **Implementation**
  - [ ] Install rustup: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
  - [ ] Create `rust-toolchain.toml` specifying the required Rust channel.
  - [ ] Reload the shell so `cargo` and `rustc` are on `PATH`.
- **Tests**
  - [ ] Run `rustc --version` and `cargo --version` to confirm versions match `rust-toolchain.toml`.
- **Documentation**
  - [ ] Record installation steps and pinned Rust version in `docs/setup.md`.

### 1.1.2 Install WebAssembly Build Target
- **Implementation**
  - [ ] Add the target: `rustup target add wasm32-unknown-unknown`.
- **Tests**
  - [ ] Verify with `rustup target list --installed | grep wasm32-unknown-unknown`.
- **Documentation**
  - [ ] Note the added target and verification command in `docs/setup.md`.

### 1.1.3 Establish Code Repository and Version Control
- **Implementation**
  - [ ] Initialize git in the project root and create the `main` branch.
  - [ ] Create `.gitignore` excluding build artifacts and editor files.
  - [ ] Define a branching strategy such as GitFlow.
  - [ ] Configure CI to run `cargo fmt -- --check` and `cargo test` on every pull request.
- **Tests**
  - [ ] Ensure CI runs `cargo fmt -- --check` and `cargo test` successfully on an initial pull request.
- **Documentation**
  - [ ] Update `CONTRIBUTING.md` with branching strategy, commit message style, and CI expectations.

## 1.2 Build System and Bundler Selection

### 1.2.1 Analysis of Bundler Options
- **Implementation**
  - [ ] Build a wasm-pack prototype noting JavaScript integration steps.
  - [ ] Build a Trunk prototype using `trunk serve` and record build speed and developer experience.
- **Tests**
  - [ ] Confirm each prototype compiles and serves a basic page without errors.
- **Documentation**
  - [ ] Summarize prototype findings in `ARCHITECTURE.md`.

### 1.2.2 Architectural Decision and Justification
- **Implementation**
  - [ ] Select Trunk as the bundler based on prototype results.
- **Tests**
  - [ ] Smoke-test `trunk serve` to ensure the build pipeline works end to end.
- **Documentation**
  - [ ] Record the decision and reasoning in `ARCHITECTURE.md`.

### 1.2.3 Initial Trunk Configuration
- **Implementation**
  - [ ] Install tools: `cargo install trunk wasm-bindgen-cli`.
  - [ ] Create `index.html` with a `<canvas>` element and `<link data-trunk rel="rust" />` tag.
  - [ ] Run the development server with `trunk serve`.
- **Tests**
  - [ ] Visit `http://localhost:8080` and confirm a blank page is served without console errors.
- **Documentation**
  - [ ] Add Trunk setup instructions to `README.md`.

## 1.3 Rendering Architecture Selection

### 1.3.1 Analysis of Rendering Options
- **Implementation**
  - [ ] Prototype Option A: Rust/Wasm for logic with Three.js for rendering.
  - [ ] Prototype Option B: Pure Rust with `web-sys` WebGL calls.
  - [ ] Measure performance and complexity of both prototypes.
- **Tests**
  - [ ] Ensure both prototypes compile and render a simple object in the browser.
- **Documentation**
  - [ ] Record performance metrics and trade-offs in `ARCHITECTURE.md`.

### 1.3.2 Architectural Decision and Justification
- **Implementation**
  - [ ] Choose the two-stack architecture (Rust logic + Three.js rendering).
- **Tests**
  - [ ] Verify a sample Three.js scene can call a Rust function compiled to Wasm.
- **Documentation**
  - [ ] Summarize the decision and rationale in `ARCHITECTURE.md`.

### 1.3.3 Initial Project Scaffolding
- **Implementation**
  - [ ] Update `index.html` with `<canvas id="main-canvas">`.
  - [ ] Create `main.js` to set up a basic Three.js renderer, camera, and scene.
  - [ ] Run `trunk serve` to view the empty scene.
- **Tests**
  - [ ] Visually confirm the scene renders and no console errors appear.
- **Documentation**
  - [ ] Document project layout and initial Three.js setup in `README.md`.

## 1.4 "Hello, Wasm!" – Initial Integration Test

### 1.4.1 Create a Simple Rust Wasm Function
- **Implementation**
  - [ ] Add `wasm-bindgen = "0.2"` to `Cargo.toml`.
  - [ ] Implement an `add(a: i32, b: i32) -> i32` function in `src/main.rs` annotated with `#[wasm_bindgen]`.
  - [ ] Build using `trunk build` or `trunk serve`.
- **Tests**
  - [ ] Write a Rust unit test asserting `add(2,3) == 5`.
  - [ ] Add a wasm-bindgen test to ensure the function exports correctly.
- **Documentation**
  - [ ] Include the example function and build instructions in `docs/wasm.md`.

### 1.4.2 Call the Wasm Function from JavaScript
- **Implementation**
  - [ ] Import the generated module in `main.js` and log the result of `add(2,3)`.
- **Tests**
  - [ ] Manual browser test verifying the expected value appears in the console.
- **Documentation**
  - [ ] Document JavaScript/Wasm integration steps in `docs/wasm.md`.

