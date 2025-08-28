use image::{load_from_memory_with_format, ImageFormat};
use photon_rs::{monochrome, PhotonImage};
use std::error::Error;

/// Applies a grayscale filter to an image.
/// The input image is expected to be in PNG format.
pub fn apply_grayscale(image_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let dynamic_image = load_from_memory_with_format(image_bytes, ImageFormat::Png)?;
    let mut photon_image = PhotonImage::new(
        dynamic_image.to_rgba8().into_raw(),
        dynamic_image.width(),
        dynamic_image.height(),
    );
    monochrome::grayscale(&mut photon_image);
    Ok(photon_image.get_raw_pixels())
}

/// Applies a sepia filter to an image.
/// The input image is expected to be in PNG format.
pub fn apply_sepia(image_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let dynamic_image = load_from_memory_with_format(image_bytes, ImageFormat::Png)?;
    let mut photon_image = PhotonImage::new(
        dynamic_image.to_rgba8().into_raw(),
        dynamic_image.width(),
        dynamic_image.height(),
    );
    monochrome::sepia(&mut photon_image);
    Ok(photon_image.get_raw_pixels())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::io::Reader as ImageReader;
    use std::fs;
    use std::io::Cursor;

    fn get_test_image_as_png() -> Vec<u8> {
        let image_bytes = fs::read("../../static/assets/test_face.jpg").expect("Failed to read test image");
        let image = ImageReader::new(Cursor::new(image_bytes))
            .with_guessed_format()
            .expect("Failed to guess image format")
            .decode()
            .expect("Failed to decode image");
        let mut png_bytes = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut png_bytes), image::ImageOutputFormat::Png)
            .expect("Failed to write image to png");
        png_bytes
    }

    #[test]
    fn test_apply_grayscale() {
        let image_bytes = get_test_image_as_png();
        let result = apply_grayscale(&image_bytes);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_sepia() {
        let image_bytes = get_test_image_as_png();
        let result = apply_sepia(&image_bytes);
        assert!(result.is_ok());
    }
}
