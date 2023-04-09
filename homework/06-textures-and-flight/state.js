import { getTrans, setTrans, m4mulC, m4mulR } from './math2.js';

export const initializeState = (gl, program) => {
  return {
    transform: m4view([1,3,2], [0,0,0], [0,1,0]),
  };
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

const incrementState = (state, ms, deltaT, keys, iteration) => {
  const pipeline = R.pipe(
    R.tap(writeState('start', iteration % 500 === 0)),
    rotation(keys, 0.001, deltaT),
    R.tap(writeState('rotated', iteration % 500 === 0)),
    motion(keys, 0.001, deltaT),
    R.tap(writeState('translated', iteration % 500 === 0)),
  );
  return pipeline(state);
};

export default (...args) => {
  const newState = incrementState(...args);
  window.state = newState;
  return newState;
};

