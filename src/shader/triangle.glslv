#version 140

in vec4 a_pos;
in vec4 a_color;
in vec3 a_normal;

out vec4 v_color;
out vec3 v_fragpos;
out vec3 v_normal;

uniform mat4 u_model;
uniform vec4 u_ambient;
uniform vec4 u_lightcolor;

uniform vec3 u_viewpos;
uniform vec3 u_lightpos;

void main() {
    v_color = a_color;
    v_normal = a_normal;
    v_fragpos = vec3(u_model * a_pos);

    gl_Position = u_model * a_pos;
}
