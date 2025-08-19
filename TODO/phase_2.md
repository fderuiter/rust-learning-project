## **Phase 2: Core Logic Implementation (Rust/Wasm)**

With the foundational architecture established, this phase focuses on implementing the computational core of the application within the Rust/Wasm module. This includes defining the necessary data structures, building the custom physics engine for the stretching effect, and designing a clean API for JavaScript to interact with.

### **2.1 Data Structures for Mesh Representation**

The first step is to model the 3D face in a way that is conducive to physics simulation and deformation. This requires efficient data structures for vertices, their connectivity, and the physical properties that govern their movement.

* **Task 2.1.1: Define Vertex and Mesh Structs.**  
  * **Action:** Create core Rust structs to represent the 3D model. A Vertex struct will store its current position, previous position (for Verlet integration), acceleration, and a resting (original) position. A Mesh struct will own a collection of these Vertex objects, along with connectivity information (e.g., faces or edges).28 A simple vector math library (or custom implementation) will be used for 3D vector operations.  
  * **Verification:** The data structures are defined and can be instantiated. Unit tests confirm that vector math operations (addition, subtraction, normalization) are correct.  
* **Task 2.1.2: Implement Mesh Initialization.**  
  * **Action:** Create a function, exposed to Wasm, that can initialize the Mesh struct from raw vertex and index data provided by JavaScript. This function will take two flat Float32Arrays (one for vertex positions, one for indices) as input, parse them, and populate the internal Mesh and Vertex collections.  
  * **Verification:** A Wasm integration test passes a simple cube's vertex and index data to the initialization function. The internal Rust Mesh struct is verified to contain the correct number of vertices and faces with the correct initial positions.

### **2.2 Physics Engine Implementation**

The characteristic "stretchy" and "jiggly" feel of the Mario 64 face is a classic example of a soft-body physics simulation. While a comprehensive physics engine like Rapier is excellent for rigid body dynamics, it is overkill for this specific effect.30 A custom, lightweight spring-mass system will provide finer control over the desired aesthetic, result in a smaller Wasm binary, and be more performant for this specific use case.

#### **2.2.1 Architectural Decision: Custom Physics Model**

* **Decision:** A custom, simplified spring-mass physics model will be implemented directly in Rust.  
* **Justification:** The project's physics requirements are highly specific: a network of vertices connected by damped springs that return to a resting state. A general-purpose engine like Rapier, with its complex collision detection and constraint solvers, introduces unnecessary overhead.30 Implementing a bespoke model based on fundamental physics principles allows for precise tuning of parameters like spring stiffness and damping, which are crucial for achieving the target artistic "feel." This approach is computationally simpler and directly addresses the problem without extraneous features.32

#### **2.2.2 Implement Spring-Mass System**

* **Task 2.2.2.1: Define Spring Constraints.**  
  * **Action:** Model the edges of the 3D mesh as springs. Each spring connects two vertices and has properties for stiffness (k) and damping (b). The force exerted by a spring will be calculated using a combination of Hooke's Law (for the spring force) and a damping force proportional to the relative velocity of its two endpoints. The fundamental equation is F=−k(∣x∣−d)(x/∣x∣)−bv, where k is the spring constant, x is the vector between the two nodes, d is the resting length, b is the damping coefficient, and v is the relative velocity.33  
  * **Verification:** A unit test with two vertices connected by a spring correctly calculates the restoring and damping forces when the vertices are displaced and moving relative to each other.  
* **Task 2.2.2.2: Implement Numerical Integration.**  
  * **Action:** Use a numerical integration method to update the position and velocity of each vertex over discrete time steps. Verlet integration is an excellent choice for this type of simulation as it is computationally efficient and stable. In each simulation step (tick), for every vertex:  
    1. Calculate the total force acting on it by summing the forces from all connected springs and any external forces (like the user's drag).  
    2. Calculate acceleration using Newton's second law: a=F/m.  
    3. Update the vertex's position based on its previous position and current acceleration.  
  * **Verification:** Unit tests for the integration step show that a single particle under a constant force (like gravity) follows a parabolic trajectory, and a simple spring system oscillates as expected.

### **2.3 Wasm API Design and Implementation**

The wasm-bindgen API is the contract between the Rust core and the JavaScript frontend. It must be designed for both functionality and performance, particularly concerning the frequent transfer of data in the render loop.

* **Task 2.3.1: Define the Public Wasm Struct.**  
  * **Action:** Create a primary public struct, e.g., FaceController, annotated with \#\[wasm\_bindgen\]. This struct will encapsulate the Mesh and the entire physics state. Its constructor will be exposed to JS to initialize the simulation with the mesh data.  
  * **Verification:** JavaScript can successfully instantiate the FaceController object.  
* **Task 2.3.2: Implement State Update and I/O Functions.**  
  * **Action:** Expose the following methods on the FaceController struct:  
    * tick(dt: f32): Advances the physics simulation by a time delta dt.  
    * on\_mouse\_down(vertex\_id: u32, x: f32, y: f32, z: f32): Informs the simulation that a user has started dragging a specific vertex to a new position in 3D space.  
    * on\_mouse\_move(x: f32, y: f32, z: f32): Updates the position of the currently dragged vertex.  
    * on\_mouse\_up(): Releases the dragged vertex, allowing it to be fully controlled by the physics simulation again.  
    * get\_vertex\_buffer\_ptr() \-\> \*const f32: This is the most performance-critical function. It returns a raw pointer to the start of the flat Vec\<f32\> that holds the updated vertex positions. This allows JavaScript to read the data directly from the Wasm linear memory without incurring the cost of copying the entire buffer on every frame.18  
  * **Verification:** A suite of wasm-bindgen-test integration tests confirms that each API function can be called from JavaScript and correctly modifies the internal state of the FaceController. The pointer returned by get\_vertex\_buffer\_ptr is validated to be non-null.

---
