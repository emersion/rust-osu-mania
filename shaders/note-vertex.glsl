#version 140

in vec2 position;

in uint at;
in uint duration;
in uint key;
in float milliseconds_per_beat;

uniform uint time;
uniform uint keys_count;

const float fall_factor = 1.0;

flat out uint v_key;

float key_x(uint key) {
	// TODO: keys_count
	return 0.1 * float(key);
}

void main() {
	v_key = key;

	float x = position.x + key_x(key);

	float a = milliseconds_per_beat*fall_factor;
	float y = position.y + (int(at) - int(time))/a - 1.0;
	if (position.y > 0.0 && duration > 0u) {
		y += int(duration)/a;
	}

	gl_Position = vec4(x, y, 0.0, 1.0);
}
