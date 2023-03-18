#version 300 es

precision highp float;
uniform vec4 color;
uniform vec3 halfway;
uniform vec3 halfway2;
uniform vec3 lightdir1;
uniform vec3 lightcolor1;
uniform vec3 lightdir2;
uniform vec3 lightcolor2;
out vec4 fragColor;
in vec3 fnormal;
void main() {
    vec3 n = normalize(fnormal);
    float blinn1 = pow(max(dot(halfway, n), 0.0), 300.0);
    float blinn2 = pow(max(dot(halfway2, n), 0.0), 300.0);
    float lambert1 = max(dot(lightdir1, n), 0.0);
    float lambert2 = max(dot(lightdir2, n), 0.0);
    fragColor = vec4(color.rgb *
                     (
                     lightcolor1 * lambert1
                   + lightcolor2 * lambert2
                   + lightcolor1 * blinn1
                   + lightcolor2 * blinn2), color.a);
  /* fragColor = vec4(fnormal, 1) * color; */
}
