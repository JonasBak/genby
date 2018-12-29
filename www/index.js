import * as wasm from "genby";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const cellSize = 5;
wasm.create();
const worldSize = wasm.size()

canvas.width = cellSize * worldSize[0];
canvas.height = cellSize * worldSize[1];

const drawWorld = cells => {
  for (let x = 0; x < worldSize[0]; x++) {
    for (let y = 0; y < worldSize[1]; y++) {
      let i = y * worldSize[0] + x;
      ctx.fillStyle = "rgb(" + cells[3 * i] + "," + cells[3 * i + 1] + "," + cells[3 * i + 2] + ")";
      ctx.fillRect(x*cellSize, y * cellSize, cellSize, cellSize);
    }
  }
};

const drawWind = winds => {
  ctx.beginPath();
  for (let x = 1; x < 20; x++) {
    for (let y = 1; y < 20; y++) {
      const x0 = x * (worldSize[0]/ 20);
      const y0 = y * (worldSize[1]/ 20);
      let i = y0  * worldSize[0] + x0;

      const windFactor = 500;

      ctx.moveTo(x0 * cellSize, y0 * cellSize);
      ctx.lineTo(x0 * cellSize + winds[2*i ] * windFactor, y0 * cellSize + winds[2*i + 1] * windFactor);
      
    }
  }
  ctx.stroke();
  ctx.closePath();

}

const loop = () => {
  wasm.tick(0.1);
  drawWorld(wasm.get_pixels());
  drawWind(wasm.get_wind_directions());
  requestAnimationFrame(loop);
};

canvas.addEventListener("click", e => {
  const x = e.layerX/cellSize;
  const y = e.layerY/cellSize;
  wasm.alter_world(x, y, 10, 0.5, 0);
});

loop();
