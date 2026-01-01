import * as THREE from "three";
import "../palette.js";
import { selectableObjects, getBioIconSprite } from "./biologicalObjects.js";
import { createPlane } from './plane.js';

export let sceneNodes = [];

export function setupDragAndDrop(camera, scene, canvas, selectedObjects) {
  let draggedItem = null;
  const raycaster = new THREE.Raycaster();
  const mouse = new THREE.Vector2();

  // --- DRAG START ---
  document.addEventListener("dragstart", e => {
    const icon = e.target.closest(".palette-icon");
    if (!icon) return;

    draggedItem = {
      type: icon.dataset.type,
      src: icon.dataset.src || null
    };
    e.dataTransfer.setData("text/plain", "bio-icon");
    console.log("Dragging:", draggedItem);
  });

  // --- DRAG OVER ---
  canvas.addEventListener("dragover", e => e.preventDefault());

  // --- DROP ---
canvas.addEventListener("drop", async e => {
  e.preventDefault();
  if (!draggedItem) return;

  // convert mouse coordinates
  const rect = canvas.getBoundingClientRect();
  mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
  mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;

  // drop on y=0 plane
  const planeRay = new THREE.Plane(new THREE.Vector3(0, 1, 0), 0);
  const pos = new THREE.Vector3();
  raycaster.setFromCamera(mouse, camera);
  raycaster.ray.intersectPlane(planeRay, pos);

  let objectToAdd = null;

  if (draggedItem.type === "plane") {
    objectToAdd = createPlane();
    selectableObjects.push(objectToAdd); 
  } else if (draggedItem.src) {
    // existing biological icon code
    try {
      const sprite = await getBioIconSprite({
        type: draggedItem.type,
        src: draggedItem.src
      });
      const square = createSelectionSquare(sprite);
      sprite.userData.selectionRing = square;
      sprite.add(square);
      objectToAdd = sprite;
      selectableObjects.push(sprite);
    } catch (err) {
      console.error("Icon load failed:", err);
    }
  }

  if (objectToAdd) {
    objectToAdd.position.copy(pos);
    scene.add(objectToAdd);
    sceneNodes.push(objectToAdd);
    console.log(`${draggedItem.type} added at`, pos);
  }

  draggedItem = null;
});
  // --- CLICK SELECTION ---
  window.addEventListener("click", event => {
    mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
    mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

    raycaster.setFromCamera(mouse, camera);
    const intersects = raycaster.intersectObjects(selectableObjects, false);

    if (intersects.length > 0) {
      const clicked = intersects[0].object;
      clicked.userData.selected = !clicked.userData.selected;

      if (clicked.userData.selected) {
        selectedObjects.push(clicked);
        if (clicked.userData.selectionRing)
          clicked.userData.selectionRing.visible = true;
      } else {
        const idx = selectedObjects.indexOf(clicked);
        if (idx !== -1) selectedObjects.splice(idx, 1);
        if (clicked.userData.selectionRing)
          clicked.userData.selectionRing.visible = false;
      }
    }
  });

  function createSelectionSquare(sprite) {
  const size = sprite.scale.x * 1.1; // slightly larger than icon
  const half = size / 2;

  const points = [
    new THREE.Vector3(-half, -half, 0),
    new THREE.Vector3( half, -half, 0),
    new THREE.Vector3( half,  half, 0),
    new THREE.Vector3(-half,  half, 0),
    new THREE.Vector3(-half, -half, 0)
  ];

  const geometry = new THREE.BufferGeometry().setFromPoints(points);

  const material = new THREE.LineDashedMaterial({
    color: 0x00ff88,
    dashSize: size * 0.08,
    gapSize: size * 0.05,
    transparent: true,
    opacity: 0.9
  });

  const line = new THREE.Line(geometry, material);
  line.computeLineDistances();
  line.visible = false;

  // 👇 THIS is the key
  line.position.z = 0.01;   // move in front of sprite
  line.renderOrder = 2;

  return line;
}

}
