#version 300 es

in vec4 position;
in vec3 normal;
out vec3 mnormal;
out vec4 v_lightdir1;
uniform vec3 lightdir1;
uniform mat4 p;
uniform mat4 v;
uniform mat4 m;
uniform mat4 mv;

void main() {
  gl_Position = p * mv * position;
  v_lightdir1 = vec4(lightdir1, 1);
  mnormal = mat3(m) * normal;
}
