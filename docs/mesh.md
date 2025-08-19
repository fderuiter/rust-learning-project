# Mesh Data Structures

This document describes the data structures used to represent the 3D mesh.

## `Vertex`

The `Vertex` struct represents a single point in the 3D mesh.

### Fields

- **`position: Vector3<f32>`**: The current position of the vertex in 3D space.
- **`old_position: Vector3<f32>`**: The position of the vertex at the previous time step. This is used for Verlet integration.
- **`acceleration: Vector3<f32>`**: The current acceleration of the vertex.
- **`mass: f32`**: The mass of the vertex. This is used to calculate the effect of forces on the vertex.

## `Mesh`

The `Mesh` struct represents the entire 3D mesh.

### Fields

- **`vertices: Vec<Vertex>`**: A vector of all the vertices in the mesh.
- **`indices: Vec<u32>`**: A vector of indices that define the triangles of the mesh. Each group of three indices represents a single triangle.

## Initialization

The `Mesh` is initialized with the `Mesh::new` function.

### `Mesh::new(positions: &[f32], indices: &[u32]) -> Result<Mesh, JsValue>`

- **`positions`**: A flat slice of `f32` values representing the vertex positions. The length of this slice must be a multiple of 3, as each vertex is represented by three consecutive values (x, y, z).
- **`indices`**: A slice of `u32` values representing the vertex indices that form the triangles of the mesh.

The function returns a `Result<Mesh, JsValue>`. If the length of the `positions` slice is not a multiple of 3, it returns an error.
