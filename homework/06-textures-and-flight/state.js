import { getTrans, setTrans } from './math2.js';

export const initializeState = (gl, program) => {
  const translation = [0,1,-1];
  return {
    transform: m4view([1,3,2], [0,0,0], [0,1,0]),
  };
}

const motion = (keys, velocity, delta) => state => {
  const translationDelta = [0,0,0,0];
  if (keys['w']) {
    translationDelta[2] += 1;
  }
  if (keys['a']) {
    translationDelta[0] += 1;
  }
  if (keys['d']) {
    translationDelta[0] -= 1;
  }
  if (keys['s']) {
    translationDelta[2] -= 1;
  }
  if (translationDelta.every(i => !i)) return state;
  const s = velocity * delta;
  const scaled = m4mul(m4scale(s,s,s), normalize(translationDelta));
  const translation = getTrans(state.transform);
  const moved = add(translation, scaled);
  const result = setTrans(moved, state.transform);

  return { ...state, transform: result };
}

const incrementState = (state, ms, deltaT, keys) => {
  const pipeline = R.pipe(
    motion(keys, 0.001, deltaT),
  );
  return pipeline(state);
};

export default (...args) => {
  const newState = incrementState(...args);
  window.state = newState;
  return newState;
};

