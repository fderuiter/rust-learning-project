pub mod mesh;
pub mod physics;
pub mod face_detection;
pub mod image_processing;

use crate::mesh::Mesh;
use crate::physics::Physics;
use wasm_bindgen::prelude::*;

use serde::Serialize;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct BBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub prob: f32,
}

#[wasm_bindgen]
pub fn detect_faces(image_bytes: &[u8]) -> Result<JsValue, JsValue> {
    let bboxes = face_detection::detect_faces(image_bytes)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let result: Vec<BBox> = bboxes
        .into_iter()
        .map(|bbox| BBox {
            x1: bbox.x1,
            y1: bbox.y1,
            x2: bbox.x2,
            y2: bbox.y2,
            prob: bbox.prob,
        })
        .collect();
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub struct FaceController {
    mesh: Mesh,
    physics: Physics,
    vertex_positions: Vec<f32>,
    dragged_vertex_index: Option<u32>,
}

#[wasm_bindgen]
impl FaceController {
    #[wasm_bindgen(constructor)]
    pub fn new(positions: &[f32], indices: &[u32]) -> FaceController {
        // In a real application, we'd propagate this error to the JS caller.
        // Constructors in wasm-bindgen can't return a Result, so we'll panic.
        let mesh = Mesh::new(positions, indices).unwrap();
        let mut physics = Physics::new();
        physics.init_springs(&mesh);
        let vertex_positions = mesh.get_vertex_positions_flat();

        FaceController {
            mesh,
            physics,
            vertex_positions,
            dragged_vertex_index: None,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.physics.time_step = dt;
        self.physics.update(&mut self.mesh);
        self.vertex_positions = self.mesh.get_vertex_positions_flat();
    }

    pub fn on_mouse_down(&mut self, vertex_id: u32, x: f32, y: f32, z: f32) {
        self.dragged_vertex_index = Some(vertex_id);
        self.mesh.vertices[vertex_id as usize].position.x = x;
        self.mesh.vertices[vertex_id as usize].position.y = y;
        self.mesh.vertices[vertex_id as usize].position.z = z;
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32, z: f32) {
        if let Some(vertex_id) = self.dragged_vertex_index {
            self.mesh.vertices[vertex_id as usize].position.x = x;
            self.mesh.vertices[vertex_id as usize].position.y = y;
            self.mesh.vertices[vertex_id as usize].position.z = z;
        }
    }

    pub fn on_mouse_up(&mut self) {
        self.dragged_vertex_index = None;
    }

    pub fn get_vertex_buffer_ptr(&self) -> *const f32 {
        self.vertex_positions.as_ptr()
    }

    pub fn get_vertex_count(&self) -> usize {
        self.mesh.vertices.len()
    }
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
