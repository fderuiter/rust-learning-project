# Frontend Documentation

This document describes the frontend setup and implementation of the 3D face-stretching application.

## Three.js Setup

The scene is set up in `main.js`. It includes:
- A `PerspectiveCamera` with a 75-degree field of view.
- A `WebGLRenderer` attached to the canvas element with the ID `main-canvas`.
- `OrbitControls` for camera manipulation.
- An `AmbientLight` and a `DirectionalLight` to illuminate the scene.
- A window resize handler to ensure the canvas and camera aspect ratio are updated when the window is resized.

## Model Loading

The 3D model is loaded using `GLTFLoader`. The model is expected to be in glTF format and located at `assets/face.gltf`.

Upon loading, the vertex and index data is extracted from the model's geometry and passed to the Wasm `FaceController` for physics simulation.

## User Interaction

The application supports the following user interactions:

- **Image Upload**: Users can upload an image using the file input. The uploaded image is applied as a texture to the 3D model, and face detection is performed on it.
- **Filter Buttons**: Users can apply grayscale and sepia filters to the uploaded image by clicking the corresponding buttons.
- **Mouse Interaction**: The application uses a `Raycaster` to detect mouse interactions with the 3D model.

- **Mousedown**: When the user clicks on the model, a ray is cast from the camera to the mouse position. The closest intersected vertex is identified, and its index is passed to the `faceController.on_mouse_down()` function.
- **Mousemove**: When the user drags the mouse while holding the button down, the new mouse position is used to calculate a new position for the selected vertex in 3D space. This new position is passed to the `faceController.on_mouse_move()` function.
- **Mouseup**: When the user releases the mouse button, the `faceController.on_mouse_up()` function is called to end the dragging interaction.

## Render Loop

The application uses `requestAnimationFrame` to create a render loop. In each frame, the following steps are performed:
1. The time delta since the last frame is calculated.
2. The `faceController.tick()` function is called with the delta time to update the physics simulation.
3. The updated vertex positions are retrieved from the Wasm module.
4. The `BufferGeometry` of the Three.js mesh is updated with the new vertex positions.
5. The scene is rendered.
