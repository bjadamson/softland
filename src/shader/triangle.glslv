#version 140

in vec4 a_pos;
in vec4 a_color;
out vec4 v_color;

uniform Locals {
	mat4 u_model;
};

void main() {
    v_color = a_color;
    gl_Position = u_model * a_pos;
}
