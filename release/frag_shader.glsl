#version 330
precision mediump float;

uniform sampler2D tex1;
uniform sampler2D tex2;

in VS_OUT {
    vec3 vert_pos;
    vec2 tex_coord;
} fs_in;

out vec4 out_color;

void main() {
    vec4 sample1 = texture(tex1, fs_in.tex_coord);
    vec4 sample2 = texture(tex2, fs_in.tex_coord);
    out_color = mix(sample1, sample2, 0.2);
}
