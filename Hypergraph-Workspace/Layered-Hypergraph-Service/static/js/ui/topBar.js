// ui/topBar.js
export function createTopBar({ onImport, onExport }) {
  const bar = document.createElement("div");

  bar.style.position = "fixed";
  bar.style.top = "0";
  bar.style.left = "0";
  bar.style.width = "100%";
  bar.style.height = "42px";
  bar.style.display = "flex";
  bar.style.alignItems = "center";
  bar.style.gap = "12px";
  bar.style.padding = "0 12px";
  bar.style.background = "#0f172a"; // slate-900
  bar.style.color = "#e5e7eb";
  bar.style.zIndex = "9999";
  bar.style.fontFamily = "system-ui, sans-serif";

  const importBtn = document.createElement("button");
  importBtn.textContent = "⬆ Import";
  styleBtn(importBtn);
  importBtn.onclick = onImport;

  const exportBtn = document.createElement("button");
  exportBtn.textContent = "⬇ Export";
  styleBtn(exportBtn);
  exportBtn.onclick = onExport;

  bar.appendChild(importBtn);
  bar.appendChild(exportBtn);

  document.body.appendChild(bar);

  // push canvas down so it’s not covered
  document.body.style.paddingTop = "42px";
}

function styleBtn(btn) {
  btn.style.background = "#1e293b";
  btn.style.border = "1px solid #334155";
  btn.style.color = "#e5e7eb";
  btn.style.padding = "6px 12px";
  btn.style.cursor = "pointer";
  btn.style.borderRadius = "6px";
}
