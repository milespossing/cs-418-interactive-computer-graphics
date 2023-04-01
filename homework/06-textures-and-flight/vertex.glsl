#version 300 es

in vec4 position;
in vec3 normal;
out vec3 mnormal;
out vec3 fnormal;
out float height;
out vec4 lightdir1;
uniform vec3 lightdir;
uniform mat4 p;
uniform mat4 v;
uniform mat4 m;

void main() {
  mat4 mv = v * m;
  gl_Position = p * mv * position;
  lightdir1 = p * v * vec4(lightdir, 1);
  mnormal = mat3(m) * normal;
  fnormal = mat3(mv) * normal;
  height = (m * position).y;
}
