use nalgebra::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub old_position: Vector3<f32>,
    pub acceleration: Vector3<f32>,
    pub mass: f32,
}

impl Vertex {
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

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
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
