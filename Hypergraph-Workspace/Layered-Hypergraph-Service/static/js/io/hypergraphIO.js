

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

  // --- 1. Download locally ---
  const blob = new Blob([JSON.stringify(payload, null, 2)], {
    type: "application/json"
  });

  const a = document.createElement("a");
  a.href = URL.createObjectURL(blob);
  a.download = "hypergraph.json";
  a.click();
  URL.revokeObjectURL(a.href);

  // --- 2. Send JSON to backend (Flask) ---
  fetch("http://localhost:5000/scene/load", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload)
  })
  .then(res => res.json())
  .then(data => console.log("Scene pushed to backend:", data))
  .catch(err => console.error("Failed to push scene:", err));
}


export function importScene({ file, addNode }) {
  const reader = new FileReader();

  reader.onload = e => {
    try {
      const data = JSON.parse(e.target.result);
      if (!data.nodes) throw new Error("Invalid file");

      data.nodes.forEach(n => {
        addNode(n);
      });
    } catch (err) {
      alert("Invalid JSON file");
      console.error(err);
    }
  };

  reader.readAsText(file);
}

