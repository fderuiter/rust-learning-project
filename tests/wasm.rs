#![allow(dead_code)]

use rust_learning_project::FaceController;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn get_vertex_positions(controller: &FaceController, num_vertices: usize) -> Vec<f32> {
    let ptr = controller.get_vertex_buffer_ptr();
    let slice = unsafe { std::slice::from_raw_parts(ptr, num_vertices * 3) };
    slice.to_vec()
}

#[wasm_bindgen_test]
fn test_face_controller_new() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];
    let indices = vec![0, 1, 2, 0, 2, 3];
    let controller = FaceController::new(&positions, &indices);
    assert!(!controller.get_vertex_buffer_ptr().is_null());
    assert_eq!(controller.get_vertex_count(), 4);
}

#[wasm_bindgen_test]
fn test_tick() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];
    let indices = vec![0, 1, 2, 0, 2, 3];
    let mut controller = FaceController::new(&positions, &indices);

    let initial_positions = get_vertex_positions(&controller, 4);
    controller.tick(0.016);
    let new_positions = get_vertex_positions(&controller, 4);

    assert_ne!(initial_positions, new_positions);
}

#[wasm_bindgen_test]
fn test_mouse_interaction() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];
    let indices = vec![0, 1, 2, 0, 2, 3];
    let mut controller = FaceController::new(&positions, &indices);

    controller.on_mouse_down(0, 1.0, 2.0, 3.0);
    controller.on_mouse_move(4.0, 5.0, 6.0);

    // The vertex position is not updated in the buffer until after the tick
    controller.tick(0.016);
    let positions_after_move = get_vertex_positions(&controller, 4);
    assert_eq!(positions_after_move[0], 4.0);
    assert_eq!(positions_after_move[1], 5.0);
    assert_eq!(positions_after_move[2], 6.0);

    controller.on_mouse_up();
    controller.tick(0.016);
    let positions_after_mouseup = get_vertex_positions(&controller, 4);
    assert_ne!(positions_after_mouseup[0], 4.0);
}

#[wasm_bindgen_test]
#[should_panic]
fn test_face_controller_new_invalid_input() {
    let positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0]; // Invalid length
    let indices = vec![0, 1, 2, 0, 2, 3];
    FaceController::new(&positions, &indices);
}
