
const entity = {
  geometry: 'sphere', // should be the key for a particular model in the sim
  model: new math.Matrix, // should be a 4x4 matrix
  color: [], // V4
};

const state = {
  view: [], // should be a 4x4 matrix
  entities: [], // should be an array of entities
}