export const config = {
  drawWorld: true,
  drawWind: true,
  drawAirPressure: true,
  cellSize: 5,
  brush: {
    radius: 15,
    diffWater: 0.8,
    diffAirPressure: 0
  }
};

export const bindBrush = (canvas, func) => {
  canvas.addEventListener("click", e => {
    const x = e.layerX/config.cellSize;
    const y = e.layerY/config.cellSize;
    func(x, y, config.brush.radius, config.brush.diffWater, config.brush.diffAirPressure);
  });
};

export const bindCheckbox = (field) => {
  const elem = document.getElementById(field);
  elem.checked = config[field];

  elem.addEventListener("change", () => config[field] = elem.checked);
};

bindCheckbox("drawWorld");
bindCheckbox("drawWind");
bindCheckbox("drawAirPressure");
