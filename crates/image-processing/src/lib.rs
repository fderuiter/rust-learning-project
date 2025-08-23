use image::{load_from_memory_with_format, ImageFormat};
use photon_rs::{monochrome, PhotonImage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn apply_grayscale(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let dynamic_image = load_from_memory_with_format(image_bytes, ImageFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image from memory: {:?}", e)))?;
    let mut photon_image = PhotonImage::new(
        dynamic_image.to_rgba8().into_raw(),
        dynamic_image.width(),
        dynamic_image.height(),
    );
    monochrome::grayscale(&mut photon_image);
    Ok(photon_image.get_raw_pixels())
}

#[wasm_bindgen]
pub fn apply_sepia(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let dynamic_image = load_from_memory_with_format(image_bytes, ImageFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image from memory: {:?}", e)))?;
    let mut photon_image = PhotonImage::new(
        dynamic_image.to_rgba8().into_raw(),
        dynamic_image.width(),
        dynamic_image.height(),
    );
    monochrome::sepia(&mut photon_image);
    Ok(photon_image.get_raw_pixels())
}
