#version 140

in vec2 position;
in uint pressed;

in uint key;

out vec4 v_color;

const vec4 color_odd = vec4(1, 1, 1, 1);
const vec4 color_even = vec4(1, 0, 0, 1);
const vec4 color_center = vec4(1, 1, 0, 1);

float key_x(uint key) {
	// TODO: keys_count
	return 0.1 * float(key);
}

vec4 key_color(uint key) {
	if (pressed == 0u) {
		return vec4(0);
	}

	if (key == 0u || key == 3u) {
		return color_odd;
	} else {
		return color_even;
	}
}

void main() {
	if (position.y == 0) {
		v_color = key_color(key);
	}

	float x = position.x + key_x(key);
	float y = position.y - 1.0;

	gl_Position = vec4(x, y, 0.0, 1.0);
}
