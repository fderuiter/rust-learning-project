use wasm_bindgen_test::*;
use rust_learning_project::mesh::Mesh;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_mesh_initialization() {
    let positions = vec![
        0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];
    let mesh = Mesh::new(&positions, &indices);

    assert_eq!(mesh.vertices.len(), 4);
    assert_eq!(mesh.indices.len(), 6);
}
