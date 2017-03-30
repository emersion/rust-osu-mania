#version 140

in vec2 position;
in uint at;
in uint duration;

uniform uint time;
uniform float milliseconds_per_beat;

const float fall_factor = 1.0;

void main() {
	gl_Position = vec4(position.x, position.y + (int(at) - int(time) + int(duration))/milliseconds_per_beat*fall_factor - 1.0, 0.0, 1.0);
}
