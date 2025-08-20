# Face Detection

This document describes the face detection functionality of the application.

## Library

The face detection is implemented using the `tensorflow` Rust crate with a pre-trained MTCNN model. The model file (`mtcnn.pb`) is included in the `assets` directory.

## Implementation

The face detection logic is encapsulated in the `src/face_detection.rs` module. It exposes a `detect_faces` function that takes an image buffer as input and returns a list of bounding boxes for the detected faces.

The `detect_faces` function is called from `main.js` when a user uploads an image. The bounding boxes are then used to identify facial landmarks, which are mapped to the nearest vertices on the 3D mesh. These vertices are then highlighted with red spheres and can be manipulated by the user.
