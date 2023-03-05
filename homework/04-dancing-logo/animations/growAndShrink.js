// range = max - min
// midPoint = (max + min) / 2

export default (minSize, maxSize, rate) => (ms) => {
  const scale = ((minSize + maxSize) / 2) + (maxSize - minSize) * Math.sin(rate * ms / 1000 / (2 * Math.PI)) / 2;
  return math.matrix([
    [scale, 0, 0, 0],
    [0, scale, 0, 0],
    [0, 0, scale, 0],
    [0, 0, 0, 1]   ]);
};
