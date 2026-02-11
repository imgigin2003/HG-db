import * as THREE from 'three';
import { createScene } from './scene/scene.js';
import { createPlane, planeSize } from './scene/plane.js';
import { createLabelRenderer, createLabel , relaxLabels } from './renderers/labels.js';
import {  relaxLayerToVenn } from './renderers/sharedNodes.js';
import {layoutEdges , createEdgeEllipse , createEdgeLabel } from './renderers/edges.js'
import { showEdgeInfo } from './interaction/info.js';
import {setupDragAndDrop , sceneNodes , selectableObjects ,undoLast, clearScene} from './interaction/events.js'
import {connectRepeatedNodes} from './renderers/connectors.js'
import {createEdgeClickHandler } from './interaction/edgeClick.js'
import { exportScene, importScene } from "./data/hypergraphIO.js";
import { getBioIconSprite  } from "./scene/biologicalObjects.js";
import { createGroupFromObjects } from "./scene/grouping.js";





// add this at the top of your file
let hypergraphData = null;  // store fetched data
let selectedObjects = [];

//init
const canvas = document.getElementById("threejs");
const { scene, camera, renderer, controls } = createScene(canvas);
const labelRenderer = createLabelRenderer();
function resizeToCanvasWrapper() {
  const wrapper = document.querySelector(".canvas-wrapper");
  if (!wrapper) return;

  const width = wrapper.clientWidth;
  const height = wrapper.clientHeight;

  camera.aspect = width / height;
  camera.updateProjectionMatrix();

  renderer.setSize(width, height, false);
  labelRenderer.setSize(width, height);
}

window.addEventListener("resize", resizeToCanvasWrapper);
resizeToCanvasWrapper();




// ------------------- Geometry / Materials -------------------

const layerGroups = {}; 
const connectorsGroup = new THREE.Group();
connectorsGroup.name = 'connectors';
scene.add(connectorsGroup);




  document.getElementById("create-hyperedge-btn").addEventListener("click", () => {
  if (selectedObjects.length === 0) {
    alert("Select some objects first!");
    return;
  }


  // compute bounding ellipse
  const positions = selectedObjects.map(o => {
  const v = new THREE.Vector3();
  o.getWorldPosition(v);
  return v;
});
console.log("🔎 Selected objects world positions:");
selectedObjects.forEach((o, i) => {
  const v = new THREE.Vector3();
  o.getWorldPosition(v);
  console.log(`  obj ${i}:`, {
    x: v.x.toFixed(3),
    y: v.y.toFixed(3),
    z: v.z.toFixed(3),
    parent: o.parent?.name || o.parent?.type
  });
});

 function computeEllipseXZ(points) {
  let minX = Infinity, maxX = -Infinity;
  let minZ = Infinity, maxZ = -Infinity;

  points.forEach(p => {
    minX = Math.min(minX, p.x);
    maxX = Math.max(maxX, p.x);
    minZ = Math.min(minZ, p.z);
    maxZ = Math.max(maxZ, p.z);
  });

  const cx = (minX + maxX) / 2;
  const cz = (minZ + maxZ) / 2;

  const rx = (maxX - minX) / 2;
  const rz = (maxZ - minZ) / 2;

  return { cx, cz, rx, rz };
}


const { cx, cz, rx, rz } = computeEllipseXZ(positions);

// vertical placement = average Y
const cy =
  positions.reduce((a, p) => a + p.y, 0) / positions.length;



  const color = Math.random() * 0xffffff;
  const ellipse = createEdgeEllipse(
  cx,
  cy,
  cz,
  rx * 1.2,   
  rz * 1.2,
  color,
  { mode: "sprite" }
);



  const group = createGroupFromObjects(selectedObjects, scene);

// 🔑 keep members selectable
group.userData.members.forEach(member => {
  if (!selectableObjects.includes(member)) {
    selectableObjects.push(member);
  }
});


  group.worldToLocal(ellipse.position);
  group.add(ellipse);

ellipse.userData.isHyperedge = true;
selectableObjects.push(group);
sceneNodes.push(group);


  const ev = new THREE.Vector3();
ellipse.getWorldPosition(ev);

console.log("⭕ Ellipse world position:", {
  x: ev.x.toFixed(3),
  y: ev.y.toFixed(3),
  z: ev.z.toFixed(3),
  parent: ellipse.parent?.name || ellipse.parent?.type
});


  console.log("✅ Created hyperedge around:", selectedObjects.map(o => o.userData.type));

  // optional: clear selection
 // === PROPER DESELECTION ===
  selectedObjects.forEach(obj => {
    obj.userData.selected = false;

    if (obj.userData.selectionRing) {
      obj.userData.selectionRing.visible = false;
    }

    if (obj.userData.originalColor) {
      obj.material.color.copy(obj.userData.originalColor);
      delete obj.userData.originalColor;
    }
  });

  selectedObjects.length = 0; // clear the array
});





const DEBUG_SHOW_NODES = false;




fetch('/api/hypergraph')
  .then(r => r.json())
  .then(data => {
    if (!data?.layers) { return; }
     
    hypergraphData = data;

      const handleEdgeClick = createEdgeClickHandler({
      hypergraphData,
      layerGroups,
      camera,
      controls,
      showEdgeInfo
    });
    const layers = data.layers;

    const allNodePositions = {};

    Object.keys(layers).forEach((layerId) => {
      const layerNum = parseInt(layerId, 10);

      const layerGroup = new THREE.Group();
      layerGroup.position.y = layerNum * 0.8;
      scene.add(layerGroup);

      layerGroups[layerId] = layerGroup;

     
      const layerList = document.getElementById("layer-list");

// create layer row
const layerRow = document.createElement("div");
layerRow.style.display = "flex";
layerRow.style.alignItems = "center";
layerRow.style.justifyContent = "space-between";
layerRow.style.marginBottom = "4px";

// layer label
const name = document.createElement("span");
name.textContent = `Layer ${layerId}`;
name.style.marginRight = "6px";

// eye icon
const eye = document.createElement("span");
eye.textContent = "👁️";
eye.style.cursor = "pointer";
eye.dataset.visible = "true"; // track state

eye.addEventListener("click", () => {
  const wasVisible = eye.dataset.visible === "true";
  const nowVisible = !wasVisible;

  // toggle the layer group
  layerGroups[layerId].visible = nowVisible;

  // update icon + state
  eye.dataset.visible = String(nowVisible);
  eye.textContent = nowVisible ? "👁️" : "🚫";

  // Update connector visibility:
  // Hide any connector that touches a hidden layer.
  // (If any of the connector's layers is hidden, hide the connector.)
  for (const child of connectorsGroup.children) {
    const touched = child.userData?.layers;
    if (!Array.isArray(touched) || touched.length === 0) continue;
    // If any touched layer is currently hidden, hide the line.
    let shouldBeVisible = true;
    for (const lid of touched) {
      // layerGroups keys are strings in your code; normalize both to string
      const lg = layerGroups[String(lid)];
      if (!lg || lg.visible === false) {
        shouldBeVisible = false;
        break;
      }
    }
    child.visible = shouldBeVisible;
  }
});


// append
layerRow.appendChild(name);
layerRow.appendChild(eye);
layerList.appendChild(layerRow);

      const plane = createPlane();
      layerGroup.add(plane);

      const layer = layers[layerId];

      const edgeParamsArray = layoutEdges(layer);


      const { nodePositions, edgeParamsArray: finalEdges  , explicitSharedNodes } = relaxLayerToVenn(layer, edgeParamsArray, layerId, 8, 0.6);

         for (const [key, pos] of Object.entries(nodePositions)) {
      
      allNodePositions[key] = { x: pos.x, z: pos.z, layer: Number(layerId) };
    }


      const edgeToOval = {};

      finalEdges.forEach(e => {
        const oval = createEdgeEllipse(
  e.cx,     // X center
  0,        // Y on floor
  e.cz,     // Z center
  e.rx,     // radius along X
  e.rz,     // radius along Z
  undefined, // color optional
  { mode: "floor" }
);

        layerGroup.add(oval);
        edgeToOval[e.eid] = oval;

          const edgeLabel = createEdgeLabel(e.eid, e, layerId , handleEdgeClick);

         
         edgeLabel.position.set(0, 0.06, 0);
         oval.add(edgeLabel);


         oval.userData.edgeId = e.eid;
  oval.userData.layerId = layerId;

  oval.cursor = 'pointer';
  oval.onClick = () => handleEdgeClick(e.eid, e, layerId);

     

      });

     // Add node labels + collect for relaxation
const labelObjs = [];
const lockedLabels = new Set();

for (const [key, pos] of Object.entries(nodePositions)) {
  const nodeId = key.split('@')[0];
  const sharedCount = explicitSharedNodes[nodeId] ? explicitSharedNodes[nodeId].size : 1;
  const nodeInfo = {
    x: pos.x.toFixed(3),
    z: pos.z.toFixed(3),
    layer: pos.layer
  };

  const label = createLabel(nodeId, pos.x, 0.02, pos.z , nodeInfo);
  layerGroup.add(label);


  if (sharedCount > 1) {
    
    lockedLabels.add(label);
  } else {
    labelObjs.push(label);
  }


  if (DEBUG_SHOW_NODES) {
    const sgeo = new THREE.SphereGeometry(0.03, 8, 8);
    const smat = new THREE.MeshBasicMaterial({ color: 0x111111 });
    const mesh = new THREE.Mesh(sgeo, smat);
    mesh.position.set(pos.x, 0.04, pos.z);
    layerGroup.add(mesh);
  }
}


if (labelObjs.length) relaxLabels(labelObjs);




      if (labelObjs.length) relaxLabels(labelObjs);

      if (data.node_to_layers) {
    connectRepeatedNodes(connectorsGroup, data.node_to_layers, allNodePositions, { lineColor: 0x444444, lineOpacity: 0.9 });
    for (const child of connectorsGroup.children) {
    const touched = child.userData?.layers;
    if (!Array.isArray(touched) || touched.length === 0) continue;
    let visible = true;
    for (const lid of touched) {
      const lg = layerGroups[String(lid)];
      if (!lg || lg.visible === false) { visible = false; break; }
    }
    child.visible = visible;
  }
  }
    });
  })
  .catch(err => { console.error(err); });


document.querySelectorAll(".section-header").forEach(btn => {
  btn.addEventListener("click", () => {
    const body = btn.nextElementSibling;
    body.style.display = body.style.display === "none" ? "block" : "none";
  });
});


// ------------------- Animation -------------------
function animate() {
  requestAnimationFrame(animate);
  controls.update();
  renderer.render(scene, camera);
  labelRenderer.render(scene, camera);
}

setupDragAndDrop(camera, scene, renderer.domElement, selectedObjects , controls);

  document.getElementById("undo-btn").onclick = () => {
  undoLast();
};

document.getElementById("clear-btn").onclick = () => {
  clearScene(scene);
};


// EXPORT
document.getElementById("export-btn").addEventListener("click", () => {
  exportScene({ sceneNodes });
});

// IMPORT
document.getElementById("import-btn").addEventListener("click", () => {
  const input = document.createElement("input");
  input.type = "file";
  input.accept = ".json";

  input.onchange = async () => {
    const file = input.files[0];
    if (!file) return;

    await importScene({
      file,
      addNode: async ({ type, position }) => {
        const sprite = await getBioIconSprite({
          type,
          src: `/static/palette-data/icons/genetics/${type}.svg`
        });

        sprite.position.set(position.x, position.y, position.z);
        scene.add(sprite);
        sceneNodes.push(sprite);
        selectableObjects.push(sprite);
      }
    });
  };

  input.click();
});



animate();
