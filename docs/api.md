# Wasm API Reference

This document provides a reference for the WebAssembly (Wasm) API of this project.

## `detect_faces`

Detects faces in an image and returns a list of bounding boxes.

### Signature

`detect_faces(image_bytes: Uint8Array) -> Promise<Array<BBox>>`

### Parameters

- **`image_bytes`**: A `Uint8Array` of the image data.

### Returns

A `Promise` that resolves to an array of `BBox` objects, each representing a detected face. The promise will reject if there is an error during face detection.

### `BBox` Object

The `BBox` object has the following properties:

- **`x1`, `y1`**: The coordinates of the top-left corner of the bounding box.
- **`x2`, `y2`**: The coordinates of the bottom-right corner of the bounding box.
- **`prob`**: The probability that the detected object is a face.

### Example

```javascript
import init, { detect_faces } from './rust_learning_project.js';

async function run() {
  await init();
  const response = await fetch('assets/test_face.jpg');
  const image_data = await response.arrayBuffer();
  const image_bytes = new Uint8Array(image_data);
  const bboxes = await detect_faces(image_bytes);
  console.log(bboxes);
}

run();
```

## `FaceController`

The `FaceController` is the main entry point for interacting with the Wasm module from JavaScript. It encapsulates the mesh and physics state and provides methods for updating the simulation and handling user input.

### Constructor

#### `new FaceController(positions: Float32Array, indices: Uint32Array) -> FaceController`

Creates a new `FaceController` instance.

- **`positions`**: A flat `Float32Array` of vertex positions, where each vertex is represented by three consecutive values (x, y, z). The coordinate system is the same as the one used by Three.js.
- **`indices`**: A `Uint32Array` of vertex indices that define the triangles of the mesh.

**Note:** This constructor will panic if the length of the `positions` array is not a multiple of 3.

### Methods

#### `tick(dt: number): void`

Advances the physics simulation by a given time step.

- **`dt`**: The time step in seconds.

#### `on_mouse_down(vertex_id: number, x: number, y: number, z: number): void`

Handles the `mousedown` event. This is used to "grab" a vertex.

- **`vertex_id`**: The ID of the vertex to grab.
- **`x`, `y`, `z`**: The new position of the grabbed vertex in the Three.js coordinate system.

#### `on_mouse_move(x: number, y: number, z: number): void`

Handles the `mousemove` event. This is used to drag the grabbed vertex.

- **`x`, `y`, `z`**: The new position of the grabbed vertex in the Three.js coordinate system.

#### `on_mouse_up(): void`

Handles the `mouseup` event. This is used to release the grabbed vertex.

#### `get_vertex_buffer_ptr(): number`

Returns a pointer to the flat array of vertex positions. This can be used to efficiently update the vertex buffer in Three.js without copying the data.

**Returns**: A pointer to the vertex buffer.

#### `get_vertex_count(): number`

Returns the number of vertices in the mesh.

**Returns**: The number of vertices.

## `add` (for demonstration)

A simple function that adds two numbers. This is included for demonstration and testing purposes.

### Signature

`add(a: number, b: number) -> number`

### Example

```javascript
import init, { add } from './rust_learning_project.js';

async function run() {
  await init();
  const result = add(2, 3);
  console.log(`2 + 3 = ${result}`);
}

run();
```
