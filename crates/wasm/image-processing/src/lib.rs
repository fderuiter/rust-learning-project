use wasm_bindgen::prelude::*;
use image_processing;

// Re-exporting the image processing functions from the image-processing crate
/// Applies a grayscale filter to an image.
///
/// This is a re-export of the `apply_grayscale` function from the
/// `image-processing` crate.
///
/// # Arguments
///
/// * `image_bytes` - A byte slice of the image data in PNG format.
///
/// # Returns
///
/// A `Result` containing a `Vec<u8>` of the processed image data, or a `JsValue`
/// with an error message if the image processing fails.
#[wasm_bindgen]
pub fn apply_grayscale(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    image_processing::apply_grayscale(image_bytes)
}

/// Applies a sepia filter to an image.
///
/// This is a re-export of the `apply_sepia` function from the
/// `image-processing` crate.
///
/// # Arguments
///
/// * `image_bytes` - A byte slice of the image data in PNG format.
///
/// # Returns
///
/// A `Result` containing a `Vec<u8>` of the processed image data, or a `JsValue`
/// with an error message if the image processing fails.
#[wasm_bindgen]
pub fn apply_sepia(image_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    image_processing::apply_sepia(image_bytes)
}
