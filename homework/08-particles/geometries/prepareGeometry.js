const computeNormals = (geometry) => {
  const normals = geometry.attributes.position.map(() => [0,0,0]);
  geometry.triangles.forEach(t => {
    const [p0, p1, p2] = t.map(p => geometry.attributes.position[p]);
    const e1 = sub(p1, p0);
    const e2 = sub(p2, p0);
    const n = cross(e1, e2);
    t.forEach(v => normals[v] = add(normals[v], n));
  });
  return normals.map(n => normalize(n));
}

export default (gl, program) => {
  const supplyData = (key, data, mode = gl.STATIC_DRAW) => {
    let buf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    let f32 = new Float32Array(data.flat());
    gl.bufferData(gl.ARRAY_BUFFER, f32, mode);

    let loc = gl.getAttribLocation(program, key);
    gl.vertexAttribPointer(loc, data[0].length, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(loc);

    return buf;
  };

  return (geometry) => {
    const normal = computeNormals(geometry);

    geometry.attributes = { ...geometry.attributes, normal };
    const triangleArray = gl.createVertexArray();
    gl.bindVertexArray(triangleArray);

    Object.keys(geometry.attributes).map((key) => {
      const value = geometry.attributes[key];
      return supplyData(key, value);
    });

    const indices = new Uint16Array(geometry.triangles.flat());
    const indexBuffer = gl.createBuffer();

    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW);

    return {
      mode: gl.TRIANGLES,
      count: indices.length,
      type: gl.UNSIGNED_SHORT,
      vao: triangleArray,
    };
  };
};
