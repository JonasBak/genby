import * as wasm from "genby";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const cellSize = 5;
wasm.create();
const worldSize = wasm.size()

canvas.width = cellSize * worldSize[0];
canvas.height = cellSize * worldSize[1];

const drawWorld = (heights) => {
  console.log(worldSize);
  for (let x = 0; x < worldSize[0]; x++) {
    for (let y = 0; y < worldSize[1]; y++) {
      const h = heights[y * worldSize[0] + x];
      const color = h * 255/2 + 255/2;
      ctx.fillStyle = "rgb(" + color + "," + color + "," + color + ")";
      ctx.fillRect(x*cellSize, y * cellSize, cellSize, cellSize);
    }
  }
}

drawWorld(wasm.export_height());
