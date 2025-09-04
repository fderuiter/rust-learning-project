use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use face_detection;

/// Detects faces in an image. This function is a wrapper around the
/// `face_detection` crate's `detect_faces` function, and is only available
/// when not compiling for the `wasm32` target.
///
/// # Arguments
///
/// * `image_bytes` - A byte slice of the image data.
///
/// # Returns
///
/// A `Result` containing a `JsValue` with the bounding boxes of the detected
/// faces, or a `JsValue` with an error message.
#[wasm_bindgen]
#[cfg(not(target_arch = "wasm32"))]
pub fn detect_faces(image_bytes: &[u8]) -> Result<JsValue, JsValue> {
    let bboxes =
        face_detection::detect_faces(image_bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&bboxes).map_err(|e| JsValue::from_str(&e.to_string()))
}
