#version 300 es

precision highp float;
const float PI = 3.141592653589;
const vec3 Y = vec3(0,1,0);

uniform float hMax;
uniform float hMin;
uniform vec3 halfway;
uniform vec3 lightcolor1;
uniform float blinnAmount;
uniform vec4 cliffColor;
uniform vec4 color1;
uniform vec4 color2;
uniform vec4 color3;
uniform float maxHeight;
uniform float minHeight;
out vec4 fragColor;
in float height;
in vec3 fnormal;
in vec3 mnormal;
in vec4 lightdir1;

void main() {
    vec3 y_arc = normalize(mnormal);
    float ctheta = dot(y_arc, Y); // cos(theta)
    bool isCliff = ctheta < 0.8;
    float h_norm = (height - minHeight) * PI / (maxHeight - minHeight);
    vec4 color = isCliff ? cliffColor : pow(max(0.0, cos(h_norm)),2.0) * color1 + pow(max(0.0, sin(h_norm)),2.0) * color2 + pow(max(0.0,-cos(h_norm)),2.0) * color3;
    vec3 n = normalize(fnormal);
    float lambert1 = max(dot(lightdir1.xyz, n), 0.0);
    float blinn1 = pow(max(dot(halfway, n), 0.0), 300.0);
    fragColor = vec4(color.rgb * (
                     lightcolor1 * lambert1 +
                     lightcolor1 * blinn1 * blinnAmount
                     ), color.a);
}
