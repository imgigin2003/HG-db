import * as THREE from "three";
import "../data/palette.js";
import { getBioIconSprite } from "../scene/biologicalObjects.js";
import { createPlane } from '../scene/plane.js';
import { separateGroup } from "../scene/grouping.js";



export let sceneNodes = [];
export let selectableObjects = [];  
const undoStack = [];

export function undoLast() {
  const action = undoStack.pop();
  if (!action) return;

  action();
}

function destroyObject(obj, scene) {
  if (!obj) return;

  // Selection ring
  if (obj.userData.selectionRing) {
    obj.remove(obj.userData.selectionRing);
    obj.userData.selectionRing.geometry?.dispose?.();
    obj.userData.selectionRing.material?.dispose?.();
    delete obj.userData.selectionRing;
  }

  // Bio label
  if (obj.userData.label) {
    obj.remove(obj.userData.label);
    obj.userData.label.geometry?.dispose?.();
    obj.userData.label.material?.dispose?.();
    delete obj.userData.label;
  }

  // Remove from scene
  if (obj.parent) {
    scene.remove(obj);
  }

  // Clean userData selection state
  obj.userData.selected = false;
}


export function clearScene(scene) {
  sceneNodes.forEach(obj => {

    if (obj.userData.selectionRing) {
      obj.remove(obj.userData.selectionRing);
      obj.userData.selectionRing.geometry?.dispose?.();
      obj.userData.selectionRing.material?.dispose?.();
      delete obj.userData.selectionRing;
    }

    // 🧹 remove label
    if (obj.userData.label) {
      obj.remove(obj.userData.label);
      obj.userData.label.material?.dispose?.();
      obj.userData.label.geometry?.dispose?.();
      delete obj.userData.label;
    }

    if (obj.parent) obj.parent.remove(obj);
  });

  sceneNodes.length = 0;
  selectableObjects.length = 0;
  undoStack.length = 0;
}


const actionPanel = document.getElementById("object-actions");
const deleteBtn = document.getElementById("delete-btn");
const separateBtn = document.getElementById("separate-btn");

function hideActionPanel() {
  actionPanel.style.display = "none";
}

function showActionPanelAtObject(object, camera, canvas) {
  const v = new THREE.Vector3();
  object.getWorldPosition(v);
  v.project(camera);

  const rect = canvas.getBoundingClientRect();
  const x = (v.x * 0.5 + 0.5) * rect.width + rect.left;
  const y = (-v.y * 0.5 + 0.5) * rect.height + rect.top;

  actionPanel.style.left = `${x + 12}px`;
  actionPanel.style.top = `${y - 12}px`;
  actionPanel.style.display = "flex";
}





export function setupDragAndDrop(camera, scene, canvas, selectedObjects , controls) {
  let draggedItem = null;
  const raycaster = new THREE.Raycaster();
  const mouse = new THREE.Vector2();
  const planeNormal = camera.getWorldDirection(new THREE.Vector3());
const dropPlane = new THREE.Plane();


const interaction = {
  dragging: false,
  dragTarget: null
};




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
  canvas.setAttribute("draggable", "false");
canvas.addEventListener("dragstart", e => e.preventDefault());

  canvas.addEventListener("dragover", e => e.preventDefault());

  // --- DROP ---
canvas.addEventListener("drop", async e => {
  e.preventDefault();
  if (!draggedItem) return;

  // mouse → NDC
  const rect = canvas.getBoundingClientRect();
  mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
  mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;

  raycaster.setFromCamera(mouse, camera);

  dropPlane.setFromNormalAndCoplanarPoint(
    planeNormal,
    new THREE.Vector3(0, 0, 0)
  );

  const pos = new THREE.Vector3();
  raycaster.ray.intersectPlane(dropPlane, pos);

  let objectToAdd = null;

  if (draggedItem.type === "plane") {
    objectToAdd = createPlane();
    selectableObjects.push(objectToAdd);

  } else if (draggedItem.src) {
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
  
  undoStack.push(() => {
  destroyObject(objectToAdd, scene);

  const i1 = sceneNodes.indexOf(objectToAdd);
  if (i1 !== -1) sceneNodes.splice(i1, 1);

  const i2 = selectableObjects.indexOf(objectToAdd);
  if (i2 !== -1) selectableObjects.splice(i2, 1);
});

}


  draggedItem = null;
});

canvas.addEventListener("mousedown", e => {
if (
    !actionPanel.contains(e.target) &&
    e.target !== canvas
  ) {
    hideActionPanel();
  }
  const rect = canvas.getBoundingClientRect();
  mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
  mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;

  raycaster.setFromCamera(mouse, camera);
  const hits = raycaster.intersectObjects(selectableObjects, true);
  if (!hits.length) return;

let hitObject = hits[0].object;

// climb to first selectable (item OR group)
while (hitObject.parent && !selectableObjects.includes(hitObject)) {
  hitObject = hitObject.parent;
}

// 🔑 DECIDE DRAG TARGET
let dragTarget = hitObject;

// if item is inside a group → drag the group
if (hitObject.userData.isInGroup && hitObject.userData.groupRoot) {
  dragTarget = hitObject.userData.groupRoot;
}

interaction.dragTarget = dragTarget;
interaction.dragging = true;
controls.enabled = false;

camera.getWorldDirection(planeNormal);
dropPlane.setFromNormalAndCoplanarPoint(
  planeNormal,
  interaction.dragTarget.position
);


});


canvas.addEventListener("mousemove", e => {
  if (!interaction.dragging || !interaction.dragTarget) return;

  const rect = canvas.getBoundingClientRect();
  mouse.x = ((e.clientX - rect.left) / rect.width) * 2 - 1;
  mouse.y = -((e.clientY - rect.top) / rect.height) * 2 + 1;

  raycaster.setFromCamera(mouse, camera);

  const pos = new THREE.Vector3();
  if (raycaster.ray.intersectPlane(dropPlane, pos)) {
    interaction.dragTarget.position.copy(pos);
  }
});


window.addEventListener("mouseup", () => {
  if (!interaction.dragging) return;

  interaction.dragging = false;
  interaction.dragTarget = null;
  controls.enabled = true;
});


deleteBtn.addEventListener("click", () => {
  if (selectedObjects.length !== 1) return;

  const target = selectedObjects[0];
  const visualTarget =
    target.userData.isGrouped && target.userData.parentGroup
      ? target.userData.parentGroup
      : target;

  // Remove selection ring
if (visualTarget.userData.selectionRing) {
  visualTarget.remove(visualTarget.userData.selectionRing);
  visualTarget.userData.selectionRing.geometry?.dispose?.();
  visualTarget.userData.selectionRing.material?.dispose?.();
  delete visualTarget.userData.selectionRing;
}

// 🧹 Remove bio label if exists
if (visualTarget.userData.label) {
  visualTarget.remove(visualTarget.userData.label);

  visualTarget.userData.label.material?.dispose?.();
  visualTarget.userData.label.geometry?.dispose?.();

  delete visualTarget.userData.label;
}

destroyObject(visualTarget, scene);


  // Cleanup arrays
  selectableObjects = selectableObjects.filter(o => o !== visualTarget);
  sceneNodes = sceneNodes.filter(o => o !== visualTarget);

  selectedObjects.length = 0;
  hideActionPanel();
});

separateBtn.addEventListener("click", () => {
  if (selectedObjects.length !== 1) return;

  const target = selectedObjects[0];
  const group =
    target.userData.isGroup
      ? target
      : target.userData.parentGroup;

  if (!group?.userData?.isGroup) return;

  separateGroup(group, scene);

  // Cleanup
  if (group.userData.selectionRing) {
    group.remove(group.userData.selectionRing);
  }

  selectableObjects = selectableObjects.filter(o => o !== group);
  sceneNodes = sceneNodes.filter(o => o !== group);

  selectedObjects.length = 0;
  hideActionPanel();
});


window.addEventListener("dblclick", event => {
  if (event.target !== canvas) return;

  const rect = canvas.getBoundingClientRect();

  mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
  mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

  raycaster.setFromCamera(mouse, camera);

  // 🔑 IMPORTANT: ignore Groups entirely
  const intersects = raycaster.intersectObjects(
    selectableObjects.filter(o => !o.userData?.isGroup),
    true
  );

  if (!intersects.length) return;

  let clicked = intersects[0].object;

  // climb until real selectable (sprite / plane)
  while (
    clicked.parent &&
    clicked.parent !== scene &&
    !selectableObjects.includes(clicked)
  ) {
    clicked = clicked.parent;
  }

  if (!selectableObjects.includes(clicked)) return;

  // toggle
  clicked.userData.selected = !clicked.userData.selected;

  const visualTarget =
    clicked.userData.isGrouped && clicked.userData.parentGroup
      ? clicked.userData.parentGroup
      : clicked;

  if (clicked.userData.selected) {
    if (!selectedObjects.includes(clicked)) {
      selectedObjects.push(clicked);
    }

    if (!visualTarget.userData.selectionRing) {
      const ring = createSelectionSquare(visualTarget);
      visualTarget.userData.selectionRing = ring;
      visualTarget.add(ring);
    }

    visualTarget.userData.selectionRing.visible = true;

    if (clicked.material?.color) {
      clicked.userData.originalColor =
        clicked.material.color.clone();
      clicked.material.color.set(0x88ff88);
    }
  } else {
    const idx = selectedObjects.indexOf(clicked);
    if (idx !== -1) selectedObjects.splice(idx, 1);

    if (visualTarget.userData.selectionRing) {
      visualTarget.userData.selectionRing.visible = false;
    }

    if (clicked.userData.originalColor) {
      clicked.material.color.copy(
        clicked.userData.originalColor
      );
      delete clicked.userData.originalColor;
    }
  }

  // After selection toggle logic
if (selectedObjects.length === 1) {
  const selected = selectedObjects[0];

  const visualTarget =
    selected.userData.isGrouped && selected.userData.parentGroup
      ? selected.userData.parentGroup
      : selected;

  showActionPanelAtObject(visualTarget, camera, canvas);

  // Only show separate if it's a group
  separateBtn.style.display = visualTarget.userData.isGroup
    ? "inline-block"
    : "none";
} else {
  hideActionPanel();
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
