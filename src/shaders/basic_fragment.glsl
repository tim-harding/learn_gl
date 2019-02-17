#version 100
precision mediump float;
varying vec3 v_color;
uniform float time;
void main() {
    float modulation = smoothstep(0.0, 1.0, abs(fract(time) - 0.5) * 2.0);
    gl_FragColor = vec4(v_color * modulation, 1.0);
}