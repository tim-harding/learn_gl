#version 330
precision mediump float;

varying vec3 v_normal;
varying vec3 v_world_pos;
out vec4 fragColor;

void main() {
    vec3 view_dir = normalize(v_world_pos);
    float fresnel = dot(view_dir, v_normal);
    fragColor = vec4(fresnel, fresnel, fresnel, 1.0);
}