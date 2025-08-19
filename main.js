import * as THREE from 'three';
import init, { add } from './rust_learning_project.js';

async function run() {
  // Initialize the Wasm module.
  await init();

  // Call the Wasm function and log the result.
  const result = add(2, 3);
  console.log(`Hello from Wasm! 2 + 3 = ${result}`);

  // Start the Three.js application.
  main();
}

run();

function main() {
  const canvas = document.querySelector('#main-canvas');
  const renderer = new THREE.WebGLRenderer({canvas});

  const fov = 75;
  const aspect = 2;  // the canvas default
  const near = 0.1;
  const far = 5;
  const camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
  camera.position.z = 2;

  const scene = new THREE.Scene();

  function render(time) {
    time *= 0.001;  // convert time to seconds

    renderer.render(scene, camera);

    requestAnimationFrame(render);
  }

  requestAnimationFrame(render);
}
