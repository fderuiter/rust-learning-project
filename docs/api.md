# Wasm API Reference

This document provides a reference for the WebAssembly (Wasm) API of this project.

## `FaceController`

The `FaceController` is the main entry point for interacting with the Wasm module from JavaScript. It encapsulates the mesh and physics state and provides methods for updating the simulation and handling user input.

### Constructor

#### `new FaceController(positions: Float32Array, indices: Uint32Array) -> FaceController`

Creates a new `FaceController` instance.

- **`positions`**: A flat `Float32Array` of vertex positions, where each vertex is represented by three consecutive values (x, y, z).
- **`indices`**: A `Uint32Array` of vertex indices that define the triangles of the mesh.

### Methods

#### `tick(dt: number): void`

Advances the physics simulation by a given time step.

- **`dt`**: The time step in seconds.

#### `on_mouse_down(vertex_id: number, x: number, y: number, z: number): void`

Handles the `mousedown` event. This is used to "grab" a vertex.

- **`vertex_id`**: The ID of the vertex to grab.
- **`x`, `y`, `z`**: The new position of the grabbed vertex.

#### `on_mouse_move(x: number, y: number, z: number): void`

Handles the `mousemove` event. This is used to drag the grabbed vertex.

- **`x`, `y`, `z`**: The new position of the grabbed vertex.

#### `on_mouse_up(): void`

Handles the `mouseup` event. This is used to release the grabbed vertex.

#### `get_vertex_buffer_ptr(): number`

Returns a pointer to the flat array of vertex positions. This can be used to efficiently update the vertex buffer in Three.js without copying the data.

**Returns**: A pointer to the vertex buffer.
