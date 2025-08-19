use crate::mesh::Mesh;
use nalgebra::Vector3;
use std::collections::HashSet;

pub struct Spring {
    pub vertex_a_index: usize,
    pub vertex_b_index: usize,
    pub rest_length: f32,
    pub stiffness: f32,
    pub damping: f32,
}

pub struct Physics {
    pub springs: Vec<Spring>,
    pub time_step: f32,
    pub gravity: Vector3<f32>,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            springs: Vec::new(),
            time_step: 0.01,
            gravity: Vector3::new(0.0, -9.81, 0.0),
        }
    }

    pub fn init_springs(&mut self, mesh: &Mesh) {
        let mut existing_springs = HashSet::new();
        for triangle in mesh.indices.chunks_exact(3) {
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
                    let rest_length = (mesh.vertices[v1_idx].position
                        - mesh.vertices[v2_idx].position)
                        .magnitude();
                    self.springs.push(Spring {
                        vertex_a_index: v1_idx,
                        vertex_b_index: v2_idx,
                        rest_length,
                        stiffness: 1000.0,
                        damping: 10.0,
                    });
                }
            }
        }
    }

    pub fn update(&self, mesh: &mut Mesh) {
        // Apply gravity
        for vertex in &mut mesh.vertices {
            vertex.acceleration = self.gravity;
        }

        // Apply spring forces
        for spring in &self.springs {
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
            if vertex_a.mass > 0.0 {
                mesh.vertices[spring.vertex_a_index].acceleration -= total_force / vertex_a.mass;
            }
            if vertex_b.mass > 0.0 {
                mesh.vertices[spring.vertex_b_index].acceleration += total_force / vertex_b.mass;
            }
        }

        // Verlet integration
        for vertex in &mut mesh.vertices {
            let old_position = vertex.position;
            vertex.position = vertex.position
                + (vertex.position - vertex.old_position)
                + vertex.acceleration * self.time_step * self.time_step;
            vertex.old_position = old_position;
        }
    }
}
