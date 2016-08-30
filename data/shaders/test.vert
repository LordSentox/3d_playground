#version 330 core
layout(location = 0) in vec3 position;
layout(location = 1) in vec3 inColour;

out vec3 adjColour;
uniform mat4 proj_mat;
uniform mat4 view_mat;

void main() {
	// The provided vector is always interpreted as a position vector.
	gl_Position = proj_mat * view_mat * vec4(position, 1.0);
	adjColour = inColour;
}
