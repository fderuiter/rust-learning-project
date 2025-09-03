use image::GenericImageView;
use std::error::Error;
use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};

/// Represents a bounding box for a detected face.
///
/// The coordinates are normalized to the range [0, 1].
#[derive(Debug, serde::Serialize)]
pub struct BBox {
    /// The x-coordinate of the top-left corner of the bounding box.
    pub x1: f32,
    /// The y-coordinate of the top-left corner of the bounding box.
    pub y1: f32,
    /// The x-coordinate of the bottom-right corner of the bounding box.
    pub x2: f32,
    /// The y-coordinate of the bottom-right corner of the bounding box.
    pub y2: f32,
    /// The probability of the detected face.
    pub prob: f32,
}

/// Detects faces in an image using a pre-trained MTCNN model.
///
/// # Arguments
///
/// * `image_bytes` - A byte slice of the image data. The image format can be
///   any format supported by the `image` crate, such as PNG, JPEG, etc.
///
/// # Returns
///
/// A `Result` containing a `Vec` of `BBox` structs, one for each detected
/// face, or an error if face detection fails.
pub fn detect_faces(image_bytes: &[u8]) -> Result<Vec<BBox>, Box<dyn Error>> {
    let model = include_bytes!("../assets/mtcnn.pb");

    let mut graph = Graph::new();
    graph.import_graph_def(model, &ImportGraphDefOptions::new())?;

    let input_image = image::load_from_memory(image_bytes)?;

    let mut flattened: Vec<f32> = Vec::new();
    for (_x, _y, rgb) in input_image.pixels() {
        flattened.push(rgb[2] as f32);
        flattened.push(rgb[1] as f32);
        flattened.push(rgb[0] as f32);
    }

    let input = Tensor::new(&[input_image.height() as u64, input_image.width() as u64, 3])
        .with_values(&flattened)?;

    let min_size = Tensor::new(&[]).with_values(&[40f32])?;
    let thresholds = Tensor::new(&[3]).with_values(&[0.6f32, 0.7f32, 0.7f32])?;
    let factor = Tensor::new(&[]).with_values(&[0.709f32])?;

    let mut args = SessionRunArgs::new();

    args.add_feed(&graph.operation_by_name_required("min_size")?, 0, &min_size);
    args.add_feed(
        &graph.operation_by_name_required("thresholds")?,
        0,
        &thresholds,
    );
    args.add_feed(&graph.operation_by_name_required("factor")?, 0, &factor);
    args.add_feed(&graph.operation_by_name_required("input")?, 0, &input);

    let bbox = args.request_fetch(&graph.operation_by_name_required("box")?, 0);
    let prob = args.request_fetch(&graph.operation_by_name_required("prob")?, 0);

    let session = Session::new(&SessionOptions::new(), &graph)?;
    session.run(&mut args)?;

    let bbox_res: Tensor<f32> = args.fetch(bbox)?;
    let prob_res: Tensor<f32> = args.fetch(prob)?;

    let bboxes: Vec<_> = bbox_res
        .chunks_exact(4)
        .zip(prob_res.iter())
        .map(|(bbox, &prob)| BBox {
            y1: bbox[0],
            x1: bbox[1],
            y2: bbox[2],
            x2: bbox[3],
            prob,
        })
        .collect();

    Ok(bboxes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // This is an integration test, but it's placed here to be co-located with the
    // function it tests. It's ignored by default because it's slow and requires
    // the `assets` directory.
    #[test]
    #[ignore]
    fn test_detect_faces_on_sample_image() {
        // The test executable will run from the root of the workspace,
        // so the path to the asset is relative to the root.
        let image_bytes = fs::read("assets/test_face.jpg").expect("Failed to read test image");
        let result = detect_faces(&image_bytes);

        assert!(result.is_ok(), "Face detection failed: {:?}", result.err());

        let bboxes = result.unwrap();
        assert!(!bboxes.is_empty(), "No faces detected in the test image");

        // Basic sanity check on the first bounding box
        let bbox = &bboxes[0];
        assert!(bbox.x1 < bbox.x2);
        assert!(bbox.y1 < bbox.y2);
        assert!(bbox.prob > 0.9); // MTCNN is usually confident on this image
    }
}
