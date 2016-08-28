#version 330 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 inColour;

out vec3 adjColour;
uniform mat4 matrix;

void main() {
	// The provided vector is always interpreted as a position vector.
	gl_Position = matrix * vec4(position.x, position.y, position.z, 1.0);
	adjColour = inColour;
}
