
export function showEdgeInfo(edgeId, edgeInfo ) {
  const box = document.getElementById("info-box");
  box.innerHTML = `
    <div style="display:flex;justify-content:space-between;align-items:center;">
      <strong>Edge: ${edgeId}</strong>
      <button onclick="document.getElementById('info-box').style.display='none'">X</button>
    </div>
    <div><strong>Details:</strong> ${JSON.stringify(edgeInfo)}</div>
  `;
  box.style.display = "block";
}

export function showNodeInfo(nodeId, nodeInfo) {
  const box = document.getElementById("info-box");
  box.innerHTML = `
    <div style="display:flex; justify-content:space-between; align-items:center;">
      <strong>Node: ${nodeId}</strong>
      <button onclick="document.getElementById('info-box').style.display='none'" 
              style="cursor:pointer;">X</button>
    </div>
    <div><strong>Details:</strong> ${JSON.stringify(nodeInfo)}</div>
  `;
  box.style.display = "block";
}
