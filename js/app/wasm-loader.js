import init from '../rust_learning_project.js';

/**
 * Initializes the WebAssembly module.
 * @returns {Promise<any>} A promise that resolves with the initialized Wasm module.
 * @throws {Error} If the Wasm module fails to load.
 */
export async function loadWasm() {
  try {
    const wasm = await init();
    return wasm;
  } catch (err) {
    console.error("Failed to load WASM module", err);
    throw err;
  }
}
