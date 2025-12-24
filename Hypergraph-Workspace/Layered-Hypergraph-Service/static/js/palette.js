import { paletteCategories } from "../palette-data/index.js";

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
    const img = document.createElement("img");
    img.src = icon.src;
    img.draggable = true;
    img.className = "palette-icon";
    img.dataset.type = icon.type;
    img.dataset.src = icon.src;

    content.appendChild(img);
  });

  wrapper.appendChild(header);
  wrapper.appendChild(content);
  palette.appendChild(wrapper);
});
