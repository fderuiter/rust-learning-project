import * as THREE from 'three';
import { detect_faces, apply_grayscale, apply_sepia } from '../rust_learning_project.js';

/**
 * Sets up all UI event listeners for the application.
 * @param {{
 *   camera: THREE.PerspectiveCamera,
 *   scene: THREE.Scene,
 *   controls: OrbitControls,
 *   faceController: FaceController,
 *   faceMesh: THREE.Mesh
 * }} options
 */
export function setupUIEventListeners({
  camera,
  scene,
  controls,
  faceController,
  faceMesh,
}) {
  toastr.options = {
    "positionClass": "toast-bottom-right",
  };

  let isDragging = false;
  let draggedHandle = null;
  const handles = [];
  const handlesGroup = new THREE.Group();
  scene.add(handlesGroup);
  let originalImageBytes = null;
  let originalTexture = null;

  const raycaster = new THREE.Raycaster();
  const mouse = new THREE.Vector2();

  function onMouseDown(event) {
    if (!faceMesh) return;

    mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

    raycaster.setFromCamera(mouse, camera);

    const handleIntersects = raycaster.intersectObjects(handlesGroup.children);
    if (handleIntersects.length > 0) {
      isDragging = true;
      controls.enabled = false;
      draggedHandle = handleIntersects[0].object;
      const intersection = handleIntersects[0].point;
      faceController.on_mouse_down(
        draggedHandle.userData.vertexIndex,
        intersection.x,
        intersection.y,
        intersection.z
      );
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
        const vertex = new THREE.Vector3().fromBufferAttribute(
          vertices,
          vertexIndex
        );
        const distance = point.distanceTo(vertex);
        if (distance < minDistance) {
          minDistance = distance;
          closestVertexIndex = vertexIndex;
        }
      }

      if (closestVertexIndex !== -1) {
        isDragging = true;
        controls.enabled = false;
        faceController.on_mouse_down(
          closestVertexIndex,
          point.x,
          point.y,
          point.z
        );
      }
    }
  }

  function onMouseMove(event) {
    if (!isDragging || !faceMesh) return;

    mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

    raycaster.setFromCamera(mouse, camera);
    const plane = new THREE.Plane(new THREE.Vector3(0, 0, 1), 0);
    const intersection = new THREE.Vector3();
    raycaster.ray.intersectPlane(plane, intersection);

    if (intersection) {
      if (draggedHandle) {
        draggedHandle.position.copy(intersection);
        faceController.on_mouse_down(
          draggedHandle.userData.vertexIndex,
          intersection.x,
          intersection.y,
          intersection.z
        );
      } else {
        faceController.on_mouse_move(
          intersection.x,
          intersection.y,
          intersection.z
        );
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
        const texture = new THREE.DataTexture(
          filteredBytes,
          faceMesh.material.map.image.width,
          faceMesh.material.map.image.height,
          THREE.RGBAFormat
        );
        texture.needsUpdate = true;
        faceMesh.material.map = texture;
      } catch (e) {
        toastr.error(`Error applying grayscale filter: ${e}`);
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
        const texture = new THREE.DataTexture(
          filteredBytes,
          faceMesh.material.map.image.width,
          faceMesh.material.map.image.height,
          THREE.RGBAFormat
        );
        texture.needsUpdate = true;
        faceMesh.material.map = texture;
      } catch (e) {
        toastr.error(`Error applying sepia filter: ${e}`);
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
      toastr.error('Please upload a valid image file.');
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      try {
        originalImageBytes = new Uint8Array(e.target.result);
        const faces = detect_faces(originalImageBytes);
        if (faces.length === 0) {
          toastr.info('No faces detected in the image.');
        } else {
          toastr.success(`Detected ${faces.length} face(s) in the image.`);
        }
        console.log('Detected faces:', faces);
      } catch (e) {
        toastr.error(`Error detecting faces: ${e}`);
        return;
      }

      handles.forEach((handle) => scene.remove(handle));
      handles.length = 0;

      const imageUrl = URL.createObjectURL(file);
      const image = new Image();
      image.onload = () => {
        faces.forEach((face) => {
          const { x1, y1, x2, y2 } = face;

          const landmarks = [
            { x: x1, y: y1 },
            { x: x2, y: y1 },
            { x: x1, y: y2 },
            { x: x2, y: y2 },
            { x: (x1 + x2) / 2, y: (y1 + y2) / 2 },
          ];

          landmarks.forEach((landmark) => {
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
                const vertex = new THREE.Vector3().fromBufferAttribute(
                  vertices,
                  vertexIndex
                );
                const distance = point.distanceTo(vertex);
                if (distance < minDistance) {
                  minDistance = distance;
                  closestVertexIndex = vertexIndex;
                }
              }

              if (closestVertexIndex !== -1) {
                const vertex = new THREE.Vector3().fromBufferAttribute(
                  vertices,
                  closestVertexIndex
                );
                const sphereGeometry = new THREE.SphereGeometry(0.01, 32, 32);
                const sphereMaterial = new THREE.MeshBasicMaterial({
                  color: 0xff0000,
                });
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
}
