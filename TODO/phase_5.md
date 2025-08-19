### **5.1 Testing Strategy**

The project's code can be divided into two domains: pure, algorithmic Rust code and the boundary-crossing Wasm interface code. Each domain requires a different testing methodology to be effective. The pure logic can be tested natively, which is fast and provides a rich debugging experience. The interface code must be tested in its target environment—a browser—to validate the interoperation with JavaScript.

### **5.2 Tier 1: Native Rust Unit Tests**

* **Action:** For all core logic that does not depend on web APIs—such as the vector math library, the spring force calculations, and the numerical integrator—standard Rust unit tests will be written. These tests will be placed in mod tests {... } blocks alongside the code they are testing and will use the standard \#\[test\] attribute.  
* **Execution:** These tests are run with the simple cargo test command.  
* **Benefits:** This approach is extremely fast, allows for the use of standard Rust debugging tools, and provides immediate feedback during development without needing to compile to Wasm or run a browser.  
* **Verification:** The cargo test command passes for all modules containing pure logic. Test coverage for the physics module exceeds 90%.

### **5.3 Tier 2: Wasm Integration and E2E Tests**

* **Action:** To test the code that interacts with JavaScript, the wasm-bindgen-test framework will be used.41 Tests will be written in Rust files under the  
  tests directory and will use the \#\[wasm\_bindgen\_test\] attribute. These tests will specifically target the public API of the FaceController, simulating calls from JavaScript and asserting the correctness of the results.  
* **Execution:** These tests are run using wasm-pack test \--headless \--firefox or \--chrome.42 This command compiles the tests to Wasm, generates the necessary JS test runner, and executes them in a headless browser environment, reporting the results to the console.  
* **Benefits:** This is the only way to reliably test the Wasm-JS boundary. It verifies that data types are correctly marshaled between the two environments, that JS exceptions can be handled, and that the code behaves as expected when running as actual WebAssembly in a browser context.44  
* **Verification:** The wasm-pack test command completes successfully. Specific tests that pass a vertex buffer to the constructor, call the tick method, and then read the resulting buffer back confirm that the data is being correctly modified and accessed across the boundary.

| Table 5.1: Testing Strategy Matrix |
| :---- |
| **Tier** |
| **Tier 1: Unit** |
| **Tier 2: Integration** |

---
