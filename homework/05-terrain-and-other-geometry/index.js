import renderWithGl from './rendering/index.js';

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
  const perspective = m4perspNegZ(0.1, 10, 1, canvas.width, canvas.height);
  // to do: update aspect ratio of projection matrix here
  if (window.gl) {
    gl.viewport(0,0, canvas.width, canvas.height)
  }
  if (window.render) await window.render(perspective);
}

/**
 * Compile, link, other option-independent setup
 */
async function setup(event) {
  const gl = document.querySelector('canvas').getContext('webgl2',
      // optional configuration object: see https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext
      {antialias: false, depth:true, preserveDrawingBuffer:true}
  );
  gl.enable(gl.DEPTH_TEST);
  gl.enable(gl.BLEND)
  gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
  window.renderScene = await renderWithGl(gl);
  window.gl = gl;
  window.render = window.renderScene();

  fillScreen();
}

/**
 * Generate geometry, render the scene
 */
async function setupScene(scene, options) {
  window.render = window.renderScene(scene, options);
  fillScreen();
}

window.setupScene = setupScene;
window.addEventListener('load', setup)
window.addEventListener('resize', fillScreen)
