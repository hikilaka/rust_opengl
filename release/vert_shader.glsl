#version 320
precision mediump float;

in vec3 vert_pos;
in vec3 vert_col;
in vec2 tex_coord;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out VS_OUT {
  vec3 vert_col;
  vec2 tex_coord;
}
vs_out;

void main() {
  vs_out.vert_col = vert_col;
  vs_out.tex_coord = tex_coord;
  gl_Position = projection * view * model * vec4(vert_pos, 1.0);
}
