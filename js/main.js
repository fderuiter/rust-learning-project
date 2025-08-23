import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader.js';
import init, { FaceController, detect_faces, apply_grayscale, apply_sepia } from './rust_learning_project.js';

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
  let draggedHandle = null;
  const handles = [];
  const handlesGroup = new THREE.Group();
  scene.add(handlesGroup);
  let originalImageBytes = null;
  let originalTexture = null;
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

    const handleIntersects = raycaster.intersectObjects(handlesGroup.children);
    if (handleIntersects.length > 0) {
        isDragging = true;
        controls.enabled = false;
        draggedHandle = handleIntersects[0].object;
        const intersection = handleIntersects[0].point;
        faceController.on_mouse_down(draggedHandle.userData.vertexIndex, intersection.x, intersection.y, intersection.z);
        return;
    }

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
        if (draggedHandle) {
            draggedHandle.position.copy(intersection);
            faceController.on_mouse_down(draggedHandle.userData.vertexIndex, intersection.x, intersection.y, intersection.z);
        } else {
            faceController.on_mouse_move(intersection.x, intersection.y, intersection.z);
        }
    }
  }

  function onMouseUp() {
    isDragging = false;
    draggedHandle = null;
    controls.enabled = true;
    if (faceController) {
      faceController.on_mouse_up();
    }
  }

  window.addEventListener('mousedown', onMouseDown);
  window.addEventListener('mousemove', onMouseMove);
  window.addEventListener('mouseup', onMouseUp);

  const imageUpload = document.getElementById('image-upload');
  const grayscaleButton = document.getElementById('grayscale-button');
  const sepiaButton = document.getElementById('sepia-button');

  grayscaleButton.addEventListener('click', () => {
      if (originalImageBytes) {
          try {
              const filteredBytes = apply_grayscale(originalImageBytes);
              const texture = new THREE.DataTexture(filteredBytes, faceMesh.material.map.image.width, faceMesh.material.map.image.height, THREE.RGBAFormat);
              texture.needsUpdate = true;
              faceMesh.material.map = texture;
          } catch (e) {
              alert(`Error applying grayscale filter: ${e}`);
              if (originalTexture) {
                  faceMesh.material.map = originalTexture;
                  faceMesh.material.needsUpdate = true;
              }
          }
      }
  });

  sepiaButton.addEventListener('click', () => {
      if (originalImageBytes) {
          try {
              const filteredBytes = apply_sepia(originalImageBytes);
              const texture = new THREE.DataTexture(filteredBytes, faceMesh.material.map.image.width, faceMesh.material.map.image.height, THREE.RGBAFormat);
              texture.needsUpdate = true;
              faceMesh.material.map = texture;
          } catch (e) {
              alert(`Error applying sepia filter: ${e}`);
              if (originalTexture) {
                  faceMesh.material.map = originalTexture;
                  faceMesh.material.needsUpdate = true;
              }
          }
      }
  });

  imageUpload.addEventListener('change', (event) => {
    const file = event.target.files[0];
    if (!file) {
      return;
    }

    if (!file.type.startsWith('image/')) {
      alert('Please upload a valid image file.');
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        originalImageBytes = new Uint8Array(e.target.result);
        const faces = detect_faces(originalImageBytes);
        if (faces.length === 0) {
          alert('No faces detected in the image.');
        }
        console.log('Detected faces:', faces);
      } catch (e) {
        alert(`Error detecting faces: ${e}`);
        return;
      }

      handles.forEach(handle => scene.remove(handle));
      handles.length = 0;

      const imageUrl = URL.createObjectURL(file);
      const image = new Image();
      image.onload = () => {
        faces.forEach(face => {
          const { x1, y1, x2, y2 } = face;

          const landmarks = [
            { x: x1, y: y1 },
            { x: x2, y: y1 },
            { x: x1, y: y2 },
            { x: x2, y: y2 },
            { x: (x1 + x2) / 2, y: (y1 + y2) / 2 },
          ];

          landmarks.forEach(landmark => {
            const ndc = new THREE.Vector2(
              (landmark.x / image.width) * 2 - 1,
              -(landmark.y / image.height) * 2 + 1
            );

            raycaster.setFromCamera(ndc, camera);
            const intersects = raycaster.intersectObject(faceMesh);

            if (intersects.length > 0) {
              const intersection = intersects[0];
              const point = intersection.point;

              let closestVertexIndex = -1;
              let minDistance = Infinity;

              const vertices = faceMesh.geometry.attributes.position;
              const face = intersection.face;
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
                const vertex = new THREE.Vector3().fromBufferAttribute(vertices, closestVertexIndex);
                const sphereGeometry = new THREE.SphereGeometry(0.01, 32, 32);
                const sphereMaterial = new THREE.MeshBasicMaterial({ color: 0xff0000 });
                const sphere = new THREE.Mesh(sphereGeometry, sphereMaterial);
                sphere.position.copy(vertex);
                sphere.userData.vertexIndex = closestVertexIndex;
                handles.push(sphere);
                handlesGroup.add(sphere);
              }
            }
          });
        });
      };
      image.src = imageUrl;

      const texture = new THREE.TextureLoader().load(imageUrl);
      originalTexture = texture;
      faceMesh.material.map = texture;
      faceMesh.material.needsUpdate = true;
    };
    reader.readAsArrayBuffer(file);
  });


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
