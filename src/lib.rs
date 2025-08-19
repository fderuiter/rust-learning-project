pub mod mesh;
pub mod physics;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello, World!".to_string()
}
