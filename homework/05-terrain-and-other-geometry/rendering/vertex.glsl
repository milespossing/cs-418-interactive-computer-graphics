#version 300 es

in vec4 position;
in vec3 normal;
out vec3 fnormal;
out vec3 mnormal;
out vec4 lightdir1;
out float height;
uniform vec3 lightdir;
uniform mat4 p;
uniform mat4 v;
uniform mat4 m;

void main() {
  mat4 mv = v * m;
  gl_Position = p * mv * position;
  // want lightdir in view, but not model coords
  lightdir1 = p * v * vec4(lightdir, 1);
  mnormal = mat3(m) * normal;
  fnormal = mat3(mv) * normal;
  // need the model coords of the position, not the model view.
  // in the model view case the height would change as the view
  // changes
  height = (m * position).y;
}
