// This represents the game loop

export const executeLoop = (gl, program, transformState, renderState) => (state) => {
  if (window.animationFrame)
    cancelAnimationFrame(window.animationFrame);
  const render = (state, last, iteration = 0) => (ms) => {
    const deltaT = last ? ms - last : 0;
    const nextState = transformState(state, ms, deltaT, window.keysBeingPressed ?? {});
    renderState(nextState, iteration);
    window.animationFrame = requestAnimationFrame(render(nextState, ms, iteration + 1));
  };
  window.animationFrame = requestAnimationFrame(render(state));
};

