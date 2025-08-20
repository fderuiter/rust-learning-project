use wasm_bindgen::prelude::*;
use photon_rs::{PhotonImage, monochrome};

#[wasm_bindgen]
pub fn apply_grayscale(image_bytes: &[u8]) -> Vec<u8> {
    let mut img = photon_rs::native::open_image_from_bytes(image_bytes).unwrap();
    monochrome::grayscale(&mut img);
    img.get_raw_pixels()
}

#[wasm_bindgen]
pub fn apply_sepia(image_bytes: &[u8]) -> Vec<u8> {
    let mut img = photon_rs::native::open_image_from_bytes(image_bytes).unwrap();
    monochrome::sepia(&mut img);
    img.get_raw_pixels()
}
