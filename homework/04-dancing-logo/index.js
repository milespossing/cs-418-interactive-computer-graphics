import { compileProgram, transformers } from './animations/index.js';
import { eventLoop } from './renderLoop.js';
import velocitySystem from './systems/velocitySystem.js';
import sideReflectSystem from './systems/sideReflectSystem.js';
import positionLogger from './systems/positionLogger.js';
import entityCollision from './systems/entityCollisionSystem.js';

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
  static: {
    entities: [{
      mode: 'static',
      geometry: geometries['logo'],
    }],
  },
  rotation: {
    entities: [{
      mode: 'mv',
      mv: transformers['rotation'],
      geometry: geometries['logo'],
    }],
  },
  rotation2: {
    entities: [{
      mode: 'mv',
      mv: transformers['rotation'],
      geometry: geometries['triangle']
    }],
  },
  growAndShrink: {
    entities: [{ mode: 'mv',
      mv: transformers['growAndShrink'],
      geometry: geometries['logo']
    }],
  },
  simpleDance: {
    entities: [{ mode: 'mv', mv: transformers['simpleDance'], geometry: geometries['logo'] }],
  },
  triangleSimple: { entities: [{ mode: 'mv', mv: transformers['simpleDance'], geometry: geometries['triangle'] }]},
  cpuTransform: { entities: [{ mode: 'mv', mv: transformers['growAndShrink'], geometry: geometries['logo'], preProcess: cpuProcessor(0.25) }]},
  gpuTransform: { entities: [{ mode: 'mv', mv: transformers['growAndShrink'], geometry: geometries['logo'], gpuTransform: true } ] },
  multipleItems: {
    entities: [
      {
        mode: 'static',
        geometry: geometries['logo'],
        model: math.matrix([
          [0.2,0,0,-0.5],
          [0,0.2,0,0.5],
          [0,0,0.2,0],
          [0,0,0,1],
        ]),
      },
      {
        mode: 'static',
        geometry: geometries['triangle'],
        model: math.matrix([
          [0.3,0,0,0.5],
          [0,0.3,0,-0.2],
          [0,0,0.3,0],
          [0,0,0,1],
        ])
      }
    ]
  },
  velocity: {
    entities: [
      {
        mode: 'static',
        geometry: geometries['logo'],
        model: math.matrix([
          [0.2,0,0,0],
          [0,0.2,0,0],
          [0,0,0.2,0],
          [0,0,0,1],
        ]),
        velocity: math.matrix([0.8, 1]),
      }
    ],
    systems: [sideReflectSystem, velocitySystem],
  },
  collision: {
    entities: [
      {
        mode: 'static',
        geometry: geometries['logo'],
        model: math.matrix([
          [0.2,0,0,0.2],
          [0,0.2,0,0.2],
          [0,0,0.2,0],
          [0,0,0,1],
        ]),
        velocity: math.matrix([0.8, 1]),
      },
      {
        mode: 'static',
        geometry: geometries['logo'],
        model: math.matrix([
          [0.2,0,0,-0.2],
          [0,0.2,0,-0.2],
          [0,0,0.2,0],
          [0,0,0,1],
        ]),
        velocity: math.matrix([-1, 0.8]),
      },
    ],
    systems: [sideReflectSystem, entityCollision, velocitySystem],
  },
  psychedelic: {
    entities: [
      {
        mode: 'static',
        geometry: geometries['quad'],
      }
    ],
    fragment: 'psychedelic',
  }
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
  const { entities, systems, fragment } = window.animations[checked.value];
  window.entities = entities;
  window.systems = systems ?? [];
  window.psychedelic = fragment === 'psychedelic';
  console.log(window.psychedelic);
}

const setup = async () => {
  document.getElementsByName('implemented')
    .forEach(r => r.addEventListener('change', setAnimation));
  const gl = document.querySelector('canvas').getContext('webgl2');
  const toJson = b => b.json();
  const logo = await fetch('logo.json').then(toJson);
  const triangle = await fetch('triangle.json').then(toJson);
  const quad = await fetch('quad.json').then(toJson);
  const geometryData = { logo, triangle, quad };
  const [program, setupGeometry] = await compileProgram(gl);

  // console.log(setupGeometry);
  window.setupGeometry = setupGeometry;
  window.processGeometry = setupGeometry;
  window.geometries = geometryData;
  window.animations = buildAnimations(geometryData);
  setAnimation();
  eventLoop(gl, program, setupGeometry);
};


window.addEventListener('load', setup);
