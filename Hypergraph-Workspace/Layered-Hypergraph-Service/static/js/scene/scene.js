// scene.js
import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

export function createScene(canvas) {
    const scene = new THREE.Scene();
    scene.background = new THREE.Color("white");

    const camera = new THREE.PerspectiveCamera(75, 1, 0.1, 100);
    camera.position.set(3, 3, 3);
    camera.lookAt(0, 0, 0);

    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambientLight);

    const dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
    dirLight.position.set(5, 10, 7);
    scene.add(dirLight);

    const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
    renderer.setPixelRatio(window.devicePixelRatio || 1);
    

    const controls = new OrbitControls(camera, canvas || renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.08;
    controls.target.set(0, 0, 0);

    return { scene, camera, renderer, controls };
}

