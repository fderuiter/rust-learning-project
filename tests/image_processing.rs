use rust_learning_project::image_processing;
use image::{ImageBuffer, Rgba};

fn create_test_image() -> Vec<u8> {
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(2, 2);
    img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
    img.put_pixel(1, 0, Rgba([0, 255, 0, 255]));
    img.put_pixel(0, 1, Rgba([0, 0, 255, 255]));
    img.put_pixel(1, 1, Rgba([255, 255, 0, 255]));
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png,
    )
    .expect("Failed to write test image");
    bytes
}

#[test]
fn test_apply_grayscale() {
    let image_bytes = create_test_image();
    let result = image_processing::apply_grayscale(&image_bytes);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[test]
fn test_apply_sepia() {
    let image_bytes = create_test_image();
    let result = image_processing::apply_sepia(&image_bytes);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}
