# Physics Engine

This document describes the physics engine used to simulate the soft-body dynamics of the mesh.

## `Spring`

The `Spring` struct represents a connection between two vertices in the mesh.

### Fields

- **`vertex_a_index: usize`**: The index of the first vertex in the spring.
- **`vertex_b_index: usize`**: The index of the second vertex in the spring.
- **`rest_length: f32`**: The length of the spring when it is at rest.
- **`stiffness: f32`**: The stiffness of the spring. This determines how much force the spring exerts when it is stretched or compressed.
- **`damping: f32`**: The damping factor of the spring. This is used to reduce oscillations and make the simulation more stable.

## `Physics`

The `Physics` struct encapsulates the state and logic of the physics simulation.

### Fields

- **`springs: Vec<Spring>`**: A vector of all the springs in the simulation.
- **`time_step: f32`**: The time step for the simulation, in seconds.
- **`gravity: Vector3<f32>`**: The gravity vector.

### Methods

#### `update(mesh: &mut Mesh)`

This method advances the physics simulation by one time step. It performs the following steps:
1.  Applies gravity to all vertices.
2.  Calculates and applies the forces from all the springs.
3.  Updates the position of each vertex using Verlet integration.

## Physics Equations

### Hooke's Law

The spring force is calculated using Hooke's Law:

`F = -k * x`

where:
- `F` is the force exerted by the spring.
- `k` is the spring constant (stiffness).
- `x` is the displacement of the spring from its rest length.

### Damping

A damping force is also applied to reduce oscillations:

`F_damping = -b * v`

where:
- `F_damping` is the damping force.
- `b` is the damping coefficient.
- `v` is the relative velocity of the two vertices connected by the spring.

### Verlet Integration

The position of each vertex is updated using Verlet integration, which is a numerical method for integrating Newton's equations of motion. It is a simple and stable method that is well-suited for this type of simulation.
