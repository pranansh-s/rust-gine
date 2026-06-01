#version 330 core

in vec2 fragPosition;
out vec4 Color;

uniform vec2 scale;
uniform float roundness;
uniform vec4 color;

// float sdRoundBox(vec2 p, vec2 b, float r) {
//     vec2 q = abs(p) - b + r;
//     return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r;
// }

void main() {
    // float edgeSoftness = 0.01;

	// float d = sdRoundBox(fragPosition, vec2(50.0, 50.0), 0.8);
    // float cornerAlpha = 1.0 - smoothstep(0.0, edgeSoftness, d);
    // Color = vec4(color.rgb, cornerAlpha);
    // vec3 col = (d > 0.0) ? vec3(0.0, 0.0, 0.0) : vec3(1.0, 1.0, 1.0);
	// col = mix(col, vec3(1.0), 1.0 - smoothstep(0.0, edgeSoftness, abs(d)));
	Color = color;
}