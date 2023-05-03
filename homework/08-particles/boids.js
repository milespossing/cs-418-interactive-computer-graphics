import {fillScreen} from "./renderUtils.js";
import compile from "./initializeGLSL.js";
import {createSphereGeometry} from "./geometries/spheres.js";
import {createLoop} from "./renderLoop.js";
import {renderState} from "./render.js";
import {IlliniBlue} from "./constants.js";
import {bounceIntoView, rotateVelocity, updateEntityPosition} from "./stateIterators.js";
import {homogenize, homogenizeRot3} from "./math2.js";

const BOID_SIZE = 0.03;
const BOID_BOUNCE_RADIUS = BOID_SIZE;
const BOID_COUNT = 50;

const randomOnRange = (min, max) => () => math.random(min, max);

// TODO: rotate towards the camera based on the current position
const generateModel = (entity, view) => {
  return new Float32Array([
    entity.scale, 0, 0, 0,
    0, entity.scale, 0, 0,
    0, 0, entity.scale, 0,
    ...entity.position, 1,
  ]);
}

const buildConfig = () => {
  return {
    backgroundColor: IlliniBlue,
    lightColor1: [1, 1, 1],
    lightDir1: [0, 1, 0],
  };
};

const generateBoid = () => {
  const ran1 = randomOnRange(-1.5,1.5);
  const ran2 = randomOnRange(0,1);
  const position = [ran1(), 0, ran1()];
  const velocity = [ran1() / 3, 0, ran1() / 3];
  const color = R.times(ran2, 3);
  return {geometry: 'sphere', scale: BOID_SIZE, position, velocity, color};
}

const buildInitialState = () => {
  return {
    entities: R.times(generateBoid, BOID_COUNT),
    view: m4view([0, 3, 0], [0,0,0], [0,0,1]),
  }
}

const advancePosition = (dt) => R.evolve({
  entities: R.map(updateEntityPosition(dt))
})

const FLOCK_DISTANCE = 0.5;
const SEPERATION_DISTANCE = 0.3;
const MINIMUM_VELOCITY = 0.5;

const enforceMinimumVelocity = velocity => {
  const n = math.norm(velocity);
  if (n > MINIMUM_VELOCITY)
    return velocity;
  return math.multiply(velocity, MINIMUM_VELOCITY / n);
}

const getAngle = R.compose(
  math.abs,
  math.acos,
  math.dot,
);

// is e2 a flockmate of e1?
const isFlockMate = R.curry((r, t, e1, e2) => {
  if (math.norm(math.subtract(e1.position, e2.position)) > r) return false;
  const forward = normalize(e1.velocity);
  const direction = normalize(math.subtract(e2.position, e1.position));
  const angle = getAngle(forward, direction);
  return angle < t;
})

const getFlockMates = R.curry((state, entity) => {
  return R.filter(isFlockMate(FLOCK_DISTANCE, Math.PI, entity), state.entities);
});

const baseRotate = rotateVelocity(2);

const rotateAlign = baseRotate(0.3);
const rotateCohesion = baseRotate(0.5);
const rotateSeparation = baseRotate(-0.8);

const alignDirections = (entity, flockmates, dt) => velocity => {
  const averageDirection = flockmates.reduce((a, b) => math.add(a, normalize(b.velocity)), [0,0,0]);
  const cross = math.cross(entity.velocity, averageDirection);
  const left = cross[1] < 0;
  const h_velocity = [...velocity, 1];
  const rotator = rotateAlign(left)(dt);
  const rotated = m4mul(rotator, h_velocity);
  return R.take(3, rotated);
}

const rotateToCohesion = (entity, flockmates, dt) => velocity => {
  const averagePosition = math.divide(flockmates.reduce((a, b) => math.add(a, b.position), [0,0,0]), flockmates.length);
  const toPosition = math.subtract(averagePosition, entity.position);
  const cross = math.cross(entity.velocity, toPosition);
  const left = cross[1] < 0;
  const h_velocity = [...velocity, 1];
  const rotator = rotateCohesion(left)(dt);
  const rotated = m4mul(rotator, h_velocity);
  return R.take(3, rotated);
}

const rotateFromSeparation = (entity, flockmates, dt) => velocity => {
  const closer = flockmates.filter(m => math.norm(math.subtract(m.position, entity.position)) < SEPERATION_DISTANCE);
  const averagePosition = math.divide(closer.reduce((a,b) => math.add(a, b.position), [0,0,0]), closer.length);
  const toPosition = math.subtract(averagePosition, entity.position);
  const cross = math.cross(entity.velocity, toPosition);
  const left = cross[1] < 0;
  const h_velocity = [...velocity, 1];
  const rotator = rotateSeparation(left)(dt);
  const rotated = m4mul(rotator, h_velocity);
  return R.take(3, rotated);
}

const processFlocking = (entity, state, dt) => velocity => {
  const flockmates = getFlockMates(state, entity);
  if (flockmates.length === 0) return velocity;
  return R.pipe(
    alignDirections(entity, flockmates, dt),
    rotateToCohesion(entity, flockmates, dt),
    rotateFromSeparation(entity, flockmates, dt),
    enforceMinimumVelocity,
  )(velocity);
};

const updateVelocity = (dt, state, perspective) => entity => R.evolve({
  velocity: R.pipe(
    bounceIntoView(perspective, state.view, entity, BOID_BOUNCE_RADIUS),
    processFlocking(entity, state, dt),
  )
})(entity);

const updateVelocities = (dt, perspective) => state => R.pipe(
  R.evolve({
    entities: R.map(updateVelocity(dt, state, perspective)),
  })
)(state)

const advanceState = (ms, dt, { perspective }) => R.pipe(
  advancePosition(dt),
  updateVelocities(dt, perspective),
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
