export function exportScene({ sceneNodes }) {
  const nodes = sceneNodes.map(obj => ({
    type: obj.userData.type,
    position: {
      x: obj.position.x,
      y: obj.position.y,
      z: obj.position.z
    }
  }));

  const payload = {
    version: 1,
    nodes
  };

  // Download locally only
  const blob = new Blob(
    [JSON.stringify(payload, null, 2)],
    { type: "application/json" }
  );

  const a = document.createElement("a");
  a.href = URL.createObjectURL(blob);
  a.download = "hypergraph.json";
  a.click();
  URL.revokeObjectURL(a.href);
}


export function importScene({ file, addNode }) {
  const reader = new FileReader();

  reader.onload = async () => {
    const data = JSON.parse(reader.result);
    if (!data.nodes) {
      alert("Invalid scene file");
      return;
    }

    for (const n of data.nodes) {
      await addNode(n);
    }
  };

  reader.readAsText(file);
}
