import { getHitBox, flipX, flipY } from './helpers.js';

const system = ([left, right]) => {
  const leftBox = getHitBox(left);
  const rightBox = getHitBox(right);
  const [leftMinX, leftMinY] = leftBox[0].toArray();
  const [leftMaxX, leftMaxY] = leftBox[2].toArray();
  const [rightMinX, rightMinY] = rightBox[0].toArray();
  const [rightMaxX, rightMaxY] = rightBox[2].toArray();
  if (leftMinX > rightMaxX || rightMinX > leftMinX) {
    return [left, right];
  }
  if (leftMinY > rightMaxY || rightMinY > leftMaxY) {
    return [left, right];
  }

  const overlapX = math.min(leftMaxX, rightMaxX) - math.max(leftMinX, rightMinX);
  const overlapY = math.min(leftMaxY, rightMaxY) - math.max(leftMinY, rightMinY);
  // TODO: determine which side this is actually happening on for more consistent results
  if (overlapX < overlapY) {
    return [left, right].map(flipX);
  } else {
    return [left, right].map(flipY);
  }
  console.log('collide');

  return [left, right];
};

export default system;
