import { compileProgram, transformers } from './animations/index.js';

const draw = (gl, program) => {
  // performs another iteration
  const loop = (geometry) => (ms) => {
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.useProgram(program);

    // the transformer to use for the next frame
    const transform = new Float32Array(window.transformer(ms / 1000));
    const transformBindPoint = gl.getUniformLocation(program, 'transform');
    gl.uniformMatrix4fv(transformBindPoint, false, transform);

    gl.bindVertexArray(geometry.vao);
    gl.drawElements(geometry.mode, geometry.count, geometry.type, 0);
    requestAnimationFrame(loop(window.geometry));
  };
  loop(window.geometry)(0);
};

const animations = {
  static: ['static', 'logo'],
  rotation: ['rotation', 'logo'],
}

const setAnimation = () => {
  const radios = document.getElementsByName('implemented');
  let checked;
  for (let i = 0; i < radios.length; i++) {
    if (radios[i].checked) {
      checked = radios[i];
      break;
    }
  }
  if (!checked) throw Error('animation not found')
  const [transformerType, geometryType] = animations[checked.value];
  const transformer = transformers[transformerType];
  const geometry = window.geometries[geometryType];
  if (!transformer || !geometry) throw Error('Invalid animation');
  window.transformer = transformer;
  window.geometry = geometry;
}

const setup = async () => {
  document.getElementsByName('implemented')
    .forEach(r => r.addEventListener('change', setAnimation));
  const gl = document.querySelector('canvas').getContext('webgl2');
  const logo = await fetch('logo.json').then(b => b.json());
  const geometryData = { logo };
  console.log(geometryData);
  const [program, geometries] = await compileProgram(gl, geometryData);
  console.log(geometries);
  window.geometries = geometries;
  setAnimation();

  draw(gl, program);
};

window.addEventListener('load', setup);
