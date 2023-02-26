import staticAnimation from './static.js';
import rotation from './rotation.js';
import { compileAndLinkAnimation } from './compile.js';


export const compileProgram = (gl, geometryData) => {
  return compileAndLinkAnimation(gl, geometryData);
};

export const getTransformers = {
  static: staticAnimation,
  rotation,
}

