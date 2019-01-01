export const config = {
  drawHeight: true,
  drawWater: true,
  drawWind: true,
  drawAirPressure: false,
  cellSize: 5,
  brush: {
    radius: 15,
    diffWater: 0.8,
    diffAirPressure: 0,
    diffHeight: 0
  }
};

export const bindBrush = (canvas, func) => {
  canvas.addEventListener("click", e => {
    const x = e.layerX/config.cellSize;
    const y = e.layerY/config.cellSize;
    func(x, y, config.brush.radius, config.brush.diffWater, config.brush.diffAirPressure, config.brush.diffHeight);
  });
};

export const bindCheckbox = (id, setValue, initialValue) => {
  const elem = document.getElementById(id);
  elem.checked = initialValue;
  elem.addEventListener("change", () => setValue(elem.checked));
};

export const bindInput = (id, setValue, initialValue) => {
  const elem = document.getElementById(id);
  elem.value = initialValue;
  elem.addEventListener("change", () => setValue(elem.value));
};

bindCheckbox("drawHeight", value => config.drawHeight = value, config.drawHeight);
bindCheckbox("drawWater", value => config.drawWater = value, config.drawWater);
bindCheckbox("drawWind", value => config.drawWind = value, config.drawWind);
bindCheckbox("drawAirPressure", value => config.drawAirPressure = value, config.drawAirPressure);

bindInput("diffWater", value => config.brush.diffWater = value, config.brush.diffWater);
bindInput("diffAirPressure", value => config.brush.diffAirPressure = value, config.brush.diffAirPressure);
bindInput("diffHeight", value => config.brush.diffHeight = value, config.brush.diffHeight);
bindInput("radius", value => config.brush.radius = value, config.brush.radius);
