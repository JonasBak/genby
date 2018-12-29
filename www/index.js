import * as wasm from "genby";
import {config, bindBrush} from "./simulationConfig";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

//const cellSize = 5;
wasm.create();
const worldSize = wasm.size()

canvas.width = config.cellSize * worldSize[0];
canvas.height = config.cellSize * worldSize[1];

bindBrush(canvas, wasm.alter_world);

const drawWorld = cells => {
  for (let x = 0; x < worldSize[0]; x++) {
    for (let y = 0; y < worldSize[1]; y++) {
      let i = y * worldSize[0] + x;
      ctx.fillStyle = "rgb(" + cells[3 * i] + "," + cells[3 * i + 1] + "," + cells[3 * i + 2] + ")";
      ctx.fillRect(x*config.cellSize, y * config.cellSize, config.cellSize, config.cellSize);
    }
  }
};

const drawAirPressure = pressure => {
  for (let x = 0; x < worldSize[0]; x++) {
    for (let y = 0; y < worldSize[1]; y++) {
      let i = y * worldSize[0] + x;
      ctx.fillStyle = "rgba(0,255,0," + pressure[i] **2 / 10 + ")";
      ctx.fillRect(x*config.cellSize, y * config.cellSize, config.cellSize, config.cellSize);
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

      ctx.moveTo(x0 * config.cellSize, y0 * config.cellSize);
      ctx.lineTo(x0 * config.cellSize + winds[2*i ] * windFactor, y0 * config.cellSize + winds[2*i + 1] * windFactor);
      
    }
  }
  ctx.stroke();
  ctx.closePath();

}

const loop = () => {
  ctx.clearRect(0,0,config.cellSize*worldSize[0], config.cellSize*worldSize[1]);
  wasm.tick(0.1);
  wasm.tick(0.1);
  wasm.tick(0.1);
  wasm.tick(0.1);
  wasm.tick(0.1);
  if (config.drawWorld)
    drawWorld(wasm.get_pixels());
  if (config.drawAirPressure)
    drawAirPressure(wasm.get_air_pressure());
  if (config.drawWind)
    drawWind(wasm.get_wind_directions());
  requestAnimationFrame(loop);
};

loop();
