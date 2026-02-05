import { CSS2DObject } from 'three/addons/renderers/CSS2DRenderer.js';

export function createBioLabel(text) {
  const div = document.createElement('div');
  div.className = 'bio-label';
  div.textContent = text;

  div.style.fontSize = '7px';
  div.style.color = '#1f2937'; // dark slate
  div.style.background = 'rgba(255,255,255,0.85)';
  div.style.padding = '2px 6px';
  div.style.borderRadius = '6px';
  div.style.whiteSpace = 'nowrap';
  div.style.pointerEvents = 'none'; // 👈 important
  div.style.userSelect = 'none';

  const label = new CSS2DObject(div);
  return label;
}
