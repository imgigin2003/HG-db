// scene/grouping.js
import * as THREE from "three";

export function createGroupFromObjects(objects, scene) {
  const group = new THREE.Group();
  group.userData.isGroup = true;
  group.userData.members = [];

  // compute center
  const center = new THREE.Vector3();
  objects.forEach(o => {
    const v = new THREE.Vector3();
    o.getWorldPosition(v);
    center.add(v);
  });
  center.divideScalar(objects.length);

  group.position.copy(center);

  objects.forEach(obj => {
    const wp = new THREE.Vector3();
    obj.getWorldPosition(wp);
    obj.userData.groupRoot = group;
    obj.userData.isInGroup = true;
    scene.remove(obj);
    group.add(obj);

    obj.position.copy(group.worldToLocal(wp));
    obj.userData.isGrouped = true;
    obj.userData.parentGroup = group;
    group.userData.isGroupRoot = true;


    group.userData.members.push(obj);
  });

  scene.add(group);
  return group;
}

export function separateGroup(group, scene) {
  if (!group?.userData?.isGroup) return;

  const members = [...group.userData.members];

  members.forEach(obj => {
    const wp = new THREE.Vector3();
    obj.getWorldPosition(wp);

    group.remove(obj);
    scene.add(obj);

    obj.position.copy(wp);
    obj.userData.isGrouped = false;
    obj.userData.parentGroup = null;
  });

  scene.remove(group);
}
