export const ident = () => math.identity(4);

export const scale = (amount) => () => [
  amount, 0, 0, 0,
  0, amount, 0, 0,
  0, 0, amount, 0,
  0, 0, 0, 1,
]
