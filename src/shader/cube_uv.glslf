#version 140

in vec4 v_color;
in vec3 v_normal;
in vec3 v_fragpos;
in vec2 v_uv;
flat in int v_face;

out vec4 target_0;

uniform mat4 u_model;
uniform vec4 u_ambient;
uniform vec4 u_lightcolor;

uniform sampler2D uv_front;
uniform sampler2D uv_back;
uniform sampler2D uv_top;
uniform sampler2D uv_bottom;
uniform sampler2D uv_left;
uniform sampler2D uv_right;

struct SpecularLight {
  vec3 viewpos;
  vec3 lightpos;
};

uniform SpecularLight u_specular;

void main() {
  vec3 norm = normalize(v_normal);
  vec3 light_dir = normalize(u_specular.lightpos - v_fragpos);
  float diff = max(dot(norm, light_dir), 0.0);
  vec4 diffuse = diff * u_lightcolor;

  vec3 view_dir = normalize(u_specular.viewpos - v_fragpos);
  vec3 reflect_dir = reflect(-light_dir, norm);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);

  float specular_strength = 1.0;
  vec4 specular = specular_strength * spec * u_lightcolor;

  switch(v_face)
  {
    case 0: target_0 = texture(uv_front, v_uv); break;
    case 1: target_0 = texture(uv_back, v_uv); break;
    case 2: target_0 = texture(uv_top, v_uv); break;
    case 3: target_0 = texture(uv_bottom, v_uv); break;
    case 4: target_0 = texture(uv_left, v_uv); break;
    case 5: target_0 = texture(uv_right, v_uv); break;
  }
}
