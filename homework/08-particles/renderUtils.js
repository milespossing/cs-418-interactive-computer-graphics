/**
 * Resizes the canvas to completely fill the screen
 */
export function fillScreen() {
  let canvas = document.querySelector('canvas')
  document.body.style.margin = '0'
  canvas.style.width = '100%'
  canvas.width = canvas.clientWidth;
  canvas.style.height = '100%'
  canvas.height = canvas.clientHeight;
  canvas.style.width = ''
  canvas.style.height = ''
  const perspective = m4perspNegZ(0.1, 10, 1, canvas.width, canvas.height);
  // to do: update aspect ratio of projection matrix here
  if (window.gl) {
    gl.viewport(0, 0, canvas.width, canvas.height);
  }
  if (window.createNewLoop)
    window.createNewLoop(perspective, window.config);
}
