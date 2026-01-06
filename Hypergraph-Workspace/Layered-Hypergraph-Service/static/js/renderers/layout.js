import { clamp , lerp } from '../utils/helper.js';
import { planeSize } from '../scene/plane.js';
import {ensureContains} from '../utils/LayoutUtils.js'

//world variables
const PAIR_CENTER_PUSH = 0.45;  

export function computeSharedNodePos(edgesList, nodeId, explicitSharedNodes) {
  if (!edgesList || edgesList.length === 0) return { x: 0, z: 0 };
  if (edgesList.length === 1) {
    const e = edgesList[0];
    return { x: e.cx, z: e.cz };
  }

  const sharedSet = explicitSharedNodes && explicitSharedNodes[nodeId];
  if (edgesList.length === 2 && sharedSet && sharedSet.size === 2) {
    const [e1, e2] = edgesList;
    return { x: (e1.cx + e2.cx) / 2, z: (e1.cz + e2.cz) / 2 };
  }

  // --- fix: balanced weighting ---
  let wx = 0, wsumx = 0, wz = 0, wsumz = 0;
  for (const e of edgesList) {
    const wX = 1 / (Math.max(e.rx, 0.3) ** 1.4);
    const wZ = 1 / (Math.max(e.rz, 0.3) ** 1.4);
    wx += e.cx * wX; wsumx += wX;
    wz += e.cz * wZ; wsumz += wZ;
  }
  let pos = { x: wx / wsumx, z: wz / wsumz };

  // --- soft projection ---
  for (const e of edgesList) {
    const u = (pos.x - e.cx) / e.rx;
    const v = (pos.z - e.cz) / e.rz;
    const n = u * u + v * v;
    if (n > 1) {
      const scale = 1 / Math.sqrt(n);
      pos.x = lerp(pos.x, e.cx + u * scale * e.rx, 0.6);
      pos.z = lerp(pos.z, e.cz + v * scale * e.rz, 0.6);
    }
  }

  return pos;
}


export function sampleInsideEllipseDeterministic(edge, nodeId) {
  let h = 0;
  for (let i = 0; i < nodeId.length; i++) h = ((h << 5) - h) + nodeId.charCodeAt(i);
  const a = ((h >>> 0) % 3141) / 3141;
  const b = (((h >>> 8) >>> 0) % 2718) / 2718;
  const angle = a * Math.PI * 2;
  const rr = 0.35 + 0.35 * b;
  return {
    x: edge.cx + Math.cos(angle) * edge.rx * rr,
    z: edge.cz + Math.sin(angle) * edge.rz * rr
  };
}




export function relaxLayerToVenn(layer, edgeParamsArray, layerId, iterations = 8, smooth = 0.6) {
  const edgeMap = {};
edgeParamsArray.forEach(e => edgeMap[e.eid] = e);


  const nodeToEdges = {};
  for (const [eid, nodes] of Object.entries(layer)) {
    for (const node of nodes) {
      if (!nodeToEdges[node]) nodeToEdges[node] = [];
      nodeToEdges[node].push(edgeMap[eid]);
    }
  }

  // build list of nodes shared by exactly 2 edges 
  const sharedPairs = [];
  for (const [node, edgesList] of Object.entries(nodeToEdges)) {
    if (edgesList.length === 2) sharedPairs.push({ node, e1: edgesList[0].eid, e2: edgesList[1].eid });
  }

  const nodePositions = {}; 


  // explicitly track shared nodes
const explicitSharedNodes = {};

for (const [edgeId, nodes] of Object.entries(layer)) {
  for (const node of nodes) {
    if (!explicitSharedNodes[node]) {
      explicitSharedNodes[node] = new Set();
    }
    explicitSharedNodes[node].add(edgeId);
  }
}



  for (const node of Object.keys(nodeToEdges)) {
    const edgesList = nodeToEdges[node];
    const key = `${node}@${layerId}`;
    if (edgesList.length === 1) nodePositions[key] = sampleInsideEllipseDeterministic(edgesList[0], node);
    else nodePositions[key] = computeSharedNodePos(edgesList , node ,  explicitSharedNodes );
    
  }

  for (let it = 0; it < iterations; it++) {
    // 1) re-position nodes based on current edges
// 1) re-position nodes based on current edges
for (const node of Object.keys(nodeToEdges)) {
  const key = `${node}@${layerId}`;
  const edgesList = nodeToEdges[node];

  // is this node actually shared? only 2-edge or 3-edge shared nodes
  const isSharedPair = sharedPairs.some(p => p.node === node);
  const isSharedTriple = edgesList.length >= 3; // optionally refine for your logic

  if (isSharedPair || isSharedTriple) {
    // only truly shared nodes use computeSharedNodePos
    const target = computeSharedNodePos(edgesList, node, explicitSharedNodes);
    nodePositions[key].x = lerp(nodePositions[key].x, target.x, 0.8);
    nodePositions[key].z = lerp(nodePositions[key].z, target.z, 0.8);
  } else {
    // everything else stays inside its own ellipse
    const target = sampleInsideEllipseDeterministic(edgesList[0], node);
    nodePositions[key].x = lerp(nodePositions[key].x, target.x, 0.5);
    nodePositions[key].z = lerp(nodePositions[key].z, target.z, 0.5);
  }
}

    

// 2) refit edges from nodes — start after 2 iterations
if (it > 1) {
  for (const e of edgeParamsArray) {
    const nodes = layer[e.eid] || [];
    if (!nodes.length) continue;

    const xs = [], zs = [];
    for (const n of nodes) {
      const key = `${n}@${layerId}`;
      const p = nodePositions[key];
      if (p && Number.isFinite(p.x) && Number.isFinite(p.z)) {
        xs.push(p.x); zs.push(p.z);
      }
    }
    if (!xs.length) continue;

    const minx = Math.min(...xs), maxx = Math.max(...xs);
    const minz = Math.min(...zs), maxz = Math.max(...zs);
    const padding = 0.08;
    const minRx = 0.14, minRz = 0.10;

    const newCx = (minx + maxx) / 2;
    const newCz = (minz + maxz) / 2;
    const nodeCountFactor = 0.06 * Math.max(nodes.length - 1, 1);
    const newRx = Math.max(minRx + nodeCountFactor, (maxx - minx) / 2 + padding);
    const newRz = Math.max(minRz + nodeCountFactor, (maxz - minz) / 2 + padding);

    // --- cap how much ellipse can grow per iteration ---
    e.cx = lerp(e.cx, newCx, smooth);
    e.cz = lerp(e.cz, newCz, smooth);
    e.rx = lerp(e.rx, newRx, Math.min(smooth, 0.4));
    e.rz = lerp(e.rz, newRz, Math.min(smooth, 0.4));

    e.cx = clamp(e.cx, -planeSize/2 + e.rx + 0.02, planeSize/2 - e.rx - 0.02);
    e.cz = clamp(e.cz, -planeSize/2 + e.rz + 0.02, planeSize/2 - e.rz - 0.02);
  }
}

// two-edge shared nodes
for (const pair of sharedPairs) {
  const key = `${pair.node}@${layerId}`;
  const pos = nodePositions[key];
  if (!pos) continue;
  const e1 = edgeMap[pair.e1], e2 = edgeMap[pair.e2];
  if (!e1 || !e2) continue;

  // vector from e1 → e2
  let dx = e2.cx - e1.cx;
  let dz = e2.cz - e1.cz;
  let dist = Math.hypot(dx, dz);
  if (dist < 1e-6) { dx = 1; dz = 0; dist = 1; }

 
  dx /= dist; dz /= dist;

 
  const sep = 0.25; 
  e1.cx = lerp(e1.cx, pos.x - dx * sep, PAIR_CENTER_PUSH);
  e1.cz = lerp(e1.cz, pos.z - dz * sep, PAIR_CENTER_PUSH);
  e2.cx = lerp(e2.cx, pos.x + dx * sep, PAIR_CENTER_PUSH);
  e2.cz = lerp(e2.cz, pos.z + dz * sep, PAIR_CENTER_PUSH);

  // ensure both ellipses still contain the shared node
  ensureContains(e1, pos, 0.08);
  ensureContains(e2, pos, 0.08);
}

  }

  return { nodePositions, edgeParamsArray ,  explicitSharedNodes };
}
