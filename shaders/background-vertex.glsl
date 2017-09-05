#version 140

in vec2 position;

uniform sampler2D background_texture;

out vec2 v_tex_coords;

void main() {
	v_tex_coords = (position + vec2(1.0))/2;
	gl_Position = vec4(position, 0.0, 1.0);
}
