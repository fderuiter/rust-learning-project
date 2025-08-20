use rust_learning_project::face_detection;
use std::fs;

#[test]
fn test_detect_faces() {
    let image_bytes = fs::read("assets/test_face.jpg").unwrap();
    let result = face_detection::detect_faces(&image_bytes);
    assert!(result.is_ok());
    let bboxes = result.unwrap();
    assert!(!bboxes.is_empty());
}
