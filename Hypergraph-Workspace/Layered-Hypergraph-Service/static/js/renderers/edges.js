import * as THREE from 'three';
import { CSS2DObject } from 'three/addons/renderers/CSS2DRenderer.js';
import { clamp, lerp } from '../utils/helper.js';
import { planeSize } from '../scene/plane.js';





export function createEdgeEllipse(
  cx,
  cy,
  cz,
  rx,
  ry,
  color = 0x7c3aed,
  { mode = "floor" } = {}
) {
  const shape = new THREE.Shape();
  shape.absellipse(0, 0, rx, ry, 0, Math.PI * 2);

  const fillGeom = new THREE.ShapeGeometry(shape, 64);
  const fillMat = new THREE.MeshBasicMaterial({
    color,
    transparent: true,
    opacity: 0.18,
    side: THREE.DoubleSide,
    depthWrite: false
  });

  const fillMesh = new THREE.Mesh(fillGeom, fillMat);

  const points = new THREE.EllipseCurve(0, 0, rx, ry, 0, Math.PI * 2).getPoints(64);
  const outline = new THREE.LineLoop(
    new THREE.BufferGeometry().setFromPoints(points),
    new THREE.LineBasicMaterial({ color })
  );

  const ellipseGroup = new THREE.Group();
  ellipseGroup.add(fillMesh, outline);

  const labelAnchor = new THREE.Object3D();
  labelAnchor.position.set(0, 0.12, 0);

  const root = new THREE.Group();

  if (mode === "floor") {
    // 🌍 default hyperedges
    ellipseGroup.rotation.x = -Math.PI / 2;
    root.position.set(cx, 0.01, cz);
  } else {
    // 🧬 biological selection hyperedges
    root.position.set(cx, cy, cz + 0.01);
  }

if (mode === "sprite") {
  ellipseGroup.rotation.x = -Math.PI / 2; // lie flat like the objects
}


  root.add(ellipseGroup);
  root.add(labelAnchor);
  root.userData.labelAnchor = labelAnchor;

  return root;
}


 export function createEdgeLabel(edgeId, edgeInfo, layerId , handleEdgeClick) {
  const div = document.createElement("div");
  div.className = "edge-label";
  div.textContent = edgeId;
  div.style.color = 'orange';
  div.style.fontSize = '12px';
  div.style.padding = '2px 4px';
  div.style.borderRadius = '4px';
  div.style.background = 'rgba(0,0,0,0.4)';
  div.style.cursor = 'pointer';
  div.style.pointerEvents = 'auto';

  div.addEventListener("click", (event) => {
    event.stopPropagation();
    console.log("🟠 Edge label clicked:", edgeId);
    handleEdgeClick(edgeId, edgeInfo, layerId );
  });
 
  const label = new CSS2DObject(div);
  return label;
}

export function layoutEdges(layer) {
  const edgeIDs = Object.keys(layer);
  const halfPlane = planeSize / 2 - 0.05;
  const out = [];

  const spacing = 0.28; 
  const positions = [];

  edgeIDs.forEach((eid) => {
    let tries = 0, maxTries = 200;
    let cx = 0, cz = 0;
    let rx = 0.18 + 0.06 * (layer[eid].length - 1);
    let rz = rx * 0.7;

    rx = clamp(rx, 0.15, 0.35);
    rz = clamp(rz, 0.12, 0.28);

    do {
      const angle = Math.random() * Math.PI * 2;
      const r = Math.random() * (halfPlane - Math.max(rx, rz));
      cx = Math.cos(angle) * r;
      cz = Math.sin(angle) * r;

      const overlaps = positions.some(p => {
        const dx = cx - p.cx, dz = cz - p.cz;
        const dist = Math.hypot(dx, dz);
        const combined = Math.max(rx, rz) + Math.max(p.rx, p.rz) + spacing;
        return dist < combined;
      });
      if (!overlaps) break;
      tries++;
    } while (tries < maxTries);

    positions.push({ cx, cz, rx, rz, eid });
    out.push({ eid, cx, cz, rx, rz });
  });

  return out; // array of {eid, cx, cz, rx, rz}
}


