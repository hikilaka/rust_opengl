#version 330
precision mediump float;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

in vec3 vert_pos;
in vec2 tex_coord;

out VS_OUT {
    vec3 vert_pos;
    vec2 tex_coord;
} vs_out;

void main() {
    vs_out.vert_pos = vert_pos;
    vs_out.tex_coord = tex_coord;
    gl_Position = projection * view * model * vec4(vert_pos, 1.0);
}
