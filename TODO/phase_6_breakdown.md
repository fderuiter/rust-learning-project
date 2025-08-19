# Phase 6: Implementation Breakdown

The following plan expands each task from `phase_6.md` into concrete implementation steps with completion checkboxes, tests, and documentation updates.

## 6.1 Build Optimization

### 6.1.1 Enable Release Optimizations
- **Implementation**
  - [ ] Set `[profile.release]` in `Cargo.toml` with `lto = true` and `opt-level = "z"` for size-focused builds.
  - [ ] Run `trunk build --release` to generate optimized artifacts.
  - [ ] Add a `Makefile` target `build-release` invoking the above command for repeatable builds.
- **Tests**
  - [ ] Compare the size of `dist/*.wasm` between debug and release builds to confirm reduction.
  - [ ] Execute `make build-release` and ensure the command completes without errors.
- **Documentation**
  - [ ] Record release build flags and expected file sizes in `docs/optimizations.md`.
  - [ ] Reference `build-release` in the "Building" section of `README.md`.

### 6.1.2 Integrate `wasm-opt`
- **Implementation**
  - [ ] Install Binaryen’s `wasm-opt` tool (e.g., `cargo install wasm-opt` or system package manager).
  - [ ] Configure Trunk’s release pipeline to run `wasm-opt -O` on the generated `.wasm` file.
  - [ ] Include the optimization step in the `build-release` target.
- **Tests**
  - [ ] Run `wasm-opt --version` to verify the tool is available.
  - [ ] After running `make build-release`, inspect `dist/*.wasm` to ensure the optimization step executes (file size shrinks or `wasm-opt` logs appear).
- **Documentation**
  - [ ] Document installation and usage of `wasm-opt` in `docs/optimizations.md` with command examples.

### 6.1.3 Profile Performance
- **Implementation**
  - [ ] Serve the release build locally using `trunk serve --release` or a static server.
  - [ ] Use browser DevTools to capture frame render times and memory usage.
  - [ ] Adjust physics parameters or rendering settings if profiling reveals bottlenecks.
- **Tests**
  - [ ] Capture a performance profile demonstrating stable ~60 FPS on reference hardware.
  - [ ] Log memory allocations to ensure they remain within acceptable bounds.
- **Documentation**
  - [ ] Summarize profiling methodology and findings in `docs/performance.md`.

## 6.2 Deployment

### 6.2.1 Prepare `dist` for Hosting
- **Implementation**
  - [ ] Run `make build-release` to produce the final `dist/` directory.
  - [ ] Verify `index.html`, `*.wasm`, and JS glue code exist and reference one another correctly.
- **Tests**
  - [ ] Serve `dist/` locally (`npx serve dist` or similar) and confirm the app loads without console errors.
- **Documentation**
  - [ ] Document the contents and purpose of the `dist/` directory in `docs/deployment.md`.

### 6.2.2 Configure Hosting Service
- **Implementation**
  - [ ] Choose a static host (e.g., GitHub Pages, Netlify, Vercel).
  - [ ] Add deployment configuration: for GitHub Pages, create a GitHub Actions workflow that uploads `dist/`; for Netlify/Vercel, define a project pointing to `dist/`.
  - [ ] Store any necessary access tokens or secrets securely in CI settings.
- **Tests**
  - [ ] Trigger the deployment workflow and verify it completes successfully.
  - [ ] Confirm the hosted URL serves the same `dist/` contents as the local build.
- **Documentation**
  - [ ] Provide step-by-step hosting setup instructions and URLs in `docs/deployment.md`.
  - [ ] Add the production URL to `README.md`.

### 6.2.3 Validate Production Site
- **Implementation**
  - [ ] Load the public URL on multiple browsers and devices.
  - [ ] Execute a smoke test of major features: loading the mesh, interacting with vertices, and observing physics.
- **Tests**
  - [ ] Record screenshots or logs confirming cross-browser functionality.
  - [ ] Monitor network requests to ensure assets are served with correct MIME types and caching headers.
- **Documentation**
  - [ ] Append a "Verification" section to `docs/deployment.md` noting testing steps and results.

