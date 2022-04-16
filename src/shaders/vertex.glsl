#version 420

in vec2 position;
in vec2 texture;
out vec2 texture_coords;

void main() {

    gl_Position = vec4(position,0.0,1.0);
    texture_coords = texture;
}