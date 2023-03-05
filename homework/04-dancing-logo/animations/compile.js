export const compileAndLinkGLSL = (gl) => (vs_source, fs_source) => {
  const vs = gl.createShader(gl.VERTEX_SHADER);
  gl.shaderSource(vs, vs_source);
  gl.compileShader(vs);
  if (!gl.getShaderParameter(vs, gl.COMPILE_STATUS)) {
    // error condition
    console.error(gl.getShaderInfoLog(vs));
    throw Error('Vertex shader compilation failed');
  }

  const fs = gl.createShader(gl.FRAGMENT_SHADER);
  gl.shaderSource(fs, fs_source);
  gl.compileShader(fs);
  if (!gl.getShaderParameter(fs, gl.COMPILE_STATUS)) {
    // error condition
    console.error(gl.getShaderInfoLog(fs));
    throw Error('Fragment shader compilation failed');
  }

  const program = gl.createProgram();
  gl.attachShader(program, vs);
  gl.attachShader(program, fs);
  gl.linkProgram(program);

  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    console.error(gl.getProgramInfoLog(program));
    throw Error("Linking failed");
  }
  return program;
};

const setupGeometry = (gl, program) => (data, type = 'static') => {
  const triangleArray = gl.createVertexArray();
  gl.bindVertexArray(triangleArray);

  Object.entries(data.attributes).forEach(([name, entries]) => {
    const buf = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    const f32 = new Float32Array(entries.flat());
    gl.bufferData(gl.ARRAY_BUFFER, f32, type === 'static' ? gl.STATIC_DRAW : gl.DYNAMIC_DRAW);

    const loc = gl.getAttribLocation(program, name);
    gl.vertexAttribPointer(loc, entries[0].length, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(loc);
  });

  const indices = new Uint16Array(data.triangles);
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

// Returns a tuple of type [program, geometry]
export const compileAndLinkAnimation = async (gl) => {
  const compile = compileAndLinkGLSL(gl);
  const root = 'animations/';
  const vs = await fetch(root + 'vertex.glsl').then(r => r.text());
  const fr = await fetch(root + 'fragment.glsl').then(r => r.text());
  const program = compile(vs, fr);
  const setupGeom = setupGeometry(gl, program);
  return [program, setupGeom];
};
