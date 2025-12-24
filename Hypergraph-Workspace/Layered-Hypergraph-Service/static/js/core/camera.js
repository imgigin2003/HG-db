// core/camera.js
export function animateCameraTo(camera, controls, targetPos, targetLookAt, durationMs = 900) {
    const startPos = camera.position.clone();
    const startTarget = controls.target.clone();
    let startTime = null;

    function animate(timestamp) {
        if (!startTime) startTime = timestamp;
        const elapsed = timestamp - startTime;
        const t = Math.min(elapsed / durationMs, 1);
        const ease = t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;

        camera.position.lerpVectors(startPos, targetPos, ease);
        controls.target.lerpVectors(startTarget, targetLookAt, ease);
        controls.update();

        if (t < 1) requestAnimationFrame(animate);
    }

    requestAnimationFrame(animate);
}

export function zoomToPoint(camera, controls, pointVec3, offsetUp = 0.25, backDist = 0.6, scaleBack = 1.0, durationMs = 700) {
    const center = pointVec3.clone();
    const tp = new THREE.Vector3(center.x, center.y + offsetUp, center.z + backDist * scaleBack);
    const startPos = camera.position.clone();
    const startTarget = controls.target.clone();
    let startTime = null;

    function anim(timestamp) {
        if (!startTime) startTime = timestamp;
        const t = Math.min((timestamp - startTime) / durationMs, 1);
        const eased = t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t;
        camera.position.lerpVectors(startPos, tp, eased);
        controls.target.lerpVectors(startTarget, center, eased);
        controls.update();
        if (t < 1) requestAnimationFrame(anim);
    }

    requestAnimationFrame(anim);
}
