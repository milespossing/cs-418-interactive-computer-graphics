export const IdentityMatrix = new Float32Array([1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1])

export const m4mulC = R.curryN(2, m4mul);
export const m4mulR = R.flip(m4mulC);

export const buildRotation = (x,y,z,v) => R.compose(m4mulC(m4rotZ(z)), m4mulC(m4rotX(x)), m4mulC(m4rotY(y)))(v);

export const createView = (position, focus) => {
  return m4view(position.slice(0,3), focus.slice(0,3), [0,1,0]);
}

export const focusOnPoint = (position, focus) => {
  const direction = math.subtract(focus, position);
  console.log(direction);
  return m4fixAxes(direction, [0,1,0]);
};

export const rotationFromHomo = (matrix) => new Float32Array([
  matrix[0], matrix[1], matrix[2],
  matrix[4], matrix[5], matrix[6],
  matrix[8], matrix[9], matrix[10],
]);


export const getTrans = (matrix) => new Float32Array([
  matrix[12], matrix[13], matrix[14],
]);

export const setTrans = (trans, matrix) => new Float32Array([
  matrix[0], matrix[1],  matrix[2],  matrix[3],
  matrix[4], matrix[5],  matrix[6],  matrix[7],
  matrix[8], matrix[9], matrix[10], matrix[11],
   trans[0],  trans[1],   trans[2], matrix[15],
]);

export const distToPlane = (p, n, point) => {
  const v = math.subtract(point, p);
  return math.abs(math.dot(v, n));
}

export const rightSideOfPlane = (p, n, point) => {
  const v = math.subtract(point, p);
  return math.dot(v, n) > 0;
}

export const homogenize = (v, a=1) => [...v, a];
export const homogenizeRot3 = (m) => new Float32Array([
  ...m[0], 0,
  ...m[1], 0,
  ...m[2], 0,
  0, 0, 0, 1,
])
