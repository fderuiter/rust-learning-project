## **Phase 4: Advanced Features and Polish**

Once the core functionality is stable, these features can be added to enhance the user experience and showcase the power of the Rust/Wasm stack.

### **4.1 Face Detection for Vertex Interaction**

* **Task 4.1.1: Integrate a Face Detection Library.**  
  * **Action:** Instead of requiring the user to find vertices manually, this feature would automatically identify key facial landmarks (corners of eyes, tip of nose, corners of mouth) as pre-defined "handles" for interaction. The rust-faces crate, which provides an interface to models like BlazeFace, is a strong candidate.35 The goal would be to compile a lightweight face detection model to Wasm.  
  * **Implementation:** Upon loading an image, it would be passed to a Wasm function that runs the face detector. The detector would return the coordinates of key landmarks. These coordinates would then be mapped to the nearest vertices on the 3D mesh, which are then highlighted as interactive points.  
  * **Verification:** When a user uploads a photo of a face, distinct markers appear on the 3D model corresponding to the detected facial features. These markers are the primary points for stretching.

### **4.2 Image-Based Face Stretching**

* **Task 4.2.1: Implement Image Upload and Texture Mapping.**  
  * **Action:** Add an HTML \<input type="file"\> element to allow users to upload their own images. When an image is selected, use JavaScript to read it into an ArrayBuffer.  
  * **Verification:** The uploaded image is successfully displayed as a texture on the 3D face model using Three.js's TextureLoader and MeshBasicMaterial.  
* **Task 4.2.2: Wasm-Powered Image Pre-processing.**  
  * **Action:** For better integration, the uploaded image data can be processed in Wasm before being used as a texture. The photon-rs library is a high-performance, pure Rust image processing library that compiles to Wasm and is ideal for this purpose.37 The  
    ArrayBuffer from JavaScript can be passed to a Rust function that uses photon-rs to perform operations like resizing, color correction, or applying filters.39 The processed image data is then passed back to JavaScript to be applied as a texture.  
  * **Verification:** A user uploads an image. They can select a filter (e.g., "grayscale," "sepia") which is applied near-instantly via the Wasm module before the texture is updated on the 3D model.
