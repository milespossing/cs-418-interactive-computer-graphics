import { getHitBox, flipX, flipY } from './helpers.js';

const system = (entities, delta, elapsed) => {
  return entities.map((e) => {
    const hitBox = getHitBox(e);
    if (hitBox[1].get([1]) >= 1) {
      // top
      return flipY(e);
    }
    if (hitBox[0].get([1]) <= -1) {
      // bottom
      return flipY(e);
    }
    if (hitBox[0].get([0]) <= -1) {
      // left
      return flipX(e);
    }
    if (hitBox[3].get([0]) >= 1) {
      // right
      return flipX(e);
    }
    return e;
  });
}

export default system;

