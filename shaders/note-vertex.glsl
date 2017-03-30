#version 140

in vec2 position;
in uint at;
in uint key;
in float milliseconds_per_beat;

uniform uint time;

const float fall_factor = 1.0;

flat out uint key_out;

void main() {
	key_out = key;
	gl_Position = vec4(position.x, position.y + (int(at) - int(time))/milliseconds_per_beat*fall_factor - 1.0, 0.0, 1.0);
}
