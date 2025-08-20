# Getting Started

This guide will walk you through the process of setting up the project, making a small change, and seeing the results.

## Prerequisites

Before you begin, make sure you have the following installed:

-   [Rust](https://www.rust-lang.org/tools/install)
-   [Node.js](https://nodejs.org/en/download/)

## Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/fderuiter/rust-learning-project.git
    cd rust-learning-project
    ```
2.  **Run the setup script:**
    ```bash
    ./setup_dev.sh
    ```
    This will install the correct Rust toolchain, the WebAssembly build target, and any other required dependencies.

## Running the Application

1.  **Start the development server:**
    ```bash
    trunk serve
    ```
2.  **Open the application in your browser:**
    Navigate to `http://localhost:8080` in your web browser. You should see a 3D face model.

## Making a Change

Now, let's make a small change to the application. We'll add a new image filter called "Invert".

1.  **Open `src/image_processing.rs` in your code editor.**
2.  **Add a new public function called `invert`:**

    ```rust
    // Add this to the bottom of the file

    /// Inverts the colors of an image.
    pub fn invert(image_bytes: &[u8]) -> Result<Vec<u8>, String> {
        let mut img = image::load_from_memory(image_bytes).map_err(|e| e.to_string())?.to_rgba8();
        for pixel in img.pixels_mut() {
            pixel[0] = 255 - pixel[0]; // R
            pixel[1] = 255 - pixel[1]; // G
            pixel[2] = 255 - pixel[2]; // B
        }
        let mut buffer = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png)
            .map_err(|e| e.to_string())?;
        Ok(buffer)
    }
    ```
3.  **Expose the new function to WebAssembly.** Open `src/lib.rs` and add the following code:

    ```rust
    // Add this with the other wasm_bindgen functions

    #[wasm_bindgen]
    pub fn invert(image_bytes: &[u8]) -> Result<JsValue, JsValue> {
        let new_image_bytes = image_processing::invert(image_bytes)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(js_sys::Uint8Array::from(&new_image_bytes[..]).into())
    }
    ```
4.  **Add a button to the frontend.** Open `index.html` and add a new button for the "Invert" filter:

    ```html
    <!-- Add this after the other filter buttons -->
    <button id="invert-button">Invert</button>
    ```
5.  **Hook up the button to the new Wasm function.** Open `main.js` and add the following code:

    ```javascript
    // Add this with the other event listeners

    document.getElementById('invert-button').addEventListener('click', async () => {
      if (!original_image_data) {
        alert('Please upload an image first.');
        return;
      }
      const new_image_bytes = await invert(original_image_data);
      const blob = new Blob([new_image_bytes], { type: 'image/png' });
      const url = URL.createObjectURL(blob);
      const texture = await new THREE.TextureLoader().load(url);
      face_mesh.material.map = texture;
      face_mesh.material.needsUpdate = true;
    });
    ```

## Seeing the Results

1.  **Go back to your browser.** The `trunk serve` command should have automatically reloaded the page.
2.  **Upload an image.**
3.  **Click the "Invert" button.** You should see the colors of the image on the face model invert.

## Next Steps

Congratulations! You've made your first change to the project. Here are some things you can do next:

-   Explore the codebase to understand how it works in more detail.
-   Try adding another image filter from the `photon-rs` documentation.
-   Read the rest of the documentation in the `docs/` directory to learn more about the project's architecture and features.
-   Check out the `TODO/` directory for ideas on how to contribute.
