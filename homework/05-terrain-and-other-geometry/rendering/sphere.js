
// Create an icosahedron with a given radius and number of subdivisions
function createIcosahedron(radius, subdivisions) {
  const t = (1 + Math.sqrt(5)) / 2; // Golden ratio
  const vertices = [
    [-1, t, 0], [1, t, 0], [-1, -t, 0], [1, -t, 0],
    [0, -1, t], [0, 1, t], [0, -1, -t], [0, 1, -t],
    [t, 0, -1], [t, 0, 1], [-t, 0, -1], [-t, 0, 1]
  ];
  const faces = [
    [0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11],
    [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
    [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
    [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]
  ];
  const result = {vertices: [], indices: []};

  // Subdivide each face recursively
  for (let i = 0; i < faces.length; i++) {
    subdivide(vertices[faces[i][0]], vertices[faces[i][1]], vertices[faces[i][2]], radius, subdivisions, result);
  }

  return result;
}

// Recursive subdivision function
function subdivide(v1, v2, v3, radius, subdivisions, result) {
  if (subdivisions <= 0) {
    const index = result.vertices.length / 3;
    result.vertices.push(...v1, ...v2, ...v3);
    result.indices.push(index, index + 1, index + 2);
    return;
  }

  // Calculate midpoints of each edge
  const v12 = normalize(add(v1, v2));
  const v23 = normalize(add(v2, v3));
  const v31 = normalize(add(v3, v1));

  // Recursively subdivide new triangles
  subdivide(v1, v12, v31, radius, subdivisions - 1, result);
  subdivide(v2, v23, v12, radius, subdivisions - 1, result);
  subdivide(v3, v31, v23, radius, subdivisions - 1, result);
  subdivide(v12, v23, v31, radius, subdivisions - 1, result);
}

// Helper functions
function add(v1, v2) {
  return [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]];
}

function normalize(v) {
  const length = Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
  return [v[0] / length * radius, v[1] / length * radius, v[2] / length * radius];
}

const createSphere = () => {

};

export default createSphere;
