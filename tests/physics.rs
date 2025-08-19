use nalgebra::Vector3;
use rust_learning_project::mesh::{Mesh, Vertex};
use rust_learning_project::physics::Spring;

#[test]
fn test_spring_force() {
    let mut mesh = Mesh::new(&[], &[]);
    mesh.vertices = vec![
        Vertex {
            position: Vector3::new(0.0, 0.0, 0.0),
            old_position: Vector3::new(0.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
            resting_position: Vector3::new(0.0, 0.0, 0.0),
            mass: 1.0,
        },
        Vertex {
            position: Vector3::new(2.0, 0.0, 0.0),
            old_position: Vector3::new(2.0, 0.0, 0.0),
            acceleration: Vector3::zeros(),
            resting_position: Vector3::new(1.0, 0.0, 0.0),
            mass: 1.0,
        },
    ];
    mesh.springs = vec![Spring {
        vertex_a_index: 0,
        vertex_b_index: 1,
        rest_length: 1.0,
        stiffness: 100.0,
        damping: 0.0,
    }];

    mesh.update();

    // The spring is stretched by 1.0 unit, so the force should be 100.0 * 1.0 = 100.0
    // The force is applied in the direction from vertex_b to vertex_a
    // The acceleration is force / mass (assuming mass = 1.0)
    assert_eq!(mesh.vertices[0].acceleration.x, 100.0);
    assert_eq!(mesh.vertices[1].acceleration.x, -100.0);
}

#[test]
fn test_verlet_integration() {
    let mut mesh = Mesh::new(&[], &[]);
    mesh.vertices = vec![Vertex {
        position: Vector3::new(0.0, 0.0, 0.0),
        old_position: Vector3::new(0.0, 0.0, 0.0),
        acceleration: Vector3::new(0.0, -10.0, 0.0),
        resting_position: Vector3::new(0.0, 0.0, 0.0),
        mass: 1.0,
    }];
    mesh.physics.time_step = 0.1;
    mesh.physics.gravity = Vector3::new(0.0, -10.0, 0.0);

    mesh.update();

    // After 1st step:
    // pos = 0 + (0 - 0) + (0, -10, 0) * 0.1 * 0.1 = (0, -0.1, 0)
    assert_eq!(mesh.vertices[0].position.y, -0.1);

    mesh.update();

    // After 2nd step:
    // pos = -0.1 + (-0.1 - 0) + (0, -10, 0) * 0.1 * 0.1 = -0.1 - 0.1 - 0.1 = -0.3
    assert_eq!(mesh.vertices[0].position.y, -0.3);
}

#[test]
fn test_oscillation() {
    let mut mesh = Mesh::new(&[0.0, 0.0, 0.0, 1.1, 0.0, 0.0], &[]);
    mesh.vertices[0].mass = 0.0; // Pin this vertex
    mesh.springs = vec![Spring {
        vertex_a_index: 0,
        vertex_b_index: 1,
        rest_length: 1.0,
        stiffness: 100.0,
        damping: 0.0,
    }];
    mesh.physics.gravity = Vector3::zeros();
    mesh.physics.time_step = 0.01;

    // Initial position of vertex 1 is 1.1, rest length is 1.0.
    // It should move towards vertex 0.
    mesh.update();
    assert!(mesh.vertices[1].position.x < 1.1);

    // Let it run for a while, it should oscillate around the rest length.
    for _ in 0..100 {
        mesh.update();
    }
    // After 100 steps, it should have oscillated back and be on the other side.
    assert!(mesh.vertices[1].position.x < 1.0);
}
