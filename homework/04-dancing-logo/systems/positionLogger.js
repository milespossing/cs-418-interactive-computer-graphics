
const system = (entities) => {
  entities.forEach(e => console.table(math.column(e.model, 3).toArray()));
  return entities;
}

export default system;
