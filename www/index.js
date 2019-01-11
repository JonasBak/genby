import * as wasm from "genby";
import {config, bindBrush} from "./simulationConfig";

const canvas = document.getElementById("canvas");

wasm.create();
const worldSize = wasm.size()

canvas.width = config.cellSize * worldSize[0];
canvas.height = config.cellSize * worldSize[1];

const gl = canvas.getContext("webgl");

const shaders = new Shader();
shaders.init(gl);
const scene = new Scene(canvas, gl, shaders);
const va = new VertexArray(gl, shaders);
scene.vas.push(va);

scene.camera.cameraTranslation = [-worldSize[0] / 2.0,-100,-worldSize[1]];
scene.camera.direction = [0, -2, -1];
scene.camera.zFar = 300;


bindBrush(canvas, wasm.alter_world);

const pushVertex = (pixels, heights, i, x, y, normal) => {
  const r = pixels[3*i]/255;
  const g = pixels[3*i+1]/255;
  const b = pixels[3*i+2]/255;
  const height = heights[i];
  va.makeVertex([x , height * 10,y], [r, g, b], normal);
}

const generateSquare = (pixels, heights, x, y, top) => {
  const x1 = x + 1;
  const y1 = top ? y+1 : y;
  const x2 = top ? x : x+1;
  const y2 = y + 1;
  
  const normal = vec3.normalize(vec3.create(), vec3.cross(
        vec3.create(),
        vec3.sub(vec3.create(), [x2, heights[y2 * worldSize[0] + x2] * 10, y2], [x, heights[y * worldSize[0] + x] * 10, y]),
        vec3.sub(vec3.create(), [x1, heights[y1 * worldSize[0] + x1] * 10, y1], [x, heights[y * worldSize[0] + x] * 10, y]),
      ));

  pushVertex(pixels, heights, y * worldSize[0] + x, x, y, normal);
  pushVertex(pixels, heights, y1 * worldSize[0] + x1, x1, y1, normal);
  pushVertex(pixels, heights, y2 * worldSize[0] + x2, x2, y2, normal);

}

const generateVa = (pixels, heights) => {
  va.vertexArray = [];
  for (let x = 0; x < worldSize[0] - 1; x++) {
    for (let y = 0; y < worldSize[1] - 1; y++) {
      generateSquare(pixels, heights, x, y, true);
      generateSquare(pixels, heights, x, y, false);
    }
  }
}

const drawWorld = pixels => {
  scene.update();

  scene.draw(gl, shaders);
};

const drawWind = winds => {
}

const loop = () => {
  wasm.tick(0.25);
  generateVa(wasm.get_pixels(config.drawHeight, config.drawWater, config.drawAirPressure, config.drawBiomes), wasm.get_heights(config.drawWater));
  drawWorld(wasm.get_pixels(config.drawHeight, config.drawWater, config.drawAirPressure, config.drawBiomes));
  if (config.drawWind)
    drawWind(wasm.get_wind_directions());
  requestAnimationFrame(loop);
};

loop();
