#version 300 es

uniform mat4 transform;

in vec4 position;
in vec4 color;

out vec4 vColor;

void main() {
    vColor = color;
    gl_Position = transform * position;
}