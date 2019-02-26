#version 330
precision mediump float;

attribute vec3 position;
uniform mat4 view_proj;

void main() {
    vec4 position = vec4(position.xzy + vec3(0.0, 0.0, -240.0), 1.0);
    position = view_proj * position;
    gl_Position = position;
}