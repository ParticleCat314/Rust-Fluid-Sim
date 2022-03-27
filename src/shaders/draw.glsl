#version 420

in vec2 texture_coords;
out vec4 color;
uniform sampler2D texture1;
uniform sampler2D texture2;
uniform sampler2D texture3;
uniform sampler2D texture4;
uniform sampler2D temp;
uniform vec2 mouse;

void main() {
    vec4 c = texture(temp,texture_coords);
    float F = 0.0;

    //vec3 col = vec3(sin(-c.b),sin(c.b-0.5),sin(c.b-1.5));
    color = vec4(cos(c.rgb),1.0);

}
