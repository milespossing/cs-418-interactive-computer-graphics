import { ident, scale } from './static.js';
import { x_axis as x_axis_rotation, y_axis as y_axis_rotation, z_axis as z_axis_rotation } from './rotation.js';
import growAndShrink from './growAndShrink.js';
import { compileAndLinkAnimation } from './compile.js';
import { compose } from './compose.js';


export const compileProgram = (gl, geometryData) => {
  return compileAndLinkAnimation(gl, geometryData);
};

export const transformers = {
  static: ident,
  smaller: scale(0.8),
  rotation: z_axis_rotation,
  growAndShrink: growAndShrink(0.05, 1, 1),
  simpleDance: compose(growAndShrink(0.05, 1, 3), z_axis_rotation),
}

