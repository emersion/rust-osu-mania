#version 140

in vec2 position;

in uint at;
in uint duration;
in uint key;
in float milliseconds_per_beat;

out vec4 v_color;

uniform uint current_time;
uniform float current_milliseconds_per_beat;
uniform uint keys_count;

const float fall_factor = 1.0;
const vec4 color_odd = vec4(1, 1, 1, 1);
const vec4 color_even = vec4(1, 0, 0, 1);
const vec4 color_center = vec4(1, 1, 0, 1);

float key_x(uint key) {
	// TODO: keys_count
	return 0.1 * float(key);
}

vec4 key_color(uint key) {
	if (key == 0u || key == 3u) {
		return color_odd;
	} else {
		return color_even;
	}
}

void main() {
	v_color = key_color(key);

	float x = position.x + key_x(key);

	float y = position.y + (int(at) - int(current_time)) * fall_factor/current_milliseconds_per_beat - 1.0;
	if (position.y > 0.0 && duration > 0u) {
		y += int(duration) * fall_factor/milliseconds_per_beat;
	}

	gl_Position = vec4(x, y, 0.0, 1.0);
}
