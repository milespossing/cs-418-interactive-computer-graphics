import {distToPlane, rightSideOfPlane} from "./math2.js";

export const updateEntityPosition = dt => (entity) => {
  const scaled = math.multiply(entity.velocity, dt / 1000);
  return {...entity, position: math.add(entity.position, scaled)};
}

export const bounceOffWall = (p, n, entity, radius, elasticity) => velocity => {
  // todo: check this
  if (math.dot(entity.velocity, n) > 0 || (distToPlane(p,n,entity.position) > radius) && rightSideOfPlane(p,n,entity.position)) {
    return velocity;
  }
  const parallelVelocity = math.abs(math.dot(entity.velocity, n));
  const newForce = math.multiply(n, parallelVelocity);
  return math.add(velocity, math.multiply(newForce, elasticity * 2));
}

export const bounceIntoView = (p, v, entity, radius) => velocity => {
  const perspectiveView = m4mul(p,v);
  const perspectiveLocation = m4mul(perspectiveView, [...entity.position, 1]);
  const perspectiveXyz = R.take(3, perspectiveLocation);
  const clipSpace = math.divide(perspectiveXyz,perspectiveLocation[3]);
  const perspectiveVelocity = m4mul(perspectiveView, [...entity.velocity, 0]);
  if (clipSpace[0] - radius < -1 && perspectiveVelocity[0] < 0) {
    return [velocity[0] * -1, velocity[1], velocity[2]];
  }
  if (clipSpace[0] + radius > 1 && perspectiveVelocity[0] > 0) {
    return [velocity[0] * -1, velocity[1], velocity[2]];
  }
  if (clipSpace[1] - radius < -1 && perspectiveVelocity[1] < 0) {
    return [velocity[0], velocity[1], velocity[2] * -1];
  }
  if (clipSpace[1] + radius > 1 && perspectiveVelocity[1] > 0) {
    return [velocity[0], velocity[1], velocity[2] * -1];
  }
  return velocity;
}


export const rotateVelocity = speed => priority => left => dt =>
  m4rotY(speed * priority * (dt / 1000) * (left ? -1 : 1));
