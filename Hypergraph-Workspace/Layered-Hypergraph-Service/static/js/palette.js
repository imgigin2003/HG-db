import { paletteCategories  } from "../palette-data/index.js";

const palette = document.getElementById("palette");

paletteCategories.forEach(category => {
  const wrapper = document.createElement("div");
  wrapper.className = "palette-category";

  const header = document.createElement("button");
  header.className = "palette-header";
  header.textContent = category.label;

  const content = document.createElement("div");
  content.className = "palette-content";

category.icons.forEach(icon => {
  let el;

  if (icon.type === "plane") {
    el = document.createElement("div");
    el.className = "palette-icon";
    el.dataset.type = "plane";
    el.draggable = true;

    const canvas = document.createElement("canvas");
    canvas.width = 40;
    canvas.height = 40;
    el.appendChild(canvas);

    drawPlaneIcon(canvas);
  } else {
    el = document.createElement("img");
    el.src = icon.src;
    el.dataset.src = icon.src;
    el.className = "palette-icon";
    el.draggable = true;
    el.dataset.type = icon.type;
  }

  content.appendChild(el);
});


  wrapper.appendChild(header);
  wrapper.appendChild(content);
  palette.appendChild(wrapper);
});

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
