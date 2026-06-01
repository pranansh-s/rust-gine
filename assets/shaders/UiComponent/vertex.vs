#version 330 core

layout (location = 0) in vec2 position;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec2 fragPosition;

void main() {
    gl_Position = projection * view * model * vec4(position, 0.0, 1.0);
    fragPosition = position;
}