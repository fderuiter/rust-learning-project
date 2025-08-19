use crate::physics::{self, Physics, Spring};
use js_sys;
use nalgebra::Vector3;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

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
    #[wasm_bindgen(skip)]
    pub mass: f32,
}

#[wasm_bindgen]
pub struct Mesh {
    #[wasm_bindgen(skip)]
    pub vertices: Vec<Vertex>,
    #[wasm_bindgen(skip)]
    pub indices: Vec<u32>,
    #[wasm_bindgen(skip)]
    pub springs: Vec<Spring>,
    #[wasm_bindgen(skip)]
    pub physics: Physics,
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
                    mass: 1.0,
                }
            })
            .collect::<Vec<Vertex>>();

        let mut springs = Vec::new();
        let mut existing_springs = HashSet::new();

        for triangle in indices.chunks_exact(3) {
            let (a, b, c) = (
                triangle[0] as usize,
                triangle[1] as usize,
                triangle[2] as usize,
            );

            let edges = [(a, b), (b, c), (c, a)];
            for (v1_idx, v2_idx) in edges.iter() {
                let (v1_idx, v2_idx) = if v1_idx < v2_idx {
                    (*v1_idx, *v2_idx)
                } else {
                    (*v2_idx, *v1_idx)
                };

                if existing_springs.insert((v1_idx, v2_idx)) {
                    let rest_length =
                        (vertices[v1_idx].position - vertices[v2_idx].position).magnitude();
                    springs.push(Spring {
                        vertex_a_index: v1_idx,
                        vertex_b_index: v2_idx,
                        rest_length,
                        stiffness: 1000.0, // Default stiffness
                        damping: 10.0,     // Default damping
                    });
                }
            }
        }

        Mesh {
            vertices,
            indices: indices.to_vec(),
            springs,
            physics: Physics::new(),
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

    pub fn get_vertex_positions_flat(&self) -> Vec<f32> {
        self.vertices
            .iter()
            .flat_map(|v| v.position.iter().cloned())
            .collect()
    }

    pub fn get_indices(&self) -> js_sys::Uint32Array {
        js_sys::Uint32Array::from(&self.indices[..])
    }

    #[wasm_bindgen]
    pub fn apply_force(&mut self, vertex_index: usize, x: f32, y: f32, z: f32) {
        if let Some(vertex) = self.vertices.get_mut(vertex_index) {
            vertex.acceleration += Vector3::new(x, y, z);
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        let (time_step, gravity) = (self.physics.time_step, self.physics.gravity);
        physics::update(self, time_step, &gravity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_new() {
        let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];
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
