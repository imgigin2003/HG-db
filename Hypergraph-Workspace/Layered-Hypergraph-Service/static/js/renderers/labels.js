// labels.js
import { CSS2DRenderer, CSS2DObject } from 'three/addons/renderers/CSS2DRenderer.js';
import * as THREE from 'three';
import { clamp, lerp } from '../utils/helper.js';
import { planeSize } from '../scene/plane.js';
import { showNodeInfo } from '../interaction/info.js';

const LABEL_MIN_DIST = 0.14;     
const LABEL_RELAX_ITERS = 18;    

export function createLabelRenderer() {
  const labelRenderer = new CSS2DRenderer();

  const wrapper = document.querySelector(".canvas-wrapper");

  labelRenderer.setSize(wrapper.clientWidth, wrapper.clientHeight);
  labelRenderer.domElement.style.position = 'absolute';
  labelRenderer.domElement.style.top = '0';
  labelRenderer.domElement.style.left = '0';
  labelRenderer.domElement.style.pointerEvents = 'none';
  labelRenderer.domElement.style.zIndex = '10';

  wrapper.appendChild(labelRenderer.domElement);
  return labelRenderer;
}


export function createLabel(text, x, y, z, nodeInfo) {
    const div = document.createElement('div');
    div.className = 'label';
    div.textContent = text;
    div.style.color = 'black';
    div.style.fontSize = '10px';
    div.style.background = 'rgba(255,255,255,0.85)';
    div.style.padding = '1px 3px';
    div.style.borderRadius = '3px';
    div.style.cursor = 'pointer';
    div.style.pointerEvents = 'auto';
    div.addEventListener("click", (event) => {
        event.stopPropagation();
        showNodeInfo(text, nodeInfo);
    });
    const label = new CSS2DObject(div);
    label.position.set(x, y, z);
    return label;
}

      // repel labels slightly in XZ-plane so they don't sit on top of each other
    export function relaxLabels(labels, minDist = LABEL_MIN_DIST, iterations = LABEL_RELAX_ITERS) {
        for (let it = 0; it < iterations; it++) {
          for (let i = 0; i < labels.length; i++) {
            for (let j = i + 1; j < labels.length; j++) {
              const a = labels[i].position;
              const b = labels[j].position;
              const dx = b.x - a.x, dz = b.z - a.z;
              const dist = Math.hypot(dx, dz);
              if (dist > 0 && dist < minDist) {
                const push = (minDist - dist) / 2;
                const nx = dx / dist, nz = dz / dist;
                a.x -= nx * push;
                a.z -= nz * push;
                b.x += nx * push;
                b.z += nz * push;

                
                a.x = clamp(a.x, -planeSize/2 + 0.02, planeSize/2 - 0.02);
                a.z = clamp(a.z, -planeSize/2 + 0.02, planeSize/2 - 0.02);
                b.x = clamp(b.x, -planeSize/2 + 0.02, planeSize/2 - 0.02);
                b.z = clamp(b.z, -planeSize/2 + 0.02, planeSize/2 - 0.02);
              }
            }
          }
        }

        
        for (const lab of labels) lab.position.y = 0.06;
      }
