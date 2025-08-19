use nalgebra::Vector3;

pub struct Spring {
    pub vertex_a_index: usize,
    pub vertex_b_index: usize,
    pub rest_length: f32,
    pub stiffness: f32,
    pub damping: f32,
}

pub struct Physics {
    pub time_step: f32,
    pub gravity: Vector3<f32>,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            time_step: 0.01,
            gravity: Vector3::new(0.0, -9.81, 0.0),
        }
    }
}
