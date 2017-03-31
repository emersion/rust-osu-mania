#version 140

in vec2 position;
in uint at;
in uint key;
in float milliseconds_per_beat;

uniform uint time;
uniform uint keys_count;

const float fall_factor = 1.0;

flat out uint key_out;

float key_x(uint key) {
	// TODO: keys_count
	return 0.1 * float(key);
}

void main() {
	key_out = key;
	gl_Position = vec4(position.x + key_x(key), position.y + (int(at) - int(time))/milliseconds_per_beat*fall_factor - 1.0, 0.0, 1.0);
}
