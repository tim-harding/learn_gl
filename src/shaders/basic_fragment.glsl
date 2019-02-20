#version 100
precision mediump float;
varying vec2 v_uv;
uniform float time;
void main() {
    float modulation = smoothstep(0.0, 1.0, abs(fract(time) - 0.5) * 2.0);
    gl_FragColor = vec4(v_uv, 1.0, 1.0);
}