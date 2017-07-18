#version 140

in vec4 v_color;
in vec3 v_normal;
in vec3 v_fragpos;

out vec4 target_0;

uniform Locals {
  mat4 u_model;
  vec4 u_ambient;
  vec3 u_lightpos;
};

void main() {
  vec3 norm = normalize(v_normal);
  vec3 light_dir = normalize(u_lightpos - v_fragpos);
  vec4 light_color = vec4(1.0, 1.0, 1.0, 1.0);
  float diff = max(dot(norm, light_dir), 0.0);
  vec4 diffuse = diff * light_color;

  vec4 result = (u_ambient + diffuse) * v_color;
  target_0 = result;
}
