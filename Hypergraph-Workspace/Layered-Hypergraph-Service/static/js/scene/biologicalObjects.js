// biologicalObjects.js
import * as THREE from "three";




export function getBioIconSprite({ type, src }, size = 0.12) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = "anonymous";
    img.src = src;

    img.onload = () => {
      const SCALE = 4; // 👈 increase for sharper icons (3–6 is sweet spot)

      const canvas = document.createElement("canvas");
      canvas.width = img.width * SCALE;
      canvas.height = img.height * SCALE;

      const ctx = canvas.getContext("2d");
      ctx.drawImage(img, 0, 0, canvas.width, canvas.height);

      const texture = new THREE.CanvasTexture(canvas);
      texture.colorSpace = THREE.SRGBColorSpace;
      texture.minFilter = THREE.LinearFilter;
      texture.magFilter = THREE.LinearFilter;
      texture.anisotropy = 8;

      const material = new THREE.SpriteMaterial({
        map: texture,
        transparent: true
      });

      const sprite = new THREE.Sprite(material);

      const aspect = canvas.width / canvas.height;
      sprite.scale.set(size * aspect, size, 1);

      sprite.userData = {
      type,
      src,
      selected: false
    };


      

      resolve(sprite);
    };

    img.onerror = reject;
  });
}
