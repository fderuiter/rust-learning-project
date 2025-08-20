use image::GenericImageView;
use std::error::Error;
use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, SessionRunArgs, Tensor};

#[derive(Debug)]
pub struct BBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub prob: f32,
}

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
