use nalgebra::Vector3;
use wasm_bindgen::JsValue;

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
    pub fn new(positions: &[f32], indices: &[u32]) -> Result<Mesh, JsValue> {
        if positions.len() % 3 != 0 {
            return Err(JsValue::from_str("Invalid positions length"));
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
