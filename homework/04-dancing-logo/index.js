import { compileProgram, transformers } from './animations/index.js';
import { eventLoop } from './collision.js';

const cpuProcessor = (amplitude) => (data, ms) => {
  const { attributes } = data;
  const { position } = attributes;
  const [ head, ...rest ] = position;
  const [x, y] = head;
  const deltaX = amplitude * Math.sin(ms / 1000);
  const deltaY = amplitude * Math.cos(ms / 1000);
  return { ...data, attributes: { ...attributes, position: [[x + deltaX, y + deltaY], ...rest]}};
}

const buildAnimations = geometries => ({
  static: [{
    mode: 'static',
    geometry: geometries['logo'],
  }],
  rotation: [{
    mode: 'mv',
    mv: transformers['rotation'],
    geometry: geometries['logo'],
  }],
  rotation2: [{
    mode: 'mv',
    mv: transformers['rotation'],
    geometry: geometries['triangle']
  }],
  growAndShrink: [{ mode: 'mv',
    mv: transformers['growAndShrink'],
    geometry: geometries['logo']
  }],
  simpleDance: [{ mode: 'mv', mv: transformers['simpleDance'], geometry: geometries['logo'] }],
  triangleSimple: [{ mode: 'mv', mv: transformers['simpleDance'], geometry: geometries['triangle'] }],
  cpuTransform: [{ mode: 'mv', mv: transformers['growAndShrink'], geometry: geometries['logo'], preProcess: cpuProcessor(0.25) }],
  gpuTransform: [{ mode: 'mv', mv: transformers['growAndShrink'], geometry: geometries['logo'], gpuTransform: true } ],
});

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
  window.entities = window.animations[checked.value];
}

const setup = async () => {
  document.getElementsByName('implemented')
    .forEach(r => r.addEventListener('change', setAnimation));
  const gl = document.querySelector('canvas').getContext('webgl2');
  const logo = await fetch('logo.json').then(b => b.json());
  const triangle = await fetch('triangle.json').then(b => b.json());
  const geometryData = { logo, triangle };
  const [program, setupGeometry] = await compileProgram(gl);

  console.log(setupGeometry);
  window.setupGeometry = setupGeometry;
  window.processGeometry = setupGeometry;
  window.geometries = geometryData;
  window.animations = buildAnimations(geometryData);
  setAnimation();
  eventLoop(gl, program, setupGeometry);
};


window.addEventListener('load', setup);
