#version 420

in vec2 texture_coords;
out vec4 color;
uniform sampler2D texture1;
uniform sampler2D texture4;
uniform bool first;
uniform vec2 res;

uniform float dt;

#define p(pos) texture(texture4,pos).r
#define dx 1.0/res.x
//#define dt 0.2



float jacobi(vec2 pos) {

    float thing = 0.0;
    if (!first) {
        thing = p(pos+vec2(2.0*dx,0.0))+p(pos-vec2(2.0*dx,0.0))+p(pos+vec2(0.0,2.0*dx))+p(pos-vec2(0.0,2.0*dx));
    }

    return -texture(texture1,pos).r+thing;

}


void main() {
    
    color = vec4(0.25*jacobi(gl_FragCoord.xy/res.xy),0.0,0.0,1.0);

}
