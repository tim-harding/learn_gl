#version 330
precision mediump float;
varying vec2 v_uv;
uniform float time;
uniform sampler2D tex1;
uniform sampler2D tex2;
void main() {
    float modulation = smoothstep(0.0, 1.0, abs(fract(time) - 0.5) * 2.0);
    vec4 first = texture(tex1, v_uv);
    vec4 second = texture(tex2, v_uv);
    gl_FragColor = mix(first, second, modulation);
}