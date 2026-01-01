import * as THREE from "three";
import "../palette.js";
import { getBioIconSprite } from "./biologicalObjects.js";
import { createPlane } from './plane.js';

export let sceneNodes = [];
export let selectableObjects = [];  


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
  // Optional: only process if clicked on canvas
  if (event.target !== canvas) return;

  // Get FRESH rect every click
  const rect = canvas.getBoundingClientRect();

  mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
  mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

  raycaster.setFromCamera(mouse, camera);

  const intersects = raycaster.intersectObjects(selectableObjects, true);

  console.log("Click - Intersects:", intersects.length); // ADD THIS FOR DEBUG

  
  if (intersects.length > 0) {
  let clicked = intersects[0].object;

  // Climb up the hierarchy until we find the actual selectable object (the Sprite or Plane)
  while (clicked.parent && !selectableObjects.includes(clicked)) {
    clicked = clicked.parent;
  }

  // Safety check: if we went too far (e.g. reached scene), abort
  if (!selectableObjects.includes(clicked)) {
    console.log("Clicked on non-selectable object");
    return;
  }

  console.log("Actually selecting:", clicked); // Should now log the Sprite

  // Now proceed with toggle logic on the real object
  clicked.userData.selected = !clicked.userData.selected;

  if (clicked.userData.selected) {
    selectedObjects.push(clicked);

    // Create and show ring if not exists
    if (!clicked.userData.selectionRing) {
      const ring = createSelectionSquare(clicked);
      clicked.userData.selectionRing = ring;
      clicked.add(ring);
    }
    clicked.userData.selectionRing.visible = true;

    // Tint the sprite
    if (clicked.material && clicked.material.color) {
      clicked.userData.originalColor = clicked.material.color.clone();
      clicked.material.color.set(0x88ff88);
    }

  } else {
    // Deselect
    const idx = selectedObjects.indexOf(clicked);
    if (idx !== -1) selectedObjects.splice(idx, 1);

    if (clicked.userData.selectionRing) {
      clicked.userData.selectionRing.visible = false;
    }

    if (clicked.userData.originalColor) {
      clicked.material.color.copy(clicked.userData.originalColor);
    }
  }
}
});


}

function createSelectionSquare(sprite) {
  const size = sprite.scale.x * 1.2; // slightly larger than icon
  const half = size / 2;

  const points = [
    new THREE.Vector3(-half, -half, 0),
    new THREE.Vector3( half, -half, 0),
    new THREE.Vector3( half,  half, 0),
    new THREE.Vector3(-half,  half, 0),
    new THREE.Vector3(-half, -half, 0)
  ];

  const geometry = new THREE.BufferGeometry().setFromPoints(points);

  const material = new THREE.LineBasicMaterial({
    color: 0x00ff00, // neon green
    linewidth: 4,
    transparent: true,
    opacity: 1
  });

  const line = new THREE.Line(geometry, material);
  line.visible = false;

  line.raycast = () => {}; // disable raycasting on the ring

  line.position.z = 0.05; // slightly in front of sprite
  line.renderOrder = 999;  // make sure it renders on top

  return line;
}
