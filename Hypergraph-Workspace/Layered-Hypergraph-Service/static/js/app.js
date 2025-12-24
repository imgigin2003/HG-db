import * as THREE from 'three';
import { createScene, handleResize } from './core/scene.js';
import { createPlane, planeSize } from './core/plane.js';
import { createLabelRenderer, createLabel , relaxLabels } from './hypergraph/labels.js';
import {  relaxLayerToVenn } from './hypergraph/layout.js';
import {layoutEdges , createEdgeEllipse , createEdgeLabel } from './hypergraph/edges.js'
import { showEdgeInfo } from './hypergraph/info.js';
import {setupDragAndDrop} from './core/events.js'
import {connectRepeatedNodes} from './hypergraph/connectors.js'
import {createEdgeClickHandler } from './hypergraph/edgeClick.js'



// add this at the top of your file
let hypergraphData = null;  // store fetched data
let selectedObjects = [];

//init
const canvas = document.getElementById("threejs");
const { scene, camera, renderer, controls } = createScene(canvas);
const labelRenderer = createLabelRenderer();
handleResize(camera, renderer, labelRenderer);


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
  const positions = selectedObjects.map(o => o.position);
  const cx = positions.reduce((a, p) => a + p.x, 0) / positions.length;
  const cz = positions.reduce((a, p) => a + p.z, 0) / positions.length;

  const maxDist = Math.max(
    ...positions.map(p => Math.hypot(p.x - cx, p.z - cz))
  );

  const rx = maxDist * 1.3;
  const rz = maxDist * 0.9;

  const color = Math.random() * 0xffffff;
  const ellipse = createEdgeEllipse(cx, cz, rx, rz, color);
  scene.add(ellipse);

  console.log("✅ Created hyperedge around:", selectedObjects.map(o => o.userData.type));

  // optional: clear selection
  selectedObjects.forEach(o => {
    o.userData.selected = false;
    o.material.emissive = new THREE.Color(0x000000);
    o.material.emissiveIntensity = 0;
  });
  selectedObjects = [];
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
        const oval = createEdgeEllipse(e.cx, e.cz, e.rx, e.rz);
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

// ------------------- Animation -------------------
function animate() {
  requestAnimationFrame(animate);
  controls.update();
  renderer.render(scene, camera);
  labelRenderer.render(scene, camera);
}

setupDragAndDrop(camera, scene, renderer.domElement, selectedObjects);

animate();
