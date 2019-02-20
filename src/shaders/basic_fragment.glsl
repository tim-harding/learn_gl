#version 330
precision mediump float;
varying vec2 v_uv;
uniform float time;
uniform sampler2D tex;
void main() {
    float modulation = smoothstep(0.0, 1.0, abs(fract(time) - 0.5) * 2.0);
    gl_FragColor = texture(tex, v_uv);
}