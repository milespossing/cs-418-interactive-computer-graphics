// renders the game state
import { createView } from './math2.js'

const IlliniBlue     = new Float32Array([0.075, 0.16, 0.292, 1])
const IdentityMatrix = new Float32Array([1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1])
const Red            = new Float32Array([1,0,0,1]);
const Green          = new Float32Array([0,1,0,1]);
const Blue           = new Float32Array([0,0,1,1]);
const CliffBrown     = new Float32Array([0.12, 0.08, 0.05, 1]);

// TODO: get the position and rotation here
const viewFromTransform = (transform) => {
  return createView(transform.translation, transform.rotation);
}

// TODO: If terrain isn't changing between states we could probably
// save some cycles here by never changing the buffer?
export const renderState = (gl, program, { terrain, maxHeight, minHeight, ...initials }) => {
  gl.useProgram(program);
  const pLoc = gl.getUniformLocation(program, 'p');
  // TODO: Need to scale this somehow so when we move we're
  // in 'world' coords
  const vLoc = gl.getUniformLocation(program, 'v');
  const mLoc = gl.getUniformLocation(program, 'm');
  const lightLoc = gl.getUniformLocation(program, 'lightdir1');
  const color1Loc = gl.getUniformLocation(program, 'color1');
  const color2Loc = gl.getUniformLocation(program, 'color2');
  const color3Loc = gl.getUniformLocation(program, 'color3');
  const cliffColorLoc = gl.getUniformLocation(program, 'cliffColor');
  const maxHeightLoc = gl.getUniformLocation(program, 'maxHeight');
  const minHeightLoc = gl.getUniformLocation(program, 'minHeight');
  const lightColorLoc = gl.getUniformLocation(program, 'lightcolor1');
  console.log(initials);
  const { color1, color2, color3, cliffColor, lightDir, lightColor } = initials;
  const m = m4trans(-0.5, 0, -0.5);
  // set universal uniforms (for all frames)
  gl.uniform4fv(color1Loc, color1);
  gl.uniform4fv(color2Loc, color2);
  gl.uniform4fv(color3Loc, color3);
  gl.uniform3fv(lightColorLoc, lightColor);
  gl.uniform3fv(lightLoc, lightDir);
  gl.uniform1f(maxHeightLoc, maxHeight);
  gl.uniform1f(minHeightLoc, minHeight);
  gl.uniform4fv(cliffColorLoc, cliffColor);
  gl.bindVertexArray(terrain.vao);
  return (state, iteration) => {
    const log = (...inputs) => {
      if (iteration % 500 === 0) console.log(...inputs);
    };
    // log(state);
    gl.clearColor(...IlliniBlue);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    const { transform } = state;
    const view = transform;
    log(view);

    gl.uniformMatrix4fv(pLoc, false, state.perspective);
    gl.uniformMatrix4fv(vLoc, false, view);
    gl.uniformMatrix4fv(mLoc, false, m);
    gl.drawElements(terrain.mode, terrain.count, terrain.type, 0);
  };
};

