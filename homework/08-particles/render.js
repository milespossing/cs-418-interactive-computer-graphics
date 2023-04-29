export const renderState = async (gl, program, geometries) => {
  gl.useProgram(program);
  const pLoc = gl.getUniformLocation(program, 'p');
  const vLoc = gl.getUniformLocation(program, 'v');
  const mLoc = gl.getUniformLocation(program,'m');
  const mvLoc = gl.getUniformLocation(program,'mv');
  const colorLoc = gl.getUniformLocation(program, 'color');
  const lightColorLoc = gl.getUniformLocation(program, 'lightcolor1');
  const lightDirLoc = gl.getUniformLocation(program, 'lightdir1');
  return function createRenderer(perspective, { backgroundColor, lightColor1, lightDir1 }) {
    gl.uniformMatrix4fv(pLoc, false, perspective);
    gl.uniform3fv(lightColorLoc, lightColor1);
    gl.uniform3fv(lightDirLoc, lightDir1);
    return function iterate(state)  {
      gl.clearColor(...backgroundColor);
      gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
      const groupByGeometry = R.groupBy(R.prop('geometry'));
      const groups = groupByGeometry(state.entities);
      gl.uniformMatrix4fv(vLoc, false, state.view);
      const drawEntity = (geometry, modelGenerator) => (entity) => {
        // individual drawing logic here
        const { color } = entity;
        const model = modelGenerator(entity, state.view);
        gl.uniform4fv(colorLoc, color);
        gl.uniformMatrix4fv(mLoc, false, model);
        gl.uniformMatrix4fv(mvLoc, false, m4mul(state.view, model));

        // draw the element
        gl.drawElements(geometry.mode, geometry.count, geometry.type, 0);
      }
      const drawEntityTypeByGeometry = (entities, geomKey) => {
        // geometry drawing logic here
        const [geometry, generator] = geometries[geomKey];
        gl.bindVertexArray(geometry.vao);
        entities.forEach(drawEntity(geometry, generator));
      };
      R.forEachObjIndexed(drawEntityTypeByGeometry, groups);
    }
  }
}