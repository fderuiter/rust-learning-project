use mesh::Mesh;
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

impl Default for Physics {
    fn default() -> Self {
        Self {
            springs: Vec::new(),
            time_step: 0.01,
            gravity: Vector3::new(0.0, -9.81, 0.0),
        }
    }
}

impl Physics {
    pub fn new() -> Self {
        Self::default()
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

    pub fn update(&self, mesh: &mut Mesh, dragged_vertex_index: Option<usize>) {
        // Apply gravity
        for (i, vertex) in &mut mesh.vertices.iter_mut().enumerate() {
            if Some(i) == dragged_vertex_index {
                continue;
            }
            vertex.acceleration = self.gravity;
        }

        // Apply spring forces
        for spring in &self.springs {
            let vertex_a = mesh.vertices[spring.vertex_a_index];
            let vertex_b = mesh.vertices[spring.vertex_b_index];
            let delta = vertex_a.position - vertex_b.position;
            let distance = delta.magnitude();
            // Avoid division by zero if vertices are at the same position
            if distance > 1e-6 {
                let direction = delta.normalize();
                let stretch = distance - spring.rest_length;
                let spring_force = spring.stiffness * stretch * direction;
                let relative_velocity = (vertex_a.position - vertex_a.old_position)
                    - (vertex_b.position - vertex_b.old_position);
                let damping_force = spring.damping * relative_velocity.dot(&direction) * direction;
                let total_force = spring_force + damping_force;

                if Some(spring.vertex_a_index) != dragged_vertex_index && vertex_a.mass > 0.0 {
                    mesh.vertices[spring.vertex_a_index].acceleration -=
                        total_force / vertex_a.mass;
                }
                if Some(spring.vertex_b_index) != dragged_vertex_index && vertex_b.mass > 0.0 {
                    mesh.vertices[spring.vertex_b_index].acceleration +=
                        total_force / vertex_b.mass;
                }
            }
        }

        // Verlet integration
        for (i, vertex) in &mut mesh.vertices.iter_mut().enumerate() {
            if Some(i) == dragged_vertex_index {
                continue;
            }
            let old_position = vertex.position;
            vertex.position = vertex.position
                + (vertex.position - vertex.old_position)
                + vertex.acceleration * self.time_step * self.time_step;
            vertex.old_position = old_position;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;
    use mesh::Mesh;

    #[test]
    fn test_physics_new() {
        let physics = Physics::new();
        assert_eq!(physics.springs.len(), 0);
        assert_eq!(physics.time_step, 0.01);
        assert_eq!(physics.gravity, Vector3::new(0.0, -9.81, 0.0));
    }

    // Helper to create a basic mesh for testing
    fn create_test_mesh(positions: Vec<f32>, indices: Vec<u32>) -> Mesh {
        Mesh::new(&positions, &indices).unwrap()
    }

    #[test]
    fn test_init_springs_single_triangle() {
        let mesh = create_test_mesh(
            vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            vec![0, 1, 2],
        );
        let mut physics = Physics::new();
        physics.init_springs(&mesh);
        assert_eq!(physics.springs.len(), 3); // A triangle has 3 edges
    }

    #[test]
    fn test_init_springs_shared_edge() {
        // Two triangles sharing an edge (0, 1)
        let mesh = create_test_mesh(
            vec![
                0.0, 0.0, 0.0, // 0
                1.0, 0.0, 0.0, // 1
                0.0, 1.0, 0.0, // 2
                1.0, 1.0, 0.0, // 3
            ],
            vec![0, 1, 2, 0, 3, 1],
        );
        let mut physics = Physics::new();
        physics.init_springs(&mesh);
        // Should be 5 springs: (0,1), (1,2), (2,0), (0,3), (3,1)
        assert_eq!(physics.springs.len(), 5);
    }

    #[test]
    fn test_update_spring_force() {
        let mut mesh = create_test_mesh(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], vec![]);
        let mut physics = Physics::new();
        physics.springs.push(Spring {
            vertex_a_index: 0,
            vertex_b_index: 1,
            rest_length: 0.5, // Spring is stretched
            stiffness: 100.0,
            damping: 0.0,
        });

        physics.update(&mut mesh, None);

        // Vertex 0 should have moved right, and vertex 1 left
        assert!(mesh.vertices[0].position.x > 0.0);
        assert!(mesh.vertices[1].position.x < 1.0);
    }

    #[test]
    fn test_update_immovable_vertex() {
        let mut mesh = create_test_mesh(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], vec![]);
        mesh.vertices[0].mass = 0.0; // Make vertex 0 immovable

        let mut physics = Physics::new();
        physics.springs.push(Spring {
            vertex_a_index: 0,
            vertex_b_index: 1,
            rest_length: 0.5,
            stiffness: 100.0,
            damping: 0.0,
        });

        physics.update(&mut mesh, None);

        // Vertex 0 should not have moved
        assert_eq!(mesh.vertices[0].position.x, 0.0);
        // Vertex 1 should have moved
        assert!(mesh.vertices[1].position.x < 1.0);
    }

    #[test]
    fn test_update_zero_distance_spring() {
        let mut mesh = create_test_mesh(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0], vec![]);
        let mut physics = Physics::new();
        physics.springs.push(Spring {
            vertex_a_index: 0,
            vertex_b_index: 1,
            rest_length: 0.5,
            stiffness: 100.0,
            damping: 0.0,
        });

        // This should not panic due to division by zero.
        // The `normalize()` on a zero vector results in a zero vector, so no force is applied.
        physics.update(&mut mesh, None);

        assert_eq!(mesh.vertices[0].position.x, 0.0);
        assert_eq!(mesh.vertices[1].position.x, 0.0);
    }
}
