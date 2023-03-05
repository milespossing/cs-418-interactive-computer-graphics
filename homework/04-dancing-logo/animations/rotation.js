export const x_axis = (ms) => {
  const degs = ms / 1000;
  return math.matrix([
    [1, 0, 0, 0,                                 ],
    [0, Math.cos(degs), -1.0 * Math.sin(degs), 0,],
    [0, Math.sin(degs),        Math.cos(degs), 0,],
    [0,              0,                     0, 1,],
  ]);
}

export const y_axis = (ms) => {
  const degs = ms / 1000;
  return math.matrix([
    [Math.cos(degs), 0, -1.0 * Math.sin(degs), 0,],
    [             0, 1, 0, 0,]                    ,
    [Math.sin(degs), 0, Math.cos(degs), 0,]       ,
    [             0,                     0, 0, 1,],
  ]);
}
// A simple rotation to make sure that everything works
// rotates clockwise very slowly
export const z_axis = (ms) => {
  const degs = ms / 1000;
  return math.matrix([
    [Math.cos(degs), -1.0 * Math.sin(degs), 0, 0],
    [Math.sin(degs),        Math.cos(degs), 0, 0],
    [             0,                     0, 1, 0],
    [             0,                     0, 0, 1],
  ]);
}
