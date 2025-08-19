## **Phase 3: Frontend and Rendering (JavaScript/Three.js)**

This phase focuses on building the visible and interactive part of the application. It leverages the Two-Stack architecture, using Three.js for all rendering tasks and communicating with the Rust/Wasm backend for the deformation logic.

### **3.1 Scene Setup**

* **Task 3.1.1: Initialize Three.js Environment.**  
  * **Action:** In main.js, set up the core Three.js components:  
    1. THREE.Scene: The container for all 3D objects.  
    2. THREE.PerspectiveCamera: To view the scene.  
    3. THREE.WebGLRenderer: To render the scene to the HTML canvas, configured to the full window size.  
    4. THREE.AmbientLight and THREE.DirectionalLight: To illuminate the face model.  
    5. OrbitControls: To allow the user to rotate and zoom the camera around the face.  
  * **Verification:** The application displays an empty, lit 3D scene. The user can click and drag to rotate the view.

### **3.2 Mesh Loading and Initialization**

* **Task 3.2.1: Load 3D Face Model.**  
  * **Action:** Use a Three.js loader (e.g., GLTFLoader or OBJLoader) to load a 3D face model. A simple, low-polygon model is ideal for the initial implementation.  
  * **Verification:** The 3D face model is successfully loaded and displayed statically in the center of the scene.  
* **Task 3.2.2: Initialize Wasm Module with Mesh Data.**  
  * **Action:** Once the model is loaded, extract its vertex position data and index data from the Three.js geometry. These are typically available as geometry.attributes.position.array. Pass these Float32Arrays to the Wasm module's constructor to initialize the FaceController. Store the returned FaceController instance in a global JavaScript variable.  
  * **Verification:** The Wasm module is initialized without errors. Console logs from within the Rust constructor can confirm that the correct number of vertices and indices were received.

### **3.3 User Interaction and Raycasting**

To allow the user to "grab" a point on the face, we need to translate a 2D mouse click on the screen into the selection of a 3D vertex on the mesh.

* **Task 3.3.1: Implement Mouse Event Listeners.**  
  * **Action:** Add JavaScript event listeners to the canvas for mousedown, mousemove, and mouseup events.  
* **Task 3.3.2: Implement Raycasting for Vertex Selection.**  
  * **Action:** On mousedown, use Three.js's Raycaster. This involves:  
    1. Converting the 2D mouse coordinates to normalized device coordinates (a range from \-1 to 1).  
    2. Creating a Raycaster that projects a ray from the camera through that point into the 3D scene.  
    3. Calling raycaster.intersectObject(mesh) to get a list of intersection points.  
    4. From the closest intersection point, identify the nearest vertex on the mesh. This vertex's index is the vertex\_id that will be managed.  
    5. Call the Wasm function faceController.on\_mouse\_down(vertex\_id,...).  
  * **Verification:** Clicking on the face model logs the index of the nearest vertex to the console. A visual indicator (e.g., changing the color of the selected vertex) confirms the selection is working.

### **3.4 The Render Loop**

The render loop is the heart of the application, orchestrating the communication between the user, the Wasm physics engine, and the Three.js renderer on every frame.

* **Task 3.4.1: Structure the requestAnimationFrame Loop.**  
  * **Action:** Create an animate() function that calls itself recursively using requestAnimationFrame. This ensures the animation runs smoothly and efficiently.  
* **Task 3.4.2: Implement the Frame Logic.**  
  * **Action:** Inside the animate() loop, perform the following steps in order:  
    1. **Update Input:** If the user is currently dragging the mouse (mousemove event), calculate the new 3D position of the dragged vertex and call the Wasm function faceController.on\_mouse\_move(...).  
    2. **Tick Physics:** Call faceController.tick(deltaTime) to advance the Rust physics simulation by one step.  
    3. **Retrieve Vertex Data:** Call faceController.get\_vertex\_buffer\_ptr() to get the memory offset of the updated vertex data within the Wasm module's linear memory.  
    4. **Update Geometry:** Access the Wasm memory buffer from JavaScript (wasm\_module.memory.buffer) and create a new Float32Array view pointing at the retrieved offset. Use this view to directly update the array property of the Three.js mesh's BufferGeometry position attribute (mesh.geometry.attributes.position.array.set(wasmBufferView)).  
    5. **Notify Three.js:** Set the needsUpdate flag on the position attribute to true (mesh.geometry.attributes.position.needsUpdate \= true;). This crucial step tells Three.js to re-upload the vertex data to the GPU on the next render call.34  
    6. **Render:** Call renderer.render(scene, camera).  
  * **Verification:** Clicking and dragging a point on the face causes the mesh to deform in real-time. When the mouse is released, the mesh "jiggles" and returns to its original shape according to the physics simulation. The animation runs at a smooth 60 frames per second.
