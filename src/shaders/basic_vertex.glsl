#version 100
precision mediump float;

attribute vec2 position;
attribute vec2 uv;
uniform mat3 transform;
varying vec2 v_uv;

void main() {
    gl_Position = vec4(transform * vec3(position, 0.0), 1.0);
    v_uv = uv;
}