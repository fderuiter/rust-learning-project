# Phase 4: Implementation Breakdown

The following plan expands each task from `phase_4.md` into concrete implementation steps with completion checkboxes, tests, and documentation updates.

## 4.1 Face Detection for Vertex Interaction

### 4.1.1 Integrate a Face Detection Library
- **Implementation**
  - [ ] Evaluate WebAssembly-compatible face detection libraries (e.g., `rust-faces`).
  - [ ] Add the chosen library and its model files to the build process.
  - [ ] Expose a Wasm function that accepts image bytes and returns facial landmark coordinates.
  - [ ] Map landmark coordinates to nearest vertices on the 3D mesh and mark them as interactive handles.
  - [ ] Highlight mapped vertices in Three.js using small spheres or colored points.
- **Tests**
  - [ ] Unit test mapping logic with synthetic coordinates to ensure correct vertex indices are selected.
  - [ ] Integration test running detection on a sample image and confirming landmarks are returned and highlighted.
- **Documentation**
  - [ ] Document library selection, model loading, and landmark-to-vertex mapping in `docs/face_detection.md`.

### 4.1.2 Handle Image Input for Detection
- **Implementation**
  - [ ] Accept image uploads or camera input and convert data into the format required by the detector.
  - [ ] Pass image data to the Wasm detection function upon load.
- **Tests**
  - [ ] Test that image data is correctly decoded and passed to Wasm using a known image.
  - [ ] Manual test: upload an image and verify markers appear on corresponding facial features.
- **Documentation**
  - [ ] Update `docs/face_detection.md` with image input workflow and troubleshooting tips.

## 4.2 Image-Based Face Stretching

### 4.2.1 Implement Image Upload and Texture Mapping
- **Implementation**
  - [ ] Add an `<input type="file">` element to the HTML for image uploads.
  - [ ] Use JavaScript to read the image into an `ArrayBuffer`.
  - [ ] Apply the image as a texture on the 3D face model using `THREE.TextureLoader` and `MeshBasicMaterial`.
  - [ ] Store the original image data for further processing.
- **Tests**
  - [ ] Manual test uploading various image formats to ensure textures display correctly.
  - [ ] Integration test verifying texture dimensions match the mesh UV layout.
- **Documentation**
  - [ ] Document the upload UI and texture-mapping steps in `docs/frontend.md`.

### 4.2.2 Wasm-Powered Image Pre-processing
- **Implementation**
  - [ ] Add `photon-rs` as a dependency compiled to Wasm.
  - [ ] Implement a Rust function to accept raw image data and apply operations such as resizing or filters (grayscale, sepia).
  - [ ] Return the processed buffer to JavaScript to update the texture.
- **Tests**
  - [ ] Unit tests for each filter validating output dimensions and pixel values.
  - [ ] Integration test demonstrating a filter applied to a sample image and rendered on the model.
- **Documentation**
  - [ ] Create `docs/image_processing.md` detailing available filters and usage examples.

### 4.2.3 User Interface for Filter Selection
- **Implementation**
  - [ ] Add UI controls (dropdown or buttons) for choosing image filters.
  - [ ] Wire controls to call the Wasm pre-processing functions with the selected filter.
- **Tests**
  - [ ] Manual test ensuring UI actions trigger filter application and texture updates.
  - [ ] Automated test verifying the correct filter name is passed from JavaScript to Wasm.
- **Documentation**
  - [ ] Update `docs/frontend.md` with instructions for using the filter UI.

### 4.2.4 Error Handling and Fallbacks
- **Implementation**
  - [ ] Display user-friendly messages for unsupported file types or detection failures.
  - [ ] Provide a fallback texture or restore the previous texture when processing fails.
- **Tests**
  - [ ] Tests for invalid file inputs ensuring errors are surfaced without crashes.
  - [ ] Manual test simulating detection failure to verify fallback behavior.
- **Documentation**
  - [ ] Describe error handling strategy and recovery options in `README.md`.

