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
  if (overlapX < overlapY) {
    return leftMinX < rightMinX
      ? [left.velocity.get([0]) < 0 ? left : flipX(left), right.velocity.get([0]) > 0 ? right : flipX(right)]
      : [right.velocity.get([0]) < 0 ? right : flipX(right), left.velocity.get([0]) > 0 ? left : flipX(left)];
  } else {
    return leftMinY < rightMinY
      ? [left.velocity.get([1]) < 0 ? left : flipY(left), right.velocity.get([1]) > 0 ? right : flipY(right)]
      : [right.velocity.get([1]) < 0 ? right : flipY(right), left.velocity.get([1]) > 0 ? left : flipY(left)];
  }
  console.log('collide');

  return [left, right];
};

export default system;
