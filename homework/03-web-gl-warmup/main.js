const compileAndLinkGLSL = (gl) => (vs_source, fs_source) => {
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

const draw = (gl, program) => (ms) => {
  gl.clear(gl.COLOR_BUFFER_BIT);
  gl.useProgram(program);
  const secondsBindPoint = gl.getUniformLocation(program, 'seconds');
  gl.uniform1f(secondsBindPoint, ms/1000);
  const connection = gl.POINTS;
  const offset = 0;
  const count = 6 + (0|ms/100)%100;
  const countBindPoint = gl.getUniformLocation(program, 'count');
  gl.uniform1i(countBindPoint, count);
  gl.drawArrays(connection, offset, count);
  return requestAnimationFrame(draw(gl, program));
};

const setup = async () => {
  const gl = document.querySelector('canvas').getContext('webgl2');
  const compile = compileAndLinkGLSL(gl);
  const vs = await fetch('vertex.glsl').then(res => res.text());
  const fs = await fetch('fragment.glsl').then(res => res.text());
  const program = compile(vs, fs);

  requestAnimationFrame(draw(gl, program));
};

window.addEventListener('load', setup);

