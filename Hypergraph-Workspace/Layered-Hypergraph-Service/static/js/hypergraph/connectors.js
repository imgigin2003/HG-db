import * as THREE from 'three';


export function connectRepeatedNodes(connectorsGroup,nodeToLayers, allNodePositions, opts = {}) {
  const {
    lineColor = 0x666666,
    lineOpacity = 0.9,
    lineWidth = 1,
    layerSpacing = 0.8,
    yOffset = 0.04
  } = opts;

  // Use a single material instance so lines share it
  const mat = new THREE.LineBasicMaterial({ color: lineColor, transparent: true, opacity: lineOpacity });

  // We'll add lines to the global connectorsGroup (so they stay in world coordinates)
  for (const [node, layers] of Object.entries(nodeToLayers)) {
    if (!Array.isArray(layers) || layers.length < 2) continue;

    // sort layers numerically
    const layersSorted = layers.slice().map(l => Number(l)).sort((a,b) => a - b);

    const pts = [];
    const touchedLayers = []; // record which layer indexes this polyline touches
    for (const layerId of layersSorted) {
      const key = `${node}@${layerId}`;
      const p = allNodePositions[key];
      if (!p) continue;
      const y = Number(layerId) * layerSpacing + yOffset;
      pts.push(new THREE.Vector3(p.x, y, p.z));
      touchedLayers.push(Number(layerId));
    }

    if (pts.length >= 2) {
      const geom = new THREE.BufferGeometry().setFromPoints(pts);
      const line = new THREE.Line(geom, mat);

      // Tag the line with the layers it spans — used for visibility toggling
      line.userData.layers = touchedLayers; // e.g. [0,2,5]

      connectorsGroup.add(line);
    }
  }

  return connectorsGroup;
}

