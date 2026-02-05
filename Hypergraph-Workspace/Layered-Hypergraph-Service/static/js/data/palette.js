import { paletteCategories, umlCategories } from "../../palette-data/index.js";

const bioPalette = document.getElementById("palette");
const umlPalette = document.getElementById("uml-palette");

// Render Bio categories
paletteCategories.forEach(category => {
  const wrapper = createCategoryWrapper(category);
  bioPalette.appendChild(wrapper);
});

// Render UML categories
umlCategories.forEach(category => {
  const wrapper = createCategoryWrapper(category);
  umlPalette.appendChild(wrapper);
});

// Utility to create a palette category DOM element
function createCategoryWrapper(category) {
  const wrapper = document.createElement("div");
  wrapper.className = "palette-category";

  const header = document.createElement("button");
  header.className = "palette-header";
  header.textContent = category.label;

  const content = document.createElement("div");
  content.className = "palette-content";

  category.icons.forEach(icon => {
  const item = document.createElement("div");
  item.className = "palette-item";

  let iconEl;

  if (icon.type === "plane") {
    iconEl = document.createElement("div");
    iconEl.className = "palette-icon";
    iconEl.draggable = true;
    iconEl.dataset.type = icon.type;
    iconEl.dataset.src = "__plane__";

    const canvas = document.createElement("canvas");
    canvas.width = 40;
    canvas.height = 40;
    iconEl.appendChild(canvas);
    drawPlaneIcon(canvas);
  } else {
    iconEl = document.createElement("img");
    iconEl.className = "palette-icon";
    iconEl.draggable = true;
    iconEl.src = icon.src;
    iconEl.dataset.src = icon.src;
    iconEl.dataset.type = icon.type;
  }

  const label = document.createElement("div");
  label.className = "palette-label";
  label.textContent = icon.type;

  item.appendChild(iconEl);
  item.appendChild(label);
  content.appendChild(item);
});

  wrapper.appendChild(header);
  wrapper.appendChild(content);
  return wrapper;
}

// Draw plane icon on canvas
function drawPlaneIcon(canvas) {
  const ctx = canvas.getContext("2d");

  ctx.clearRect(0, 0, canvas.width, canvas.height);

  ctx.fillStyle = "rgba(255,202,254,0.7)";
  ctx.strokeStyle = "#0000ff";
  ctx.lineWidth = 2;

  ctx.save();
  ctx.translate(20, 22);
  ctx.transform(1, 0, 0.4, 0.7, 0, 0);

  ctx.fillRect(-14, -6, 28, 12);
  ctx.strokeRect(-14, -6, 28, 12);

  ctx.restore();
}
