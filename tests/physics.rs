use nalgebra::Vector3;
use rust_learning_project::mesh::{Mesh, Vertex};
use rust_learning_project::physics::{Physics, Spring};

#[test]
fn test_spring_force() {
    let mut mesh = Mesh {
        vertices: vec![
            Vertex {
                position: Vector3::new(0.0, 0.0, 0.0),
                old_position: Vector3::new(0.0, 0.0, 0.0),
                acceleration: Vector3::zeros(),
                mass: 1.0,
            },
            Vertex {
                position: Vector3::new(2.0, 0.0, 0.0),
                old_position: Vector3::new(2.0, 0.0, 0.0),
                acceleration: Vector3::zeros(),
                mass: 1.0,
            },
        ],
        indices: vec![],
    };
    let mut physics = Physics::new();
    physics.springs.push(Spring {
        vertex_a_index: 0,
        vertex_b_index: 1,
        rest_length: 1.0,
        stiffness: 100.0,
        damping: 0.0,
    });

    physics.update(&mut mesh, None);

    // The spring is stretched by 1.0 unit, so the force should be 100.0 * 1.0 = 100.0
    // The force is applied in the direction from vertex_b to vertex_a
    // The acceleration is force / mass (assuming mass = 1.0)
    // Note: The physics update applies gravity first, so we need to account for that.
    // The test will be more stable if we set gravity to zero.
    physics.gravity = Vector3::zeros();
    physics.update(&mut mesh, None);

    assert!(mesh.vertices[0].acceleration.x > 0.0);
    assert!(mesh.vertices[1].acceleration.x < 0.0);
}

#[test]
fn test_verlet_integration() {
    let mut mesh = Mesh {
        vertices: vec![Vertex {
            position: Vector3::new(0.0, 0.0, 0.0),
            old_position: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
            mass: 1.0,
        }],
        indices: vec![],
    };
    let mut physics = Physics::new();
    physics.time_step = 0.1;
    physics.gravity = Vector3::new(0.0, -10.0, 0.0);

    physics.update(&mut mesh, None);

    // After 1st step:
    // pos = 0 + (0 - 0) + (0, -10, 0) * 0.1 * 0.1 = (0, -0.1, 0)
    assert_eq!(mesh.vertices[0].position.y, -0.1);

    physics.update(&mut mesh, None);

    // After 2nd step:
    // pos = -0.1 + (-0.1 - 0) + (0, -10, 0) * 0.1 * 0.1 = -0.1 - 0.1 - 0.1 = -0.3
    assert_eq!(mesh.vertices[0].position.y, -0.3);
}

#[test]
fn test_oscillation() {
    let mut mesh = Mesh {
        vertices: vec![
            Vertex {
                position: Vector3::new(0.0, 0.0, 0.0),
                old_position: Vector3::new(0.0, 0.0, 0.0),
                acceleration: Vector3::zeros(),
                mass: 0.0, // Pin this vertex
            },
            Vertex {
                position: Vector3::new(1.1, 0.0, 0.0),
                old_position: Vector3::new(1.1, 0.0, 0.0),
                acceleration: Vector3::zeros(),
                mass: 1.0,
            },
        ],
        indices: vec![],
    };
    let mut physics = Physics::new();
    physics.springs.push(Spring {
        vertex_a_index: 0,
        vertex_b_index: 1,
        rest_length: 1.0,
        stiffness: 100.0,
        damping: 0.0,
    });
    physics.gravity = Vector3::zeros();
    physics.time_step = 0.01;

    // Initial position of vertex 1 is 1.1, rest length is 1.0.
    // It should move towards vertex 0.
    physics.update(&mut mesh, None);
    assert!(mesh.vertices[1].position.x < 1.1);

    // Let it run for a while, it should oscillate around the rest length.
    for _ in 0..100 {
        physics.update(&mut mesh, None);
    }
    // After 100 steps, it should have oscillated back and be on the other side.
    assert!(mesh.vertices[1].position.x < 1.0);
}
