
const system = (entities, delta, elapsed) => {
  return entities.map(e => {
    if (!e.velocity) return e;
    // console.log(e);
    if (e.model.get([1, 3]) >= 1) return e;
    const vel = math.multiply(e.velocity, delta / 1000);
    const deltaT = math.matrix([[0, 0, 0, vel.get([0])], [0,0,0, vel.get([1])], [0,0,0,0], [0,0,0,0]]);
    return { ...e, model: math.add(deltaT, e.model) };
  })
};

export default system;
