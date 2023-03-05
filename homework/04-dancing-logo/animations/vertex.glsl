#version 300 es

uniform mat4 mv;
uniform bool transform_first;
uniform float seconds;

in vec4 position;
in vec4 color;

out vec4 vColor;

void main() {
  vColor = color;
  vec4 pos_prime = transform_first && gl_VertexID == 1 ? vec4(position.x * sin(seconds), position.y * cos(seconds), position.zw) : position;
  gl_Position = mv * pos_prime;
}
