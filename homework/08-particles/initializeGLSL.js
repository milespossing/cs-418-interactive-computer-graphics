
export default async (gl, vertexLoc, fragmentLoc) => {
  const vertex = await fetch(vertexLoc).then(r => r.text());
  const fragment = await fetch(fragmentLoc).then(r => r.text());
  const vs = gl.createShader(gl.VERTEX_SHADER);
  const fs = gl.createShader(gl.FRAGMENT_SHADER);
  gl.shaderSource(vs, vertex);
  gl.compileShader(vs);
  if (!gl.getShaderParameter(vs, gl.COMPILE_STATUS)) {
    console.error(gl.getShaderInfoLog(vs))
    throw Error("Vertex shader compilation failed")
  }
  gl.shaderSource(fs, fragment);
  gl.compileShader(fs);
  if (!gl.getShaderParameter(fs, gl.COMPILE_STATUS)) {
    console.error(gl.getShaderInfoLog(fs))
    throw Error("Fragment shader compilation failed")
  }
  const program = gl.createProgram();
  gl.attachShader(program, vs);
  gl.attachShader(program, fs);
  gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    console.error(gl.getProgramInfoLog(program))
    throw Error("Linking failed")
  }
  return program;
}
