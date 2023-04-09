import compile from './compile.js';
import incrementState, { initializeState } from './state.js';
import getScene from './terrain.js';
import { renderState } from './render.js';
import { executeLoop } from './loop.js';

/**
 * Resizes the canvas to completely fill the screen
 */
async function fillScreen() {
  let canvas = document.querySelector('canvas')
  document.body.style.margin = '0'
  canvas.style.width = '100%'
  canvas.style.height = '100%'
  canvas.width = canvas.clientWidth
  canvas.height = canvas.clientHeight
  canvas.style.width = ''
  canvas.style.height = ''
  const nextState = { 
    ...window.state,
    perspective: m4perspNegZ(0.1, 10, 1, canvas.width, canvas.height),
  };
  // to do: update aspect ratio of projection matrix here
  if (window.gl) {
    gl.viewport(0,0, canvas.width, canvas.height);
  }
  if (window.createNewLoop)
    window.createNewLoop(nextState);
}

const setup = async () => {
  const gl = document.querySelector('canvas').getContext('webgl2',
      // optional configuration object: see https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext
      {antialias: false, depth:true, preserveDrawingBuffer:true}
  );
  gl.enable(gl.DEPTH_TEST);
  gl.enable(gl.BLEND)
  gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
  const program = await compile(gl);
  const { terrain, maxHeight, minHeight } = getScene(gl, program);
  const initialState = initializeState(gl, program);
  const config = {
    terrain,
    maxHeight,
    minHeight,
    color1: new Float32Array([0,0,1,1]),
    color2: new Float32Array([1,0,0,1]),
    color3: new Float32Array([0,1,0,1]),
    cliffColor: new Float32Array([0.2, 0.18, 0.1, 1]),
    lightDir: new Float32Array([0, 1, 0]),
    lightColor: new Float32Array([1,1,1]),
  };
  window.state = initialState;
  window.gl = gl;
  window.program = program;
  window.createNewLoop = executeLoop(gl, program, incrementState, await renderState(gl, program, config));

  fillScreen();
};

window.keysBeingPressed = {};
window.addEventListener('keydown', event => window.keysBeingPressed[event.key] = true);
window.addEventListener('keyup', event => window.keysBeingPressed[event.key] = false);
window.addEventListener('load', setup);
window.addEventListener('resize', () => requestAnimationFrame(fillScreen));
