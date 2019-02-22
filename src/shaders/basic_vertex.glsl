#version 100
precision mediump float;

attribute vec2 position;
attribute vec2 uv;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
varying vec2 v_uv;

void main() {
    vec4 position = vec4(position, 0.0, 1.0);
    position = projection * view * model * position;
    gl_Position = position;
    v_uv = uv;
}