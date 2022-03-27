#version 420

in vec2 texture_coords;
out vec4 color;

uniform sampler2D texture2;
uniform vec2 res;
uniform float dt;

#define dx 1.0/res.x

vec3 BilinearInterpolation(vec2 pos, sampler2D q){
    
    float x1 = floor(pos.x*res.x)/res.x; 
    float x2 = ceil(pos.x*res.x)/res.x;
    float y1 = floor(pos.y*res.y)/res.y;
    float y2 = ceil(pos.y*res.y)/res.y;
    float w11 = ((x2-pos.x)*(y2-pos.y))/((x2-x1)*(y2-y1));
    float w12 = ((x2-pos.x)*(pos.y-y1))/((x2-x1)*(y2-y1));
    float w21 = ((pos.x-x1)*(y2-pos.y))/((x2-x1)*(y2-y1));
    float w22 = ((pos.x-x1)*(pos.y-y1))/((x2-x1)*(y2-y1));
    vec3 final = w11*texture(q,vec2(x1,y1)).rgb+w12*texture(q,vec2(x1,y2)).rgb+w21*texture(q,vec2(x2,y1)).rgb+w22*texture(q,vec2(x2,y2)).rgb;
    return final;
}

float divergence(sampler2D tex, vec2 pos) {
    float divx = (texture(tex,pos+vec2(dx,0.0)).r-texture(tex,pos-vec2(dx,0.0)).r);
    float divy = (texture(tex,pos+vec2(0.0,dx)).g-texture(tex,pos-vec2(0.0,dx)).g);
    return divx+divy;
}

void main() {
    vec2 uv = gl_FragCoord.xy/res.x;
    color = vec4(0.5*divergence(texture2,uv),0.0,0.0,0.0);
}

