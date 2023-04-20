import {fillScreen} from "./renderUtils.js";
import {createLoop} from "./renderLoop.js";
import compile from './initializeGLSL.js'
import {renderState} from "./render.js";
import {IlliniBlue} from "./constants.js";
import {createSphereGeometry} from "./geometries/spheres.js";
import {distToPlane, rightSideOfPlane} from "./math2.js";

const GRAVITY = [0, -.98, 0];
const EPSILON = 0.0005;
const VELOCITY_RANGE = 5;

const generateModel = sphere => {
  return new Float32Array([
    sphere.radius, 0,0,0,
    0, sphere.radius,0,0,
    0, 0, sphere.radius,0,
    ...sphere.position, 1,
  ]);//m4mul(m4trans(...sphere.position), m4scale(sphere.radius, sphere.radius, sphere.radius));
}

const generateSphere = () => {
  const radius = math.random(.05, 0.1);
  const color = [math.random(), math.random(), math.random(), 1];
  const velocity = R.times(() => math.random(-VELOCITY_RANGE, VELOCITY_RANGE), 3);
  const position = [math.random(-1, 1), math.random(-1, 1), math.random(-1, 1)];
  return {
    geometry: 'sphere',
    color,
    radius,
    position,
    velocity,
  };
}

const buildInitialState = () => ({
  view: m4view([3, 1, 0], [0, 0, 0], [0, 1, 0]),
  entities: R.times(generateSphere, 50),
});

const buildConfig = () => {
  return {
    backgroundColor: IlliniBlue,
    lightColor1: [1, 1, 1],
    lightDir1: [0, 1, 0],
  };
};

const applyConstantForce = force => (velocity) => math.add(velocity, force);
const multiplyForce = multiplier => velocity => math.multiply(velocity, multiplier);
const bounceOffWall = (p, n, entity, elasticity) => velocity => {
  // todo: check this
  if (math.dot(entity.velocity, n) > 0 || (distToPlane(p,n,entity.position) > entity.radius) && rightSideOfPlane(p,n,entity.position)) {
    return velocity;
  }
  const parallelVelocity = math.abs(math.dot(entity.velocity, n));
  const newForce = math.multiply(n, parallelVelocity);
  return math.add(velocity, math.multiply(newForce, elasticity * 2));
}

const updateEntityVelocity = dt => entity =>
  R.evolve({
    velocity: R.pipe(
      applyConstantForce(math.multiply(GRAVITY, dt / 1000)),
      multiplyForce(1 - EPSILON),
      bounceOffWall([-1, 0, 0], [1, 0, 0], entity, 0.8),
      bounceOffWall([1, 0, 0], [-1, 0, 0], entity, 0.8),
      bounceOffWall([0, -1, 0], [0, 1, 0], entity, 0.8),
      bounceOffWall([0, 1, 0], [0, -1, 0], entity, 0.8),
      bounceOffWall([0, 0, 1], [0, 0, -1], entity, 0.8),
      bounceOffWall([0, 0, -1], [0, 0, 1], entity, 0.8),
    )
  })(entity);

const updateEntityPosition = dt => (entity) => {
  const scaled = math.multiply(entity.velocity, dt / 1000);
  return {...entity, position: math.add(entity.position, scaled)};
}

const advanceState = (ms, dt) => R.pipe(
  R.identity,
  R.evolve({
    entities: R.pipe(
      R.map(updateEntityPosition(dt)),
      R.map(updateEntityVelocity(dt)),
    ),
  }),
);

const setup = async () => {
  const gl = document.querySelector('canvas').getContext('webgl2',
    // optional configuration object: see https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext
    {antialias: false, depth: true, preserveDrawingBuffer: true}
  );
  window.gl = gl;
  gl.enable(gl.DEPTH_TEST);
  gl.enable(gl.BLEND)
  gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
  const program = await compile(gl, './shaders/vertex.glsl', './shaders/fragment.glsl');
  const sphere = await createSphereGeometry(gl, program);
  const geometry = {
    sphere: [sphere, generateModel],
  };
  const render = await renderState(gl, program, geometry);
  window.createNewLoop = createLoop(render, advanceState, buildInitialState());
  window.config = buildConfig();
  fillScreen();
}

window.addEventListener('load', setup);
window.addEventListener('resize', fillScreen);
