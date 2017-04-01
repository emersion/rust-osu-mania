#version 140

flat in uint v_key;

uniform uint keys_count;

out vec4 color;

const vec4 color_odd = vec4(1, 1, 1, 1);
const vec4 color_even = vec4(1, 0, 0, 1);
const vec4 color_center = vec4(1, 1, 0, 1);

void main() {
	// TODO: keys_count
	if (v_key == 0u || v_key == 3u) {
		color = color_odd;
	} else {
		color = color_even;
	}
}
