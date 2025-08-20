use wasm_bindgen::prelude::*;
use photon_rs::{monochrome, native::open_image_from_bytes};

#[wasm_bindgen]
pub fn apply_grayscale(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut img = open_image_from_bytes(image_bytes)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    monochrome::grayscale(&mut img);
    Ok(img.get_raw_pixels())
}

#[wasm_bindgen]
pub fn apply_sepia(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut img = open_image_from_bytes(image_bytes)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    monochrome::sepia(&mut img);
    Ok(img.get_raw_pixels())
}
