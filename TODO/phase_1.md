## **Phase 1: Project Foundation and Architectural Decisions**

This initial phase is the most critical, as the decisions made here will dictate the entire development trajectory, workflow, and final architecture of the application. The primary goal is to establish a stable, efficient, and scalable foundation by selecting the right tools and architectural patterns that align with the project's core objective: creating a high-performance, interactive 3D experience on the web.

### **1.1 Environment and Toolchain Configuration**

A standardized development environment is paramount to ensure consistency, prevent environment-specific defects, and streamline the onboarding process for new team members. The following tasks establish this baseline.

#### **1.1.1 Install Rust Toolchain**

* **Action:** All developers must install the Rust programming language and its associated tooling via rustup. This tool facilitates the management of Rust versions, targets, and components, ensuring that the entire team works with a consistent compiler and build system.1 A  
  rust-toolchain.toml file will be committed to the root of the project repository to lock the specific version of the Rust toolchain, which rustup will automatically respect.  
* **Verification:** Executing rustc \--version and cargo \--version in the project directory should display the versions specified in the rust-toolchain.toml file.

#### **1.1.2 Install WebAssembly Build Target**

* **Action:** The standard Rust installation does not include the WebAssembly (Wasm) compilation target by default. It must be added explicitly by executing the command: rustup target add wasm32-unknown-unknown.2 This target is specifically designed for generating Wasm binaries that can run in a browser environment, which lacks the typical operating system interfaces.  
* **Verification:** The command rustup target list \--installed must list wasm32-unknown-unknown among the installed targets.

#### **1.1.3 Establish Code Repository and Version Control**

* **Action:** A Git repository will be initialized to manage the project's source code. A robust branching strategy, such as GitFlow, will be adopted to manage feature development, releases, and hotfixes in an organized manner. The main development branch will be protected to enforce a pull-request-based review process. A continuous integration (CI) pipeline will be configured to automatically build and test the project on every commit and pull request.  
* **Verification:** A newly cloned repository has a clean working directory. Pull requests to the main branch are blocked without passing all CI checks and receiving at least one code review approval.

### **1.2 Build System and Bundler Selection**

The choice of a bundler is a pivotal decision that profoundly influences the developer experience, build complexity, and overall project workflow. The modern Rust and WebAssembly ecosystem offers several mature options, with wasm-pack and Trunk being the leading candidates for this project.

#### **1.2.1 Analysis of Bundler Options**

* **wasm-pack:** This tool is a comprehensive solution for building, testing, and publishing Rust-generated WebAssembly to be consumed as a package.4 Its primary design goal is to create a library or module that can be easily integrated into a larger, potentially pre-existing, JavaScript application.5 It excels at generating the necessary JavaScript "glue" code and TypeScript type definitions, making the Wasm module feel like a native JavaScript package that can be published to npm.5 This approach treats the Rust/Wasm component as a distinct dependency within a broader JS-centric ecosystem.  
* **Trunk:** In contrast, Trunk is an opinionated bundler specifically designed for building entire web *applications* with Rust and WebAssembly.8 It adopts an asset-centric approach, using an  
  index.html file as the central manifest to drive the entire build process.10  
  Trunk handles not only the compilation of Rust to Wasm but also the bundling of all associated assets, including CSS, SCSS, images, and JavaScript snippets.12 Its integrated development server with live-reloading capabilities (  
  trunk serve) provides a seamless and rapid iteration cycle highly conducive to frontend development.3

#### **1.2.2 Architectural Decision and Justification**

* **Decision:** The project will adopt **Trunk** as its primary build tool and bundler.  
* **Justification:** This decision is rooted in the nature of the project itself. We are building a self-contained, interactive web application, not a library intended for wider distribution. Trunk's application-first philosophy is therefore a more natural fit. It abstracts away the multi-step complexity of a manual cargo build followed by a wasm-bindgen CLI invocation, which wasm-pack helps automate but still conceptually separates.10  
  The selection of Trunk is a strategic choice that prioritizes developer velocity and a streamlined workflow. The traditional web development paradigm, centered around an index.html file, is more intuitive and accessible, particularly for team members who may be more experienced with frontend technologies than with the intricacies of the Rust build system. By unifying the build process into a single, simple command (trunk serve), we reduce cognitive overhead and accelerate the feedback loop between code changes and visual results. This is invaluable for a project that requires fine-tuning of visual and interactive elements. This choice directly shapes the development process into a more agile and integrated experience, fostering closer collaboration between Rust and frontend-focused engineers.

#### **1.2.3 Initial Trunk Configuration**

* **Action:** Install Trunk and its necessary dependency, wasm-bindgen-cli, using Cargo: cargo install trunk wasm-bindgen-cli.3 A root  
  index.html file will be created to serve as the application's entry point. This file will contain a \<canvas\> element for rendering and will link to the Rust application via a \<link data-trunk rel="rust"/\> tag.11 A  
  Trunk.toml file can be added later for more advanced configuration, such as setting up proxies or customizing build outputs.13  
* **Verification:** Executing trunk serve from the project root successfully compiles the starter Rust code, bundles all assets, and serves a functional, albeit blank, web page on http://localhost:8080.

### **1.3 Rendering Architecture Selection**

This is arguably the most significant architectural decision of the project. It determines how the 3D graphics are rendered to the screen and defines the primary boundary between the Rust/Wasm and JavaScript domains.

#### **1.3.1 Analysis of Rendering Options**

* **Option A: Two-Stack Architecture (Rust/Wasm for Logic \+ Three.js for Rendering)**  
  * **Description:** In this model, responsibilities are clearly delineated. The Rust/Wasm module is the computational core, responsible for all physics simulations and mesh deformation logic. On each frame, it calculates the new positions of all vertices in the 3D model. This data, typically as a flat array of floating-point numbers, is then passed across the Wasm-JS bridge. The rendering is handled entirely by a mature, high-level JavaScript 3D library like Three.js. The JavaScript code receives the updated vertex data and uses it to modify a THREE.BufferGeometry object, which Three.js then efficiently renders to the WebGL canvas.14  
  * **Advantages:** This approach dramatically reduces development time and risk by leveraging the extensive, battle-tested feature set of Three.js, which includes scene management, camera controls, lighting, materials, and a highly optimized WebGL renderer.16 It allows the development team to focus its Rust expertise on the unique value proposition of the project—the custom deformation logic—rather than on reinventing fundamental graphics programming concepts. Graphics-related debugging is also simplified, as browser-based tools for inspecting Three.js scenes are readily available.  
  * **Disadvantages:** The primary concern is the performance overhead of transferring vertex data from Wasm to JavaScript on every single frame. This can become a bottleneck if not managed with care, for example, by using shared memory to avoid data copies.18  
* **Option B: Pure Rust Architecture (Rust/Wasm \+ WebGL via web-sys)**  
  * **Description:** This architecture commits to a single-language codebase where the entire application, including the rendering pipeline, is written in Rust. Communication with the browser's graphics API is achieved through the web-sys crate, which provides raw, one-to-one bindings for all Web APIs, including WebGL.2 In this model, the Rust code would be responsible for acquiring a WebGL rendering context from the canvas, compiling GLSL shaders, creating and managing vertex and index buffers, setting up matrix transformations, and executing the final draw calls.20  
  * **Advantages:** The theoretical peak performance could be higher by eliminating the Wasm-JS bridge from the critical render loop. It results in a unified codebase, which can be appealing from a maintenance perspective.  
  * **Disadvantages:** This approach entails a monumental increase in implementation complexity and project scope. It requires the team to build a rudimentary 3D engine from scratch, solving complex problems in linear algebra, shader management, and state management that have already been solved by libraries like Three.js. The development process would be significantly slower and more prone to subtle, hard-to-debug graphical errors.22

#### **1.3.2 Architectural Decision and Justification**

* **Decision:** The project will adopt the **Two-Stack Architecture (Option A)**.  
* **Justification:** This decision is a pragmatic and strategic application of WebAssembly's core value proposition. The purpose of Wasm in the modern web ecosystem is not necessarily to replace JavaScript, but to *augment* it by enabling near-native performance for computationally intensive tasks.7 The CPU-heavy work in this project is the custom physics simulation that drives the mesh deformation. This is a perfect candidate for implementation in Rust.  
  Conversely, the act of rendering via WebGL is primarily a GPU-bound task. While the API calls are made from the CPU, libraries like Three.js are already highly optimized for this, minimizing draw calls and managing GPU state efficiently.17 Re-implementing this low-level graphics plumbing in Rust via  
  web-sys would yield marginal performance benefits at the cost of an enormous development effort. It would be an inefficient allocation of engineering resources.  
  By choosing the Two-Stack model, we are correctly identifying the project's primary bottleneck and applying the most effective tool—Rust and Wasm—to solve it. We then leverage another best-in-class tool, Three.js, for the already-solved problem of rendering. This approach is not a compromise; it is the optimal architecture, reflecting a mature understanding of how to effectively integrate WebAssembly into a modern web application to achieve the best balance of performance, development speed, and maintainability.

#### **1.3.3 Initial Project Scaffolding**

* **Action:** The index.html file will be updated to include a \<canvas id="main-canvas"\> element. A new JavaScript file, main.js, will be created and linked in the HTML. This script will be responsible for setting up the basic Three.js scene: a renderer attached to the canvas, a camera, and a simple scene with a background color.  
* **Verification:** Loading the index.html page in a browser displays the empty Three.js scene rendered onto the canvas, confirming that the basic rendering environment is functional.

### **1.4 "Hello, Wasm\!" \- Initial Integration Test**

This task serves as the foundational end-to-end verification of the entire toolchain, ensuring that a Rust function can be compiled to Wasm and successfully invoked from JavaScript.

#### **1.4.1 Create a Simple Rust Wasm Function**

* **Action:** In the src/main.rs file, a simple public function, such as pub fn add(a: i32, b: i32) \-\> i32, will be created. This function will be annotated with the \#\[wasm\_bindgen\] attribute to signal that it should be exposed to JavaScript.24 The  
  wasm-bindgen crate will be added as a dependency in Cargo.toml.  
* **Dependencies:** wasm-bindgen \= "0.2"

#### **1.4.2 Call the Wasm Function from JavaScript**

* **Action:** In main.js, the generated Wasm module will be imported. Thanks to Trunk, this is as simple as import init, { add } from './pkg/project\_name.js';. The script will then call the init() function to load and instantiate the Wasm module, followed by a call to the exported add function. The result will be logged to the browser's developer console.16  
* **Verification:** Upon loading the page, the developer console displays the correct result of the addition (e.g., WASM says: 2 \+ 3 \= 5). This successful output confirms that the entire pipeline—Rust compilation, Wasm generation via wasm-bindgen, asset bundling via Trunk, and JavaScript interoperability—is correctly configured and functional.

| Table 1.1: Technology Stack Decision Matrix |
| :---- |
| **Component** |
| **Core Logic Language** |
| **Web Compilation Target** |
| **Build Tool / Bundler** |
| **JS/Wasm Interop** |
| **3D Rendering Library** |
| **UI Framework** |

---
