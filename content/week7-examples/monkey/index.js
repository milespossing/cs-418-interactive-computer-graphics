
const IlliniBlue = new Float32Array([0.075, 0.16, 0.292, 1]);
const IlliniOrange = new Float32Array([1, 0.373, 0.02, 1]);
const IdentityMatrix = new Float32Array([1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1])

const compileProgram = async (gl) => {
  const vertex = await fetch('vertex.glsl').then(r => r.text());
  const fragment = await fetch('fragment.glsl').then(r => r.text());
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

const fillScreen = (canvas, gl) => () => {
  canvas.style.width = '100vw';
  canvas.style.height = '100vh';
  canvas.width = canvas.clientWidth;
  canvas.height = canvas.clientHeight;
  canvas.style.width = '';
  canvas.style.height = '';
  gl.viewport(0,0, canvas.width, canvas.height);
  return m4perspNegZ(0.1, 10, 1, canvas.width, canvas.height);
};

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

const prepareGeometry = (gl, program) => {
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
    const normals = computeNormals(geometry);
    geometry.attributes.normal = normals;
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

const getMv = (ms) => {
  const m = m4mul(m4rotY(ms/1000), m4rotX(-Math.PI/2));
  const v = m4view([1,1,3], [0,0,0], [0,1,0]);
  return m4mul(v, m);
}

const draw = (gl, program, geometry) => {
  const pLoc = gl.getUniformLocation(program, 'p');
  const lightLoc = gl.getUniformLocation(program, 'lightdir1');
  const halfwayLoc = gl.getUniformLocation(program, 'halfway');
  const halfwayLoc2 = gl.getUniformLocation(program, 'halfway2');
  const lightColorLoc = gl.getUniformLocation(program, 'lightcolor1');
  const lightLoc2 = gl.getUniformLocation(program, 'lightdir2');
  const lightColorLoc2 = gl.getUniformLocation(program, 'lightcolor2');
  const mvLoc = gl.getUniformLocation(program, 'mv');
  const colorLoc = gl.getUniformLocation(program, 'color');
  gl.useProgram(program);
  gl.bindVertexArray(geometry.vao);
  return (perspective, modelView, ms) => {
    gl.clearColor(...IlliniBlue);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    const lightDir = normalize([1,1,1]);
    const lightDir2 = normalize([-1,-1, 2]);
    const halfway = normalize(add(lightDir, [0,0,1]));
    const halfway2 = normalize(add(lightDir2, [0,0,1]));
    gl.uniform3fv(lightLoc, lightDir);
    gl.uniform3fv(halfwayLoc, halfway);
    gl.uniform3fv(halfwayLoc2, halfway2);
    gl.uniform3fv(lightColorLoc, [1,0,1]);
    gl.uniform3fv(lightLoc2, lightDir2);
    gl.uniform3fv(lightColorLoc2, [0,0.8,1]);
    gl.uniformMatrix4fv(pLoc, false, perspective);
    gl.uniform4fv(colorLoc, IlliniOrange);
    gl.uniformMatrix4fv(mvLoc, false, getMv(ms));
    gl.drawElements(geometry.mode, geometry.count, geometry.type, 0);
  };
}

const timestep = (draw, fill) => {
  const next = (ms) => {
    const perspective = fill();
    draw(perspective, IdentityMatrix, ms);
    requestAnimationFrame(next);
  };
  return next;
};

const setup = async () => {
  const canvas = document.querySelector('canvas');
  document.body.style.margin = '0'
  const gl = canvas.getContext('webgl2',
      {antialias: false, depth:true, preserveDrawingBuffer:true}
  )
  gl.enable(gl.DEPTH_TEST);
  gl.enable(gl.BLEND)
  gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
  const program = await compileProgram(gl);
  const monkey = await fetch('monkey.json').then(r => r.json());
  const geometry = await prepareGeometry(gl, program)(monkey);
  requestAnimationFrame(timestep(draw(gl, program, geometry), fillScreen(canvas, gl)));
}

window.addEventListener('load', setup);
