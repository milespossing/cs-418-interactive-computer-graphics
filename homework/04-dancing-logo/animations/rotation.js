
// A simple rotation to make sure that everything works
export default (seconds) => {
  const rads = seconds / (2 * Math.PI);
  return [
    Math.cos(rads), -1.0 * Math.sin(rads), 0, 0,
    Math.sin(rads),        Math.cos(rads), 0, 0,
                 0,                     0, 1, 0,
                 0,                     0, 0, 1,
  ];
}
