#version 140

in vec4 a_pos;
in vec4 a_color;
in vec3 a_normal;
in vec2 a_uv;

out vec4 v_color;
out vec3 v_fragpos;
out vec3 v_normal;
out vec2 v_uv;
flat out int v_face;

uniform mat4 u_model;

void main() {
    v_color = a_color;
    v_normal = a_normal;
    v_fragpos = vec3(u_model * a_pos);
    v_uv = a_uv;

    v_face = gl_VertexID / 6;
    gl_Position = u_model * a_pos;
}
