// plane.js
import * as THREE from 'three';
import { EdgesGeometry, LineBasicMaterial, LineSegments } from 'three';

const planeSize = 3;
const geometry = new THREE.PlaneGeometry(planeSize, planeSize);
const material = new THREE.MeshBasicMaterial({
    color: '#ffcafe',
    side: THREE.DoubleSide,
    transparent: true,
    opacity: 0.7
});
const edgesGeometry = new EdgesGeometry(geometry);
const lineMaterial = new LineBasicMaterial({ color: 0x0000ff });

export function createPlane() {
    const plane = new THREE.Mesh(geometry, material);
    plane.rotation.x = -Math.PI / 2;
    const border = new LineSegments(edgesGeometry.clone(), lineMaterial);
    plane.add(border);
    return plane;
}

export { planeSize };
