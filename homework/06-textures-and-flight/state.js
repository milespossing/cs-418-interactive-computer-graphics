import { getTrans, setTrans, m4mulC, m4mulR } from './math2.js';

const initialFlightTransform = m4view([0.5,1,0.5], [0,0,0], [0,1,0]);
const initialVehicleTransform = m4view([0,0,0], [1,0,0], [0,1,0]);

export const initializeState = (gl, program) => {
  return {
    transform: initialFlightTransform,
    vehicle: false,
    fog: false,
  };
}

const toggleVehicleMode = (keys, lastKeys) => state => {
  if (!keys['g'] || lastKeys['g']) return state;
  // if vehicle, then our next will be flight
  const nextTransform = state.vehicle ? initialFlightTransform : initialVehicleTransform;
  // set transform and flip vehicle
  return { ...state, vehicle: !state.vehicle, transform: nextTransform };
}

const toggleFogMode = (keys, lastKeys) => state => {
  if (!keys['f'] || lastKeys['f']) return state;
  console.log('activating fog');
  // if vehicle, then our next will be flight
  // set transform and flip vehicle
  return { ...state, fog: !state.fog };
}

const constrainVehiclePosition = (getTerrainHeight) => state => {
  if (!state.vehicle) return state;
  const [x,_,z] = getTrans(state.transform);
  const y = getTerrainHeight(x,z);
  console.log([x,y,z]);
  return { ...state, transform: setTrans([x,y,z], state.transform) };
}

const motion = (keys, velocity, delta) => state => {
  const translationDelta = [0,0,0];
  const deltaV = velocity * delta;
  if (keys['w']) {
    translationDelta[2] += deltaV;
  }
  if (keys['a']) {
    translationDelta[0] += deltaV;
  }
  if (keys['d']) {
    translationDelta[0] -= deltaV;
  }
  if (keys['s']) {
    translationDelta[2] -= deltaV;
  }
  if (keys['q']) {
    translationDelta[1] -= deltaV;
  }
  if (keys['e']) {
    translationDelta[1] += deltaV;
  }
  if (translationDelta.every(i => !i)) return state;
  const translate = m4mulC(m4trans(...translationDelta));

  return {
    ...state,
    transform: translate(state.transform),
  };
}

const writeState = (message, write) => state => {
  if (write) { console.log(message, state); }
}

const rotation = (keys, velocity, delta) => state => {
  const deltaT = velocity * delta;
  let panLeft = 0;
  let tiltDown = 0;
  if (keys['ArrowDown']) {
    tiltDown = 1;
  } else if (keys['ArrowUp']) {
    tiltDown = -1;
  }
  if (keys['ArrowLeft']) {
    panLeft = -1;
  } else if (keys['ArrowRight']) {
    panLeft = 1;
  }
  const tilter = tiltDown ? m4mulC(m4rotX(deltaT * tiltDown)) : R.identity;
  const panner = panLeft ? m4mulC(m4rotY(deltaT * panLeft)) : R.identity;
  const inc = R.pipe(panner, tilter);
  return { ...state, transform: inc(state.transform) };
}

const setNextState = (state) => { window.state = state };

const incrementState = (getTerrainHeight) => (state, ms, deltaT, keys, lastKeys, iteration) => {
  const pipeline = R.pipe(
    // R.tap(writeState('start', iteration % 500 === 0)),
    toggleFogMode(keys, lastKeys),
    // toggleVehicleMode(keys, lastKeys),
    rotation(keys, 0.001, deltaT),
    // R.tap(writeState('rotated', iteration % 500 === 0)),
    motion(keys, 0.0001, deltaT),
    // constrainVehiclePosition(getTerrainHeight),
    // R.tap(writeState('translated', iteration % 500 === 0)),
    // R.tap(writeState('state', keys[' '])),
    R.tap(setNextState),
  );
  return pipeline(state);
};

export default incrementState;

