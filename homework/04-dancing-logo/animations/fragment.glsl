#version 300 es
precision highp float;

in vec4 vColor;

uniform float seconds;
uniform bool psychedelic;

out vec4 fragColor;

void main() {
  if (psychedelic) {
    float r = sin(seconds + (gl_FragCoord[0] * 0.02 * cos(seconds / 10.3) + gl_FragCoord[1] * 0.4 * sin(seconds / 12.1)));
    float g = cos((gl_FragCoord[0] * 0.02 * sin(-seconds / 10.3) + gl_FragCoord[1] * 0.03 * cos(seconds / 1.8)));
    float b = sin(gl_FragCoord[0] * 0.002 * sin(-seconds * 2.3));
    fragColor = vec4(r, g, b, 1.0);
  } else {
    fragColor = vec4(vColor);
  }
}
