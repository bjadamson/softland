#version 140

in vec4 v_color;
in vec3 v_normal;
in vec3 v_fragpos;

out vec4 target_0;

uniform mat4 u_model;
uniform vec4 u_ambient;
uniform vec4 u_lightcolor;

uniform vec3 u_viewpos;
uniform vec3 u_lightpos;

void main() {
  vec3 norm = normalize(v_normal);
  vec3 light_dir = normalize(u_lightpos - v_fragpos);
  float diff = max(dot(norm, light_dir), 0.0);
  vec4 diffuse = diff * u_lightcolor;

  vec3 view_dir = normalize(u_viewpos - v_fragpos);
  vec3 reflect_dir = reflect(-light_dir, norm);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);

  float specular_strength = 1.0;
  vec4 specular = specular_strength * spec * u_lightcolor;

  target_0 = (u_ambient + diffuse + specular) * v_color;
}
