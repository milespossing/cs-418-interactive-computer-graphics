
export const createLoop = (createRenderer, iterateState, initialState) => {
  let currentLoop = undefined;
  return (perspective, config) => {
    if (currentLoop) {
      cancelAnimationFrame(currentLoop);
    }
    const renderer = createRenderer(perspective, config);
    const iterate = (previousState, lastMs) => (ms) => {
      const deltaT = ms - lastMs;
      const nextState = iterateState(ms, deltaT)(previousState);
      renderer(nextState);
      currentLoop = requestAnimationFrame(iterate(nextState, ms));
    }
    currentLoop = requestAnimationFrame(iterate(initialState, 0));
  }
}