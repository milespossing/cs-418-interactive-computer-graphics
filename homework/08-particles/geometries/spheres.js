import prepareGeometry from "./prepareGeometry.js";

export const createSphereGeometry = async (gl, program) => {
  const data = await fetch('geometries/sphere80.json').then(res => res.json());
  return prepareGeometry(gl, program)(data);
}