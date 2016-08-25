#version 330 core
in vec3 adjColour;
out vec4 color;

void main() {
	color = vec4(adjColour, 0.75);
}
