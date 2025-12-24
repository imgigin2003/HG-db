// edgeClick.js
import * as THREE from 'three';

export function createEdgeClickHandler({
  hypergraphData,
  layerGroups,
  camera,
  controls,
  showEdgeInfo
}) {
  return function handleEdgeClick(edgeId, edgeInfo, currentLayerId) {

    if (!hypergraphData) return console.warn("no hypergraphData");

    console.log("Clicked edge:", edgeId, "in layer:", currentLayerId);
    showEdgeInfo(edgeId, edgeInfo);

    const currentLayerGroup = layerGroups[String(currentLayerId)];
    if (!currentLayerGroup) return;

    currentLayerGroup.visible = false;

    const edgeNodes = hypergraphData.layers[String(currentLayerId)]?.[edgeId] || [];
    if (!edgeNodes.length) return;

    let found = null;
    for (const node of edgeNodes) {
      const layersForNode = (hypergraphData.node_to_layers?.[node] || []).map(String);
      const other = layersForNode.filter(l => l !== String(currentLayerId));
      const chosen = other.find(l => layerGroups[l]);
      if (chosen) {
        found = { node, layerId: chosen };
        break;
      }
    }
    if (!found) return;

    const nextLayerGroup = layerGroups[found.layerId];
    if (!nextLayerGroup) return;

    Object.values(layerGroups).forEach(g => { g.visible = false; });
    nextLayerGroup.visible = true;

    // find node label in that layer
    let nodeLabelObj = findNodeLabel(nextLayerGroup, found.node);

    if (!nodeLabelObj) {
      const box = new THREE.Box3().setFromObject(nextLayerGroup);
      const center = box.getCenter(new THREE.Vector3());
      zoomToPoint(center, 1);
      return;
    }

    const worldPos = new THREE.Vector3();
    nodeLabelObj.getWorldPosition(worldPos);

    const offsetUp = 0.25;
    const backDist = 0.6;
    const targetCenter = worldPos.clone();
    const targetPos = new THREE.Vector3(
      targetCenter.x,
      targetCenter.y + offsetUp,
      targetCenter.z + backDist
    );

    animateCamera(camera, controls, targetPos, targetCenter);
  };
}


// helper: find label by text
function findNodeLabel(layerGroup, nodeId) {
  for (const child of layerGroup.children) {
    if (child.element && child.element.textContent === nodeId) return child;

    for (const c2 of child.children || []) {
      if (c2.element && c2.element.textContent === nodeId) return c2;
    }
  }
  return null;
}


// helper: camera animation
function animateCamera(camera, controls, targetPos, targetTarget, duration=900) {
  const startPos = camera.position.clone();
  const startTarget = controls.target.clone();

  let startTime = null;

  function anim(ts) {
    if (!startTime) startTime = ts;
    const t = Math.min((ts - startTime) / duration, 1);
    const ease = t < 0.5 ? 2*t*t : -1 + (4 - 2*t) * t;

    camera.position.lerpVectors(startPos, targetPos, ease);
    controls.target.lerpVectors(startTarget, targetTarget, ease);
    controls.update();

    if (t < 1) requestAnimationFrame(anim);
  }

  requestAnimationFrame(anim);
}
