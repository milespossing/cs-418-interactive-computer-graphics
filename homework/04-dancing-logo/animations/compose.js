export const compose = (fn1, fn2) => (seconds) => {
  const m1 = fn1(seconds);
  const m2 = fn2(seconds);
  return math.multiply(m1, m2);
};
