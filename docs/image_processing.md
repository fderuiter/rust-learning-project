# Image Processing

This document describes the image processing functionality of the application.

## Library

The image processing is implemented using the `photon-rs` Rust crate.

## Implementation

The image processing logic is encapsulated in the `src/image_processing.rs` module. It exposes functions for applying various filters to an image. Currently, the following filters are implemented:

*   Grayscale
*   Sepia

These functions take an image buffer as input and return a new image buffer with the filter applied.

The filter functions are called from `main.js` when a user clicks on the corresponding filter button. The new image buffer is then used to update the texture on the 3D face model.
