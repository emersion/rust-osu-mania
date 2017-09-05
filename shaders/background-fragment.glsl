#version 140

in vec2 v_tex_coords;

uniform sampler2D background_texture;

out vec4 color;

void main() {
	color = texture(background_texture, v_tex_coords);
}
