import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';
import init, { FaceController } from './rust_learning_project.js';

async function run() {
  // Initialize the Wasm module.
  const wasm = await init();

  // Start the Three.js application.
  main(wasm);
}

run();

function main(wasm) {
  const canvas = document.querySelector('#main-canvas');
  const renderer = new THREE.WebGLRenderer({canvas, antialias: true});
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);

  const fov = 75;
  const aspect = window.innerWidth / window.innerHeight;
  const near = 0.1;
  const far = 100;
  const camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
  camera.position.z = 2;

  const controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x333333);

  {
    const color = 0xFFFFFF;
    const intensity = 1;
    const light = new THREE.DirectionalLight(color, intensity);
    light.position.set(-1, 2, 4);
    scene.add(light);
  }
  {
    const color = 0xFFFFFF;
    const intensity = 0.5;
    const light = new THREE.AmbientLight(color, intensity);
    scene.add(light);
  }

  let faceController;
  let faceMesh;
  let isDragging = false;
  const clock = new THREE.Clock();

  const loader = new GLTFLoader();
  loader.load('assets/face.gltf', (gltf) => {
    const model = gltf.scene;
    faceMesh = model.children[0];
    scene.add(model);

    const positions = faceMesh.geometry.attributes.position.array;
    const indices = faceMesh.geometry.index.array;

    console.log('JS vertex count:', positions.length / 3);
    console.log('JS index count:', indices.length);

    faceController = new FaceController(positions, indices);
    console.log('Rust vertex count:', faceController.get_vertex_count());
  });

  const raycaster = new THREE.Raycaster();
  const mouse = new THREE.Vector2();

  function onMouseDown(event) {
    if (!faceMesh) return;

    mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouse.y = - (event.clientY / window.innerHeight) * 2 + 1;

    raycaster.setFromCamera(mouse, camera);
    const intersects = raycaster.intersectObject(faceMesh);

    if (intersects.length > 0) {
      const intersection = intersects[0];
      const face = intersection.face;
      const point = intersection.point;

      let closestVertexIndex = -1;
      let minDistance = Infinity;

      const vertices = faceMesh.geometry.attributes.position;
      const faceIndices = [face.a, face.b, face.c];
      for (const vertexIndex of faceIndices) {
        const vertex = new THREE.Vector3().fromBufferAttribute(vertices, vertexIndex);
        const distance = point.distanceTo(vertex);
        if (distance < minDistance) {
          minDistance = distance;
          closestVertexIndex = vertexIndex;
        }
      }

      if (closestVertexIndex !== -1) {
        isDragging = true;
        controls.enabled = false;
        faceController.on_mouse_down(closestVertexIndex, point.x, point.y, point.z);
      }
    }
  }

  function onMouseMove(event) {
    if (!isDragging || !faceMesh) return;

    mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouse.y = - (event.clientY / window.innerHeight) * 2 + 1;

    raycaster.setFromCamera(mouse, camera);
    const plane = new THREE.Plane(new THREE.Vector3(0, 0, 1), 0);
    const intersection = new THREE.Vector3();
    raycaster.ray.intersectPlane(plane, intersection);

    if (intersection) {
        faceController.on_mouse_move(intersection.x, intersection.y, intersection.z);
    }
  }

  function onMouseUp() {
    isDragging = false;
    controls.enabled = true;
    if (faceController) {
      faceController.on_mouse_up();
    }
  }

  window.addEventListener('mousedown', onMouseDown);
  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', onMouseUp);


  function resizeRendererToDisplaySize(renderer) {
    const canvas = renderer.domElement;
    const width = canvas.clientWidth;
    const height = canvas.clientHeight;
    const needResize = canvas.width !== width || canvas.height !== height;
    if (needResize) {
      renderer.setSize(width, height, false);
    }
    return needResize;
  }

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

  requestAnimationFrame(render);
}
