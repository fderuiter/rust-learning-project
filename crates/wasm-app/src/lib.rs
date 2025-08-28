use wasm_bindgen::prelude::*;

// Import crates
use mesh::Mesh;
use physics::Physics;
use image_processing;
#[cfg(not(target_arch = "wasm32"))]
use face_detection;

/// Detects faces in an image and returns a list of bounding boxes.
/// This function is only available in non-Wasm builds.
#[wasm_bindgen]
#[cfg(not(target_arch = "wasm32"))]
pub fn detect_faces(image_bytes: &[u8]) -> Result<JsValue, JsValue> {
    let bboxes =
        face_detection::detect_faces(image_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&bboxes).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// The main controller for the face-stretching simulation.
#[wasm_bindgen]
pub struct FaceController {
    mesh: Mesh,
    physics: Physics,
    vertex_positions: Vec<f32>,
    dragged_vertex_index: Option<u32>,
}

#[wasm_bindgen]
impl FaceController {
    /// Creates a new `FaceController`.
    #[wasm_bindgen(constructor)]
    pub fn new(positions: &[f32], indices: &[u32]) -> FaceController {
        let mesh = Mesh::new(positions, indices).expect("Failed to create mesh");
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

    /// Advances the physics simulation by one time step.
    pub fn tick(&mut self, dt: f32) {
        self.physics.time_step = dt;
        self.physics.update(
            &mut self.mesh,
            self.dragged_vertex_index.map(|i| i as usize),
        );
        self.vertex_positions = self.mesh.get_vertex_positions_flat();
    }

    /// Called when the user presses the mouse button.
    pub fn on_mouse_down(&mut self, vertex_id: u32, x: f32, y: f32, z: f32) {
        self.dragged_vertex_index = Some(vertex_id);
        self.mesh.vertices[vertex_id as usize].position.x = x;
        self.mesh.vertices[vertex_id as usize].position.y = y;
        self.mesh.vertices[vertex_id as usize].position.z = z;
    }

    /// Called when the user moves the mouse.
    pub fn on_mouse_move(&mut self, x: f32, y: f32, z: f32) {
        if let Some(vertex_id) = self.dragged_vertex_index {
            self.mesh.vertices[vertex_id as usize].position.x = x;
            self.mesh.vertices[vertex_id as usize].position.y = y;
            self.mesh.vertices[vertex_id as usize].position.z = z;
        }
    }

    /// Called when the user releases the mouse button.
    pub fn on_mouse_up(&mut self) {
        self.dragged_vertex_index = None;
    }

    /// Returns a pointer to the vertex buffer.
    pub fn get_vertex_buffer_ptr(&self) -> *const f32 {
        self.vertex_positions.as_ptr()
    }

    /// Returns the number of vertices in the mesh.
    pub fn get_vertex_count(&self) -> usize {
        self.mesh.vertices.len()
    }
}

// Re-exporting the image processing functions from the image-processing crate
/// Applies a grayscale filter to an image.
#[wasm_bindgen]
pub fn apply_grayscale(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    image_processing::apply_grayscale(image_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Applies a sepia filter to an image.
#[wasm_bindgen]
pub fn apply_sepia(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    image_processing::apply_sepia(image_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// A simple function to add two numbers, used for testing.
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

    #[test]
    fn test_face_controller_new() {
        let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let indices = vec![0, 1];
        let controller = FaceController::new(&positions, &indices);
        assert_eq!(controller.get_vertex_count(), 2);
    }

    #[test]
    fn test_face_controller_simulation() {
        let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let indices = vec![0, 1];
        let mut controller = FaceController::new(&positions, &indices);

        // Drag a vertex
        controller.on_mouse_down(0, 0.1, 0.2, 0.3);
        controller.on_mouse_move(0.2, 0.3, 0.4);
        controller.tick(0.016);
        let ptr = controller.get_vertex_buffer_ptr();
        let data = unsafe { std::slice::from_raw_parts(ptr, controller.get_vertex_count() * 3) };
        assert_ne!(data[0], 0.0); // Position should have changed

        // Release the mouse
        controller.on_mouse_up();
        controller.tick(0.016);
        let ptr2 = controller.get_vertex_buffer_ptr();
        let data2 = unsafe { std::slice::from_raw_parts(ptr2, controller.get_vertex_count() * 3) };
        assert_ne!(data, data2); // Position should have changed again
    }
}
