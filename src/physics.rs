use crate::mesh::Mesh;
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

impl Default for Physics {
    fn default() -> Self {
        Self {
            time_step: 0.01,
            gravity: Vector3::new(0.0, -9.81, 0.0),
        }
    }
}

impl Physics {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn update(mesh: &mut Mesh, time_step: f32, gravity: &Vector3<f32>) {
    for vertex in &mut mesh.vertices {
        vertex.acceleration = *gravity;
    }

    for spring in &mesh.springs {
        let vertex_a = mesh.vertices[spring.vertex_a_index];
        let vertex_b = mesh.vertices[spring.vertex_b_index];

        let delta = vertex_a.position - vertex_b.position;
        let distance = delta.magnitude();
        let direction = delta.normalize();

        let stretch = distance - spring.rest_length;
        let spring_force = spring.stiffness * stretch * direction;

        let relative_velocity = (vertex_a.position - vertex_a.old_position)
            - (vertex_b.position - vertex_b.old_position);
        let damping_force = spring.damping * relative_velocity.dot(&direction) * direction;

        let total_force = spring_force + damping_force;

        let mass_a = mesh.vertices[spring.vertex_a_index].mass;
        if mass_a > 0.0 {
            mesh.vertices[spring.vertex_a_index].acceleration -= total_force / mass_a;
        }

        let mass_b = mesh.vertices[spring.vertex_b_index].mass;
        if mass_b > 0.0 {
            mesh.vertices[spring.vertex_b_index].acceleration += total_force / mass_b;
        }
    }

    for vertex in &mut mesh.vertices {
        let old_position = vertex.position;
        vertex.position = vertex.position
            + (vertex.position - vertex.old_position)
            + vertex.acceleration * time_step * time_step;
        vertex.old_position = old_position;
    }
}
