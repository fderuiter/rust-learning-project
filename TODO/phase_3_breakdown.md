# Phase 3: Implementation Breakdown

The following plan expands each task from `phase_3.md` into concrete implementation steps with completion checkboxes, tests, and documentation updates.

## 3.1 Scene Setup

### 3.1.1 Initialize Three.js Environment
- **Implementation**
  - [x] Create `main.js` and import required Three.js modules including `OrbitControls`.
  - [x] Instantiate `THREE.Scene`, `THREE.PerspectiveCamera`, and `THREE.WebGLRenderer` bound to `<canvas id="main-canvas">`.
  - [x] Configure renderer size to the window and append its DOM element to the document.
  - [x] Add `THREE.AmbientLight` and `THREE.DirectionalLight` to illuminate the model.
  - [x] Enable `OrbitControls` on the camera with damping for smooth interaction.
  - [x] Register a window `resize` handler to update camera aspect and renderer size.
- **Tests**
  - [x] Manual verification that an empty, lit scene appears and is rotatable when served via `trunk`.
- **Documentation**
  - [x] Document Three.js setup and control configuration in `docs/frontend.md`.

## 3.2 Mesh Loading and Initialization

### 3.2.1 Load 3D Face Model
- **Implementation**
  - [x] Import `GLTFLoader` (or similar) and load a low-polygon face model.
  - [x] On load, center and scale the mesh, then add it to the scene.
- **Tests**
  - [x] Visual test confirming the model renders at the origin with expected orientation.
- **Documentation**
  - [x] Record the model source and loading steps in `docs/frontend.md`.

### 3.2.2 Initialize Wasm Module with Mesh Data
- **Implementation**
  - [x] Extract vertex position and index arrays from the mesh's geometry.
  - [x] Pass these arrays to the Wasm `FaceController` constructor and keep the instance globally.
  - [x] Log vertex and index counts in both JavaScript and Rust for verification.
- **Tests**
  - [x] Integration test ensuring counts logged in JS match those reported by Rust.
- **Documentation**
  - [x] Document the data transfer process to Wasm in `docs/frontend.md`.

## 3.3 User Interaction and Raycasting

### 3.3.1 Implement Mouse Event Listeners
- **Implementation**
  - [x] Attach `mousedown`, `mousemove`, and `mouseup` listeners to the canvas.
  - [x] Track the selected vertex ID and whether the user is currently dragging.
- **Tests**
  - [x] Manual test confirming drag state toggles correctly on mouse events.
- **Documentation**
  - [x] Describe the mouse event flow in `docs/frontend.md`.

### 3.3.2 Implement Raycasting for Vertex Selection
- **Implementation**
  - [x] On `mousedown`, convert screen coordinates to normalized device coordinates.
  - [x] Use `THREE.Raycaster` to intersect the face mesh and obtain the closest hit.
  - [x] Determine the nearest vertex index to the intersection point.
  - [x] Call `faceController.on_mouse_down(vertex_id, ...)` and visually highlight the selection.
- **Tests**
  - [x] Manual test selecting multiple vertices to ensure correct indices are reported.
- **Documentation**
  - [x] Update `docs/frontend.md` with raycasting math and selection visuals.

## 3.4 The Render Loop

### 3.4.1 Structure the `requestAnimationFrame` Loop
- **Implementation**
  - [x] Implement an `animate(time)` function that calls `requestAnimationFrame(animate)`.
  - [x] Compute `deltaTime` between frames for physics updates.
- **Tests**
  - [x] Console log `deltaTime` during development to verify timing values.
- **Documentation**
  - [x] Document loop structure and timing calculations in `docs/frontend.md`.

### 3.4.2 Implement Frame Logic
- **Implementation**
  - [x] If dragging, compute the new 3D target position and call `faceController.on_mouse_move`.
  - [x] Advance physics each frame via `faceController.tick(deltaTime)`.
  - [x] Retrieve updated vertex data with `faceController.get_vertex_buffer_ptr()`.
  - [x] Map `wasmModule.memory.buffer` into a `Float32Array` and copy into the mesh's position attribute.
  - [x] Mark `mesh.geometry.attributes.position.needsUpdate = true`.
  - [x] Render the scene with `renderer.render(scene, camera)`.
- **Tests**
  - [x] Manual test: drag a handle and observe mesh deformation updating each frame.
- **Documentation**
  - [x] Document data flow from Wasm to Three.js in `docs/frontend.md`.
