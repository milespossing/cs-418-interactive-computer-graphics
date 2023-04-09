#version 300 es

precision highp float;
const float PI = 3.141592653589;
const vec3 Y = vec3(0,1,0);

out vec4 fragColor;
in float height;
in vec3 fnormal;
in vec3 mnormal;
in vec3 vnormal;
in vec4 lightdir1;
in vec2 vTexCoord;
uniform vec3 lightcolor1;
uniform vec4 color1;
uniform vec4 color2;
uniform vec4 color3;
uniform vec4 cliffColor;
uniform sampler2D texture1;


void main() {
    vec3 y_arc = normalize(mnormal);
    float ctheta = dot(y_arc, Y); // cos(theta)
    vec3 n = normalize(mnormal);
    float lambert1 = max(dot(lightdir1.xyz, n), 0.0);
    bool isCliff = ctheta < 0.9;
    vec4 color = isCliff ? cliffColor : texture(texture1, vTexCoord); //pow(max(0.0, cos(h_norm)),2.0) * color1 + pow(max(0.0, sin(h_norm)),2.0) * color2 + pow(max(0.0,-cos(h_norm)),2.0) * color3;
    fragColor = vec4(color.rgb * (
                     lightcolor1 * lambert1
                     ), 1.0);//vec4(0.5,0.5,0.5,0.5);*/
}
