
export const getHitBox = (entity) => {
  const [minX, minY, maxX, maxY] = entity.geometry.attributes.position.reduce((agg, p) => {
    const _minX = math.min(agg[0], p[0]);
    const _minY = math.min(agg[1], p[1]);
    const _maxX = math.max(agg[2], p[0]);
    const _maxY = math.max(agg[3], p[1]);
    return [_minX, _minY, _maxX, _maxY];
  }, [0, 0, 0, 0]);
  const vectors = [[minX, minY, 0, 1], [minX, maxY, 0, 1], [maxX, maxY, 0, 1], [maxX, minY, 0, 1]];
  const extents = vectors.map((v) => math.multiply(entity.model, v));
  return extents;
}

export const flipY = e => ({ ...e, velocity: math.matrix([ e.velocity.get([0]), e.velocity.get([1]) * -1]) });
export const flipX = e => ({ ...e, velocity: math.matrix([ e.velocity.get([0]) * -1, e.velocity.get([1])]) });
