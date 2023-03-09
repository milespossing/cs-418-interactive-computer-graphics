import { getHitBox, flipX, flipY } from './helpers.js';

const system = (entities, delta, elapsed) => {
  return entities.map((e) => {
    const hitBox = getHitBox(e);
    if (hitBox[1].get([1]) >= 1 && e.velocity.get([1]) > 0) {
      // top
      return flipY(e);
    }
    if (hitBox[0].get([1]) <= -1 && e.velocity.get([1]) < 0) {
      // bottom
      return flipY(e);
    }
    if (hitBox[0].get([0]) <= -1 && e.velocity.get([0]) < 0) {
      // left
      return flipX(e);
    }
    if (hitBox[3].get([0]) >= 1 && e.velocity.get([0]) > 0) {
      // right
      return flipX(e);
    }
    return e;
  });
}

export default system;

