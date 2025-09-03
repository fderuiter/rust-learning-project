use nalgebra::Vector3;

/// Represents a single vertex in a 3D mesh.
///
/// This struct holds the state of a vertex for physics simulations,
/// including its current and previous positions, acceleration, and mass.
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    /// The current position of the vertex in 3D space.
    pub position: Vector3<f32>,
    /// The position of the vertex in the previous frame, used for Verlet integration.
    pub old_position: Vector3<f32>,
    /// The current acceleration of the vertex.
    pub acceleration: Vector3<f32>,
    /// The mass of the vertex.
    pub mass: f32,
}

impl Vertex {
    /// Creates a new `Vertex` at the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the vertex.
    /// * `y` - The y-coordinate of the vertex.
    /// * `z` - The z-coordinate of the vertex.
    ///
    /// # Returns
    ///
    /// A new `Vertex` instance.
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        let position = Vector3::new(x, y, z);
        Vertex {
            position,
            old_position: position,
            acceleration: Vector3::zeros(),
            mass: 1.0,
        }
    }
}

/// Represents a 3D mesh composed of vertices and indices.
///
/// The mesh is defined by a list of vertices and a list of indices that
/// form triangles.
pub struct Mesh {
    /// A vector of `Vertex` structs that make up the mesh.
    pub vertices: Vec<Vertex>,
    /// A vector of indices that define the triangles of the mesh.
    pub indices: Vec<u32>,
}

impl Mesh {
    /// Creates a new `Mesh` from a flat list of vertex positions and indices.
    ///
    /// # Arguments
    ///
    /// * `positions` - A slice of `f32` values, where each group of three
    ///   values represents the x, y, and z coordinates of a vertex.
    /// * `indices` - A slice of `u32` values that define the triangles of the mesh.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `Mesh`, or an error message if the
    /// length of the `positions` slice is not a multiple of 3.
    pub fn new(positions: &[f32], indices: &[u32]) -> Result<Mesh, String> {
        if positions.len() % 3 != 0 {
            return Err("Invalid positions length".to_string());
        }
        let vertices = positions
            .chunks_exact(3)
            .map(|pos| Vertex::new(pos[0], pos[1], pos[2]))
            .collect();

        Ok(Mesh {
            vertices,
            indices: indices.to_vec(),
        })
    }

    /// Returns a flattened vector of the mesh's vertex positions.
    ///
    /// This is useful for passing the vertex data to rendering APIs.
    ///
    /// # Returns
    ///
    /// A `Vec<f32>` containing the x, y, and z coordinates of each vertex
    /// in sequence.
    pub fn get_vertex_positions_flat(&self) -> Vec<f32> {
        self.vertices
            .iter()
            .flat_map(|v| v.position.iter().cloned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_new_valid() {
        let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let indices = vec![0, 1];
        let mesh = Mesh::new(&positions, &indices).unwrap();
        assert_eq!(mesh.vertices.len(), 2);
        assert_eq!(mesh.indices.len(), 2);
        assert_eq!(mesh.vertices[1].position, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_mesh_new_invalid_positions() {
        let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0]; // Invalid length
        let indices = vec![0, 1];
        let result = Mesh::new(&positions, &indices);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_vertex_positions_flat() {
        let positions = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let indices = vec![0, 1];
        let mesh = Mesh::new(&positions, &indices).unwrap();
        let flat_positions = mesh.get_vertex_positions_flat();
        assert_eq!(flat_positions, positions);
    }
}
