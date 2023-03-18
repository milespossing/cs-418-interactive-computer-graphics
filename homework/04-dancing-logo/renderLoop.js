// manages the collisions for multiple geometries.

// 1. initialize the models (scale, transform, velocity, hit_box)
// 2. every frame we move the models and check for collisions
// 2.1. If there is a collision, depending on the direction we flip the velocity for both logos

// A simple identity function
const identity = (a) => a;

// modes:
// static: the mv is an I4 matrix
// mv: (modelView) processes the model view as a function of time
const processFrame = (ms, delta) => (entity) => {
  const { mode } = entity;
  switch (mode) {
    case 'static':
      const modelView = entity.model ?? math.identity(4);
      return { ...entity, modelView };
    case 'mv':
      return { ...entity, modelView: entity.mv(ms) };
    default: throw Error('Unknown mode');
  }
};

// returns a callback function which can be used for drawing
const draw = (gl, program, setupGeometry) => {
  const mvBindPoint = gl.getUniformLocation(program, 'mv');
  const transformFirstBindPoint = gl.getUniformLocation(program, 'transform_first');
  // drawing callback function; set the uniforms for webGL, do any processing in the cpu
  return (ms) => (entity) => {
    const { modelView, geometry, preProcess } = entity;
    const processedGeometry = preProcess ? preProcess(geometry, ms) : geometry;
    const glslGeometry = setupGeometry(processedGeometry, 'dynamic');
    if (false && ms < 1000) {
      console.table(modelView);
      console.log(modelView.toArray().flat());
    }
    const arrTransform = new Float32Array(modelView.toArray().flat());
    gl.uniform1i(transformFirstBindPoint, !!entity.gpuTransform);
    gl.uniformMatrix4fv(mvBindPoint, true, arrTransform);
    gl.bindVertexArray(glslGeometry.vao);
    gl.drawElements(glslGeometry.mode, glslGeometry.count, glslGeometry.type, 0);
  };
};

// primary event loop, it prepares and executes a callback of type A -> A -> () where A: Number
export const eventLoop = (gl, program, setupGeometry, hooks = {}) => {
  gl.useProgram(program);
  const secondsBindPoint = gl.getUniformLocation(program, 'seconds');
  const psychedelicBindPoint = gl.getUniformLocation(program, 'psychedelic');
  const drawEntity = draw(gl, program, setupGeometry);
  // the main render loop for the program
  const loop = (last) => (ms) => {
    if (hooks.preRender) hooks.preRender();
    const delta = ms - last;
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.uniform1f(secondsBindPoint, ms / 1000);
    gl.uniform1i(psychedelicBindPoint, !!window.psychedelic);
    window.entities = window.systems.reduce((agg, s) => s(agg, delta, ms), window.entities);
    window.entities = window.entities.map(processFrame(ms, delta));
    window.entities.forEach(drawEntity(ms));
    return requestAnimationFrame(loop(ms));
  }
  return loop(0)(0);
}
