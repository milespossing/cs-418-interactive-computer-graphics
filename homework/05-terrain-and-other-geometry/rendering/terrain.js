const IlliniOrange = new Float32Array([1, 0.373, 0.02, 1]);
const Red =   new Float32Array([1,0,0,1]);
const Green = new Float32Array([0,1,0,1]);
const Blue =  new Float32Array([0,0,1,1]);

const getRandoms = (resolution) => [
  Math.floor(Math.random() * resolution), // i
  Math.floor(Math.random() * resolution), // j
  Math.random() * Math.PI * 2, // theta
];

const performSlice = (resolution) => (size) => (grid) => {
  const [i,j,theta] = getRandoms(resolution);
  const p = [i,j];
  const n = [math.cos(theta), math.sin(theta)];
  return grid.map((value, index, matrix) => {
    const result = math.dot(math.subtract(index, p), n);
    return result >= 0 ? value + size : value - size;
  });
};

const tesilate = (grid, resolution) => {
  const position = grid.toArray().map((row, i) =>
    row.map((value, j) => [i/resolution, value/resolution, j/resolution])
  ).flat();
  const getIndex = (i,j) => (i * resolution) + j;
  const triangles = grid.toArray().map((row, i) => row.map((value, j) => {
    return (i < resolution - 1 && j < resolution - 1)
      ? [
          [getIndex(i, j), getIndex(i,j+1), getIndex(i+1,j)],
          [getIndex(i+1,j), getIndex(i, j+1), getIndex(i+1,j+1)],
        ]
      : [];
  })).flat().flat();
  const attributes = { position };
  return { attributes, triangles };
};

const sliceN = (n, resolution, grid, i) => {
  if (i >= n) return grid;
  const size = Math.random() * 10;
  return sliceN(n, resolution, performSlice(resolution)(size)(grid), i + 1);
}

const postProcess = c => grid => {
  const max = math.max(grid);
  const min = math.min(grid);
  const h = (max - min) * c;
  return max - min > 0
    ? grid.map((value) => (value - min) * h / (max - min) - (h / 2))
    : grid;
}

const terrain = ({ resolution, slices, shiny }) => {
  const grid = sliceN(slices, resolution, math.zeros(resolution, resolution), 0);
  const geometry = tesilate(postProcess(0.3)(grid), resolution);
  const colors = [
    Blue,
    Red,
    Green,
  ];
  const cliffColor = new Float32Array([0.12, 0.08, 0.05, 1]);
  const lightDir = normalize([1, 1, 1]);
  return Promise.resolve([geometry, { shiny, colors, cliffColor, model: m4trans(-0.5, 0, -0.5), color: IlliniOrange }]);
};

export default terrain;
