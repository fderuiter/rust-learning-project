use nalgebra::Vector3;
use wasm_bindgen::prelude::*;
use js_sys;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Vertex {
    #[wasm_bindgen(skip)]
    pub position: Vector3<f32>,
    #[wasm_bindgen(skip)]
    pub old_position: Vector3<f32>,
    #[wasm_bindgen(skip)]
    pub acceleration: Vector3<f32>,
    #[wasm_bindgen(skip)]
    pub resting_position: Vector3<f32>,
}

#[wasm_bindgen]
pub struct Mesh {
    #[wasm_bindgen(skip)]
    pub vertices: Vec<Vertex>,
    #[wasm_bindgen(skip)]
    pub indices: Vec<u32>,
}

#[wasm_bindgen]
impl Mesh {
    #[wasm_bindgen(constructor)]
    pub fn new(positions: &[f32], indices: &[u32]) -> Mesh {
        let vertices = positions
            .chunks_exact(3)
            .map(|pos| {
                let position = Vector3::new(pos[0], pos[1], pos[2]);
                Vertex {
                    position,
                    old_position: position,
                    acceleration: Vector3::zeros(),
                    resting_position: position,
                }
            })
            .collect();

        Mesh {
            vertices,
            indices: indices.to_vec(),
        }
    }

    pub fn get_vertex_positions(&self) -> js_sys::Float32Array {
        let positions: Vec<f32> = self
            .vertices
            .iter()
            .flat_map(|v| v.position.iter().cloned())
            .collect();
        js_sys::Float32Array::from(&positions[..])
    }

    pub fn get_indices(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(&self.indices[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_new() {
        let positions = vec![
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];
        let mesh = Mesh::new(&positions, &indices);

        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.indices.len(), 6);
        assert_eq!(mesh.vertices[0].position, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(mesh.vertices[1].position, Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(mesh.vertices[2].position, Vector3::new(1.0, 1.0, 0.0));
        assert_eq!(mesh.vertices[3].position, Vector3::new(0.0, 1.0, 0.0));
    }
}
