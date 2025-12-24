import * as THREE from 'three';
import { clamp, lerp } from './helper.js';

export function ensureContains(e, pos, padding = 0.06, planeSize = 1) {
  const dx = pos.x - e.cx;
  const dz = pos.z - e.cz;
  const u = dx / e.rx;
  const v = dz / e.rz;
  const n = u * u + v * v;
  if (n > 1) {
    const scale = Math.sqrt(n) + 0.06;
    e.rx *= scale;
    e.rz *= scale;
  }
  
  e.rx = Math.max(e.rx, 0.14);
  e.rz = Math.max(e.rz, 0.10);
  e.cx = clamp(e.cx, -planeSize/2 + e.rx + 0.02, planeSize/2 - e.rx - 0.02);
  e.cz = clamp(e.cz, -planeSize/2 + e.rz + 0.02, planeSize/2 - e.rz - 0.02);
}

