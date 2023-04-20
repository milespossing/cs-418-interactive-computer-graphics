#version 300 es

precision highp float;

out vec4 fragColor;
in vec3 mnormal;
in vec4 v_lightdir1;
uniform vec3 lightcolor1;
uniform vec4 color;


void main() {
    vec3 n = normalize(mnormal);
    float lambert1 = max(dot(v_lightdir1.xyz, n), 0.0);
    vec4 lit = vec4(color.rgb * (
                   lightcolor1 * lambert1
                   ), 1.0);//vec4(0.5,0.5,0.5,0.5);*/
    fragColor = lit;
}
