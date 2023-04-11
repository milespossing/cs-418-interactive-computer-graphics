// This represents the game loop

export const executeLoop = (gl, program, transformState, renderState) => (state) => {
  if (window.animationFrame)
    cancelAnimationFrame(window.animationFrame);
  const render = (state, last, iteration = 0, lastKeyState = {}) => (ms) => {
    const deltaT = last ? ms - last : 0;
    const currentKeys = window.keysBeingPressed ? { ...window.keysBeingPressed } : {};
    const nextState = transformState(state, ms, deltaT, currentKeys, lastKeyState, iteration);
    renderState(nextState, iteration);
    window.animationFrame = requestAnimationFrame(render(nextState, ms, iteration + 1, currentKeys));
  };
  window.animationFrame = requestAnimationFrame(render(state));
};

