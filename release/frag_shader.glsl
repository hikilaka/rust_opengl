#version 320
precision mediump float;

uniform sampler2D tex1;
uniform sampler2D tex2;

in VS_OUT {
  vec3 vert_col;
  vec2 tex_coord;
}
fs_in;

out vec4 out_color;

void main() {
  vec2 tex_coords = vec2(fs_in.tex_coord.x, -fs_in.tex_coord.y);
  vec4 sample1 = texture(tex1, tex_coords);
  vec4 sample2 = texture(tex2, tex_coords);
  out_color = mix(sample1, sample2, 0.2);
}
