// renders the game state
import { createView } from './math2.js';

const IlliniBlue     = new Float32Array([0.075, 0.16, 0.292, 1])
const IdentityMatrix = new Float32Array([1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1])
const Red            = new Float32Array([1,0,0,1]);
const Green          = new Float32Array([0,1,0,1]);
const Blue           = new Float32Array([0,0,1,1]);
const CliffBrown     = new Float32Array([0.12, 0.08, 0.05, 1]);

const loadImage = () => new Promise((res,rej) => {
  const img = new Image();
  img.src = 'texture.jpg';
  img.crossOrigin = 'anonymous';
  img.addEventListener('load', (event) => {
    res({ img, event });
  });
  setTimeout(() => {
    rej('Timeout');
  }, 5000);
});

const setupImage = async (gl) => {
  const { img } = await loadImage();
  const slot = 0;
  const texture = gl.createTexture();
  gl.activeTexture(gl.TEXTURE0 + slot);
  gl.bindTexture(gl.TEXTURE_2D, texture);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  gl.texImage2D(
    gl.TEXTURE_2D,
    0,
    gl.RGBA,
    gl.RGBA,
    gl.UNSIGNED_BYTE,
    img
  );
  gl.generateMipmap(gl.TEXTURE_2D);
  return slot;
}

// TODO: If terrain isn't changing between states we could probably
// save some cycles here by never changing the buffer?
export const renderState = async (gl, program, { geometry, ...initials }) => {
  gl.useProgram(program);
  const pLoc = gl.getUniformLocation(program, 'p');
  // TODO: Need to scale this somehow so when we move we're
  // in 'world' coords
  const vLoc = gl.getUniformLocation(program, 'v');
  const mLoc = gl.getUniformLocation(program, 'm');
  const mvLoc = gl.getUniformLocation(program, 'mv');
  const lightLoc = gl.getUniformLocation(program, 'lightdir');
  const color1Loc = gl.getUniformLocation(program, 'color1');
  const color2Loc = gl.getUniformLocation(program, 'color2');
  const color3Loc = gl.getUniformLocation(program, 'color3');
  const cliffColorLoc = gl.getUniformLocation(program, 'cliffColor');
  const lightColorLoc = gl.getUniformLocation(program, 'lightcolor1');
  const imgLoc = gl.getUniformLocation(program, 'texture1');
  const slot = await setupImage(gl);
  const { color1, color2, color3, cliffColor, lightDir, lightColor } = initials;
  const m = m4trans(-0.5, 0, -0.5);
  // set universal uniforms (for all frames)
  gl.uniform4fv(color1Loc, color1);
  gl.uniform4fv(color2Loc, color2);
  gl.uniform4fv(color3Loc, color3);
  gl.uniform3fv(lightColorLoc, lightColor);
  gl.uniform3fv(lightLoc, lightDir);
  gl.uniform4fv(cliffColorLoc, cliffColor);
  gl.uniform1i(imgLoc, slot);
  gl.bindVertexArray(geometry.vao);
  return (state, iteration) => {
    const log = (...inputs) => {
      if (iteration % 500 === 0) console.log(...inputs);
    };
    // log(state);
    gl.clearColor(...IlliniBlue);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    const { transform } = state;

    gl.uniformMatrix4fv(pLoc, false, state.perspective);
    gl.uniformMatrix4fv(vLoc, false, transform);
    gl.uniformMatrix4fv(mLoc, false, m);
    gl.uniformMatrix4fv(mvLoc, false, m4mul(transform,m));
    gl.drawElements(geometry.mode, geometry.count, geometry.type, 0);
  };
};

