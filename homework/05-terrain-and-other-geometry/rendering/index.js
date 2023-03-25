import renderModel from './model.js';
import terrain from './terrain.js';

const IlliniOrange = new Float32Array([1, 0.373, 0.02, 1]);
const IlliniBlue = new Float32Array([0.075, 0.16, 0.292, 1])
const IdentityMatrix = new Float32Array([1,0,0,0, 0,1,0,0, 0,0,1,0, 0,0,0,1])
const Black = new Float32Array([0,0,0,1]);

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

const compileProgram = async (gl) => {
  const vertex = await fetch('rendering/vertex.glsl').then(r => r.text());
  const fragment = await fetch('rendering/fragment.glsl').then(r => r.text());
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

const getMaxMin = (acc, [head, ...tail]) => {
  if (head === undefined) return acc;
  if (head[1] > acc.max) {
    return getMaxMin({...acc, max: head[1]}, tail);
  } else if (head[1] < acc.min) {
    return getMaxMin({...acc, min: head[1]}, tail);
  }
  return getMaxMin(acc, tail);
}

const curriedAdd = R.curry((a,b) => add(a,b));
const curriedMult = R.curry((a,b) => m4mul(a,b));

const draw = (gl, program) => (perspective) => ([geom, options]) => {
  gl.useProgram(program);
  console.log(options);
  const maxHeightLoc = gl.getUniformLocation(program, 'maxHeight');
  const minHeightLoc = gl.getUniformLocation(program, 'minHeight');
  const cliffColorLoc = gl.getUniformLocation(program, 'cliffColor');
  const pLoc = gl.getUniformLocation(program, 'p');
  const vLoc = gl.getUniformLocation(program, 'v');
  const mLoc = gl.getUniformLocation(program, 'm');
  const lightLoc = gl.getUniformLocation(program, 'lightdir1');
  const lightColorLoc = gl.getUniformLocation(program, 'lightcolor1');
  const halfwayLoc = gl.getUniformLocation(program, 'halfway');
  const color1Loc = gl.getUniformLocation(program, 'color1');
  const color2Loc = gl.getUniformLocation(program, 'color2');
  const color3Loc = gl.getUniformLocation(program, 'color3');
  const blinnAmountLoc = gl.getUniformLocation(program, 'blinnAmount');
  const { max, min } = getMaxMin({max:0, min:0}, geom.attributes.position);
  const render = (ms) => {
    gl.clearColor(...IlliniBlue); // f(...[1,2,3]) means f(1,2,3)
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    const v = m4view([math.cos(ms / 1000),1,math.sin(ms / 1000)], [0,0,0], [0,1,0]);
    const toView = curriedMult(v);
    const geometry = prepareGeometry(gl, program)(geom);
    const m = options.model ?? IdentityMatrix;
    const [color1, color2, color3] = options.colors ?? [IlliniOrange, IlliniOrange, IlliniOrange];
    const cliffColor = options.cliffColor ?? color1;
    const lightDir = normalize(options.lightDir ?? [1,1,1]);

    const lightColor = options.lightColor ?? [1,1,1];
    const mv = toView(m);
    const getHalfway = R.compose(normalize, curriedAdd([0,0,1]));
    const halfway = getHalfway(lightDir);

    gl.uniform1f(blinnAmountLoc, options.shiny ? 1 : 0);
    gl.uniform1f(maxHeightLoc, max);
    gl.uniform1f(minHeightLoc, min);
    gl.uniform3fv(halfwayLoc, halfway);
    gl.uniform3fv(lightLoc, lightDir);
    gl.uniform4fv(color1Loc, color1);
    gl.uniform4fv(color2Loc, color2);
    gl.uniform4fv(color3Loc, color3);
    gl.uniform4fv(cliffColorLoc, cliffColor);
    gl.uniformMatrix4fv(pLoc, false, perspective);
    gl.uniformMatrix4fv(vLoc, false, v);
    gl.uniformMatrix4fv(mLoc, false, m);
    gl.uniform3fv(lightColorLoc, lightColor);
    gl.bindVertexArray(geometry.vao);
    gl.drawElements(geometry.mode, geometry.count, geometry.type, 0);
    window.animationFrame = requestAnimationFrame(render);
  };
  return requestAnimationFrame(render);
};

/**
 * Draw one frame
 */
const setScene = async (gl) => {
  const program = await compileProgram(gl);
  const drawScene = draw(gl, program);
  return (scene, options) => {
    if (window.animationFrame) {
      cancelAnimationFrame(window.animationFrame);
    }
    const output = async (perspective) => {
      switch (scene) {
        case 'model':
          window.animationFrame = await renderModel(options).then(drawScene(perspective));
          break;
        case 'terrain':
          window.animationFrame = await terrain(options).then(drawScene(perspective));
          break;
      }
    };
    output._tag = 'hello world';
    return output;
  }
};

export default setScene;
