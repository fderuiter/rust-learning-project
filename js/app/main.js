import { loadWasm } from './wasm-loader.js';
import { setupThree } from './three-setup.js';
import { loadFaceModel, createRenderLoop } from './face-model.js';
import { setupUIEventListeners } from './ui-events.js';

/**
 * The main entry point for the application.
 * Initializes the Wasm module, sets up the Three.js scene, loads the 3D model,
 * sets up UI event listeners, and starts the render loop.
 */
async function main() {
  try {
    // Load the WebAssembly module.
    const wasm = await loadWasm();
    const { renderer, camera, scene, controls, resizeRendererToDisplaySize } = setupThree();
    const { faceMesh, faceController } = await loadFaceModel(scene);

    setupUIEventListeners({
      camera,
      scene,
      controls,
      faceController,
      faceMesh,
    });

    const render = createRenderLoop(renderer, scene, camera, controls, faceController, faceMesh, wasm, resizeRendererToDisplaySize);
    render();

  } catch (err) {
    console.error("Failed to initialize the application", err);
  }
}

main();
