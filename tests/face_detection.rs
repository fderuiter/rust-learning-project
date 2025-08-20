use rust_learning_project::face_detection;
use std::fs;

#[test]
fn test_detect_faces() {
    let image_bytes = fs::read("assets/test_face.jpg").expect("Failed to read test image");
    let result = face_detection::detect_faces(&image_bytes);
    assert!(result.is_ok());
    let bboxes = result.expect("Failed to detect faces");
    assert!(!bboxes.is_empty());
}
