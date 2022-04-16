#version 420

in vec2 texture_coords;
out vec4 color;


uniform sampler2D texture2;
uniform sampler2D texture4;
uniform vec2 res;
uniform float dt;


#define dx 1.0/res.x
#define p(pos) texture(texture4,pos).r


vec2 grad(sampler2D tex, vec2 pos) {
    float gradx = p(pos+vec2(dx,0.0))-p(pos-vec2(dx,0.0));
    float grady = p(pos+vec2(0.0,dx))-p(pos-vec2(0.0,dx));
    return vec2(gradx,grady);
}



void main() {
    vec2 uv = gl_FragCoord.xy/res.xy;
    vec2 velocity = texture(texture2,uv).rg-3.0*(grad(texture4,uv));


    color = vec4(clamp(velocity,-1.5,1.5),0.0,texture(texture2,uv).a);
}
