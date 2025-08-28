import * as THREE from 'three';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';
import { FaceController } from '../rust_learning_project.js';

/**
 * Loads the 3D face model and initializes the Wasm FaceController.
 * @param {THREE.Scene} scene The Three.js scene to add the model to.
 * @returns {Promise<{faceMesh: THREE.Mesh, faceController: FaceController}>} A promise that resolves with the face mesh and the face controller.
 */
export async function loadFaceModel(scene) {
  const loader = new GLTFLoader();
  const gltf = await loader.loadAsync('assets/face.gltf');

  const model = gltf.scene;
  const faceMesh = model.children[0];
  scene.add(model);

  const positions = faceMesh.geometry.attributes.position.array;
  const indices = faceMesh.geometry.index.array;

  console.log('JS vertex count:', positions.length / 3);
  console.log('JS index count:', indices.length);

  const faceController = new FaceController(positions, indices);
  console.log('Rust vertex count:', faceController.get_vertex_count());

  return { faceMesh, faceController };
}

/**
 * Creates the render loop for the application.
 * @param {THREE.WebGLRenderer} renderer The Three.js renderer.
 * @param {THREE.Scene} scene The Three.js scene.
 * @param {THREE.PerspectiveCamera} camera The Three.js camera.
 * @param {OrbitControls} controls The OrbitControls instance.
 * @param {FaceController} faceController The Wasm face controller.
 * @param {THREE.Mesh} faceMesh The face mesh.
 * @param {any} wasm The initialized Wasm module.
 * @param {(renderer: THREE.WebGLRenderer) => boolean} resizeRendererToDisplaySize The function to resize the renderer.
 * @returns {() => void} The render function.
 */
export function createRenderLoop(renderer, scene, camera, controls, faceController, faceMesh, wasm, resizeRendererToDisplaySize) {
    const clock = new THREE.Clock();

    function render() {
        const deltaTime = clock.getDelta();

        if (resizeRendererToDisplaySize(renderer)) {
            const canvas = renderer.domElement;
            camera.aspect = canvas.clientWidth / canvas.clientHeight;
            camera.updateProjectionMatrix();
        }

        if (faceController) {
            faceController.tick(deltaTime);

            const vertexBufferPtr = faceController.get_vertex_buffer_ptr();
            const vertexCount = faceController.get_vertex_count();
            const wasmMemory = wasm.memory.buffer;
            const wasmVertexBuffer = new Float32Array(wasmMemory, vertexBufferPtr, vertexCount * 3);

            faceMesh.geometry.attributes.position.array.set(wasmVertexBuffer);
            faceMesh.geometry.attributes.position.needsUpdate = true;
        }

        controls.update();
        renderer.render(scene, camera);

        requestAnimationFrame(render);
    }

    return render;
}
