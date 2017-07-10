#version 150 core

in vec4 a_pos;
in vec4 a_color;
out vec4 v_color;

void main() {
    v_color = a_color;
    gl_Position = a_pos;
}
