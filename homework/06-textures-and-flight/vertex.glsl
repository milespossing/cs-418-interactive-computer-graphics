#version 300 es

in vec4 position;
in vec3 normal;
in vec2 aTexCoord;
out vec3 mnormal;
out vec3 vnormal;
out vec3 fnormal;
out float height;
out vec4 lightdir1;
out vec2 vTexCoord;
uniform vec3 lightdir;
uniform mat4 p;
uniform mat4 v;
uniform mat4 m;
uniform mat4 mv;

void main() {
  gl_Position = p * mv * position;
  lightdir1 = vec4(lightdir, 1);
  mnormal = mat3(m) * normal;
  fnormal = mat3(mv) * normal;
  vnormal = mat3(v) * normal;
  vTexCoord = aTexCoord;
}
