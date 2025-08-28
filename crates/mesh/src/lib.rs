use nalgebra::Vector3;

/// Represents a single vertex in the mesh, with properties for physics simulation.
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    /// The current position of the vertex.
    pub position: Vector3<f32>,
    /// The position of the vertex in the previous time step, used for Verlet integration.
    pub old_position: Vector3<f32>,
    /// The acceleration of the vertex.
    pub acceleration: Vector3<f32>,
    /// The mass of the vertex. If mass is 0, the vertex is considered immovable.
    pub mass: f32,
}

impl Vertex {
    /// Creates a new `Vertex` at the given position.
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

/// Represents a 3D mesh, composed of vertices and indices.
pub struct Mesh {
    /// The vertices of the mesh.
    pub vertices: Vec<Vertex>,
    /// The indices that define the faces of the mesh.
    pub indices: Vec<u32>,
}

impl Mesh {
    /// Creates a new `Mesh` from a flat list of positions and a list of indices.
    pub fn new(positions: &[f32], indices: &[u32]) -> Result<Mesh, String> {
        if positions.len() % 3 != 0 {
            return Err("The length of the positions array must be a multiple of 3.".to_string());
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

    /// Returns a flat list of the positions of all vertices in the mesh.
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
