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

const buildTerrain = (resolution, slices) => {
  const grid = sliceN(slices, resolution, math.zeros(resolution, resolution), 0);
  return tesilate(postProcess(0.3)(grid), resolution);
};

const computeNormals = (geometry) => {
  const normals = geometry.attributes.position.map(() => [0,0,0]);
  geometry.triangles.forEach(t => {
    const [p0, p1, p2] = t.map(p => geometry.attributes.position[p]);
    const e1 = sub(p1, p0);
    const e2 = sub(p2, p0);
    const n = cross(e1, e2);
    t.forEach(v => normals[v] = add(normals[v], n));
  });
  return normals.map(n => normalize(n));
}

const computeTexCoords = (geometry) => {
  return geometry.attributes.position.map((p) => [p[0],p[2]]);
}

const prepareGeometry = (gl, program) => {
  const supplyData = (key, data, mode = gl.STATIC_DRAW) => {
    let buf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    let f32 = new Float32Array(data.flat());
    gl.bufferData(gl.ARRAY_BUFFER, f32, mode);

    let loc = gl.getAttribLocation(program, key);
    gl.vertexAttribPointer(loc, data[0].length, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(loc);

    return buf;
  };

  return (geometry) => {
    const normal = computeNormals(geometry);
    const aTexCoord = computeTexCoords(geometry);

    geometry.attributes = { ...geometry.attributes, normal, aTexCoord };
    const triangleArray = gl.createVertexArray();
    gl.bindVertexArray(triangleArray);

    Object.keys(geometry.attributes).map((key) => {
      const value = geometry.attributes[key];
      return supplyData(key, value);
    });

    const indices = new Uint16Array(geometry.triangles.flat());
    const indexBuffer = gl.createBuffer();

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW);

    return {
      mode: gl.TRIANGLES,
      count: indices.length,
      type: gl.UNSIGNED_SHORT,
      vao: triangleArray,
    };
  };
};

const getMaxMin = (positions) => positions.reduce((acc, p) => {
  if (p[1] < acc.min) return { ...acc, min: p[1] };
  if (p[1] > acc.max) return { ...acc, max: p[1] };
  return acc;
}, { min: 0, max: 0 });

const getScene = (gl, program) => {
  const generated = buildTerrain(100, 50, false);
  const terrain = prepareGeometry(gl, program)(generated);
  const { max, min } = getMaxMin(generated.attributes.position);
  return {
    terrain,
    maxHeight: max,
    minHeight: min,
  };
}

export default getScene;
