#version 140

in vec4 a_pos;
in vec4 a_color;
in vec3 a_normal;

out vec4 v_color;
out vec3 v_fragpos;
out vec3 v_normal;

uniform Locals {
  mat4 u_model;
  vec4 u_ambient;
  vec4 u_lightcolor;
  vec3 u_lightpos;
};

void main() {
    v_color = u_ambient * a_color;
    v_normal = a_normal;
    v_fragpos = vec3(u_model * a_pos);

    gl_Position = u_model * a_pos;
}
