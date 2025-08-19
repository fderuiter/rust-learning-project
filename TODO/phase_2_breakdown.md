# Phase 2: Implementation Breakdown

The following plan expands each task from `phase_2.md` into concrete implementation steps with completion checkboxes, tests, and documentation updates.

## 2.1 Data Structures for Mesh Representation

### 2.1.1 Define Vertex and Mesh Structs
- **Implementation**
- [x] Introduce a small vector math module or dependency for 3D operations.
- [x] Create a `Vertex` struct with current, previous, and resting positions plus acceleration.
- [x] Create a `Mesh` struct owning `Vec<Vertex>` and connectivity data.
- **Tests**
  - [x] Write unit tests covering vector addition, subtraction, and normalization.
- **Documentation**
  - [ ] Describe the `Vertex` layout and `Mesh` ownership model in `docs/mesh.md`.

### 2.1.2 Implement Mesh Initialization
- **Implementation**
  - [x] Expose an `init_mesh(vertices: &[f32], indices: &[u32])` function with `#[wasm_bindgen]`.
  - [x] Parse flat arrays into internal `Mesh` and `Vertex` collections.
  - [x] Handle invalid input lengths and return meaningful errors.
- **Tests**
  - [ ] wasm-bindgen test passing a cube mesh and verifying vertex/face counts and error handling.
- **Documentation**
  - [ ] Document the initialization API and expected array formats in `docs/mesh.md`.

## 2.2 Physics Engine Implementation

### 2.2.1 Architectural Decision: Custom Physics Model
- **Implementation**
  - [x] Finalize constants for spring stiffness and damping after experimentation.
- **Tests**
  - [x] Benchmark different constants and compare results to choose stable values.
- **Documentation**
  - [x] Record rationale for a bespoke spring-mass system and chosen constants in `ARCHITECTURE.md`.

### 2.2.2 Implement Spring-Mass System

#### 2.2.2.1 Define Spring Constraints
- **Implementation**
  - [x] Create a `Spring` struct linking two vertex indices with stiffness `k` and damping `b`.
  - [x] Implement force calculation using Hooke’s Law with a damping term.
- **Tests**
  - [x] Unit test forces on a two-node system for correct restoring and damping behavior.
- **Documentation**
  - [ ] Document the spring constraint equations in `docs/physics.md`.

#### 2.2.2.2 Implement Numerical Integration
- **Implementation**
  - [x] Implement Verlet integration to update vertex positions each tick.
  - [x] Apply accumulated spring and external forces to compute acceleration.
- **Tests**
  - [x] Write tests verifying motion under constant force and oscillation in a simple spring.
- **Documentation**
  - [ ] Explain the integration scheme and update `docs/physics.md`.

## 2.3 Wasm API Design and Implementation

### 2.3.1 Define the Public Wasm Struct
- **Implementation**
  - [x] Create a `FaceController` struct annotated with `#[wasm_bindgen]` encapsulating the mesh and physics state.
  - [x] Implement a constructor that builds the mesh from provided arrays.
- **Tests**
  - [x] Integration test ensuring JavaScript can instantiate the controller.
- **Documentation**
  - [x] Document `FaceController` fields and constructor usage in `docs/api.md`.

### 2.3.2 Implement State Update and I/O Functions
- **Implementation**
  - [x] Add `tick(dt: f32)` to advance the physics simulation.
  - [x] Add mouse interaction handlers: `on_mouse_down`, `on_mouse_move`, and `on_mouse_up`.
  - [x] Implement `get_vertex_buffer_ptr() -> *const f32` returning a pointer to the vertex buffer.
- **Tests**
  - [x] Integration tests verifying state updates and that the returned pointer is non-null.
- **Documentation**
  - [ ] Update `docs/api.md` with function descriptions and usage examples.

