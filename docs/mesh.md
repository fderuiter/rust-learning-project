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
