// manages the collisions for multiple geometries.

// 1. initialize the models (scale, transform, velocity, hit_box)
// 2. every frame we move the models and check for collisions
// 2.1. If there is a collision, depending on the direction we flip the velocity for both logos
const identity = (a) => a;

// modes:
// mv: (modelView) processes the model view as a function of time
const processFrame = (ms, delta) => (entity) => {
  const { mode, velocity, transform, mv } = entity;
  switch (mode) {
    case 'static':
      return { ...entity, modelView: math.identity(4) };
    case 'mv':
      return { ...entity, modelView: mv(ms) };
    default: throw Error('Unknown mode');
  }
};

const draw = (gl, program, setupGeometry) => {
  const mvBindPoint = gl.getUniformLocation(program, 'mv');
  const transformFirstBindPoint = gl.getUniformLocation(program, 'transform_first');
  return (ms) => (entity) => {
    const { modelView, geometry, preProcess } = entity;
    const processedGeometry = preProcess ? preProcess(geometry, ms) : geometry;
    const glslGeometry = setupGeometry(processedGeometry, 'dynamic');
    const arrTransform = new Float32Array(modelView.toArray().flat());
    gl.uniform1i(transformFirstBindPoint, !!entity.gpuTransform);
    gl.uniformMatrix4fv(mvBindPoint, false, arrTransform);
    gl.bindVertexArray(glslGeometry.vao);
    gl.drawElements(glslGeometry.mode, glslGeometry.count, glslGeometry.type, 0);
  };
};

export const eventLoop = (gl, program, setupGeometry) => {
  gl.useProgram(program);
  const secondsBindPoint = gl.getUniformLocation(program, 'seconds');
  const drawEntity = draw(gl, program, setupGeometry);
  const loop = (last) => (ms) => {
    const delta = ms - last;
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.uniform1f(secondsBindPoint, ms / 1000);
    window.entities = window.entities.map(processFrame(ms, delta));
    window.entities.forEach(drawEntity(ms));
    return requestAnimationFrame(loop(ms));
  }
  return loop()(0);
}
