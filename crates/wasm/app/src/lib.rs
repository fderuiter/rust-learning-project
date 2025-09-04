use wasm_bindgen::prelude::*;

// Import crates
use mesh::Mesh;
use physics::Physics;

/// A controller for the 3D face mesh, handling user interactions and physics.
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
    ///
    /// # Arguments
    ///
    /// * `positions` - A flat array of vertex positions for the mesh.
    /// * `indices` - An array of indices for the mesh.
    ///
    /// # Returns
    ///
    /// A new `FaceController` instance.
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

    /// Advances the physics simulation by a given time step.
    ///
    /// * `dt` - The time step to advance the simulation by.
    pub fn tick(&mut self, dt: f32) {
        self.physics.time_step = dt;
        self.physics.update(
            &mut self.mesh,
            self.dragged_vertex_index.map(|i| i as usize),
        );
        self.vertex_positions = self.mesh.get_vertex_positions_flat();
    }

    /// Handles the mouse down event, starting a drag operation on a vertex.
    ///
    /// # Arguments
    ///
    /// * `vertex_id` - The ID of the vertex to drag.
    /// * `x` - The new x-coordinate of the vertex.
    /// * `y` - The new y-coordinate of the vertex.
    /// * `z` - The new z-coordinate of the vertex.
    pub fn on_mouse_down(&mut self, vertex_id: u32, x: f32, y: f32, z: f32) {
        self.dragged_vertex_index = Some(vertex_id);
        self.mesh.vertices[vertex_id as usize].position.x = x;
        self.mesh.vertices[vertex_id as usize].position.y = y;
        self.mesh.vertices[vertex_id as usize].position.z = z;
    }

    /// Handles the mouse move event, updating the position of the dragged vertex.
    ///
    /// # Arguments
    ///
    /// * `x` - The new x-coordinate of the vertex.
    /// * `y` - The new y-coordinate of the vertex.
    /// * `z` - The new z-coordinate of the vertex.
    pub fn on_mouse_move(&mut self, x: f32, y: f32, z: f32) {
        if let Some(vertex_id) = self.dragged_vertex_index {
            self.mesh.vertices[vertex_id as usize].position.x = x;
            self.mesh.vertices[vertex_id as usize].position.y = y;
            self.mesh.vertices[vertex_id as usize].position.z = z;
        }
    }

    /// Handles the mouse up event, ending the drag operation.
    pub fn on_mouse_up(&mut self) {
        self.dragged_vertex_index = None;
    }

    /// Returns a pointer to the vertex buffer.
    ///
    /// # Returns
    ///
    /// A raw pointer to the vertex buffer.
    pub fn get_vertex_buffer_ptr(&self) -> *const f32 {
        self.vertex_positions.as_ptr()
    }

    /// Returns the number of vertices in the mesh.
    ///
    /// # Returns
    ///
    /// The number of vertices.
    pub fn get_vertex_count(&self) -> usize {
        self.mesh.vertices.len()
    }
}
