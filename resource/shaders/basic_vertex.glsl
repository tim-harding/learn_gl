#version 330
precision mediump float;

attribute vec3 position;
attribute vec3 normal;
uniform mat4 view_proj;
out vec3 v_normal;
out vec3 v_world_pos;

void main() {
    v_world_pos = position + vec3(0.0, 0.0, -240.0);
    vec4 position = vec4(v_world_pos, 1.0);
    position = view_proj * position;
    v_normal = mat3(view_proj) * normal;
    gl_Position = position;
}