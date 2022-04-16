#version 420

in vec2 texture_coords;
out vec4 color;
uniform sampler2D render_texture1;

#define itter 100

void main() {
    //vec4 c = texture(render_texture1,texture_coords);
    vec2 uv = gl_FragCoord.xy/600.0;


    color = vec4(0.1,0.0,0.0,0.0);

}
