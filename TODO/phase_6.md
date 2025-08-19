## **Phase 6: Deployment**

The final phase involves preparing the application for production release by optimizing its performance and size, and deploying it to a static hosting service.

### **6.1 Build Optimization**

* **Action:** The final Wasm binary size and performance will be optimized. This involves:  
  1. Building in release mode: trunk build \--release. This enables crucial LLVM optimizations.  
  2. Enabling Link-Time Optimization (LTO) in Cargo.toml to allow for more aggressive cross-crate optimizations.  
  3. Running wasm-opt on the generated Wasm file. Trunk can be configured to do this automatically. This tool from the Binaryen toolkit performs Wasm-specific optimizations that can significantly reduce file size.  
* **Verification:** The final .wasm file in the dist directory is significantly smaller than the development build. Performance profiling in the browser shows fast load times and consistent 60 FPS animation.

### **6.2 Deployment**

* **Action:** The trunk build \--release command generates a self-contained dist directory with the index.html, the optimized .wasm binary, its JS wrapper, and any other assets. This entire directory can be deployed to any static web hosting service (e.g., GitHub Pages, Netlify, Vercel).  
* **Verification:** The application is accessible via a public URL and functions identically to the local development version.

---
