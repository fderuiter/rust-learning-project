export default async function init() {
  return {
    FaceController: class {},
    detect_faces: () => [],
    apply_grayscale: (bytes) => bytes,
    apply_sepia: (bytes) => bytes,
  };
}
