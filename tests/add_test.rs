use wasm_bindgen_test::*;
use rust_learning_project::add;

#[wasm_bindgen_test]
fn test_add_wasm() {
    assert_eq!(add(2, 3), 5);
}
