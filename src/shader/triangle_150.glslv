#version 150 core

in vec3 a_pos;
in vec4 a_color;
out vec4 v_color;

void main() {
    v_color = a_color;
    gl_Position = vec4(a_pos, 1.0);
}
