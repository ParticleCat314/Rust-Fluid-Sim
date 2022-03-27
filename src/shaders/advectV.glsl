#version 420

in vec2 texture_coords;
out vec4 color;
uniform sampler2D texture1;
uniform sampler2D texture2;
uniform sampler2D texture3;
uniform sampler2D texture4;
uniform vec2 mouse;
uniform vec2 mouse_delta;
uniform vec2 res;

uniform float dt;

//#define dt 0.2
#define dx 1.0/res.x

#define t1(a) texture(texture3,a).b
//#define res vec2(600.0,600.0)
vec4 BilinearInterpolation(vec2 pos, sampler2D q){
    
    float x1 = floor(pos.x*res.x)/res.x; 
    float x2 = ceil(pos.x*res.x)/res.x;
    float y1 = floor(pos.y*res.y)/res.y;
    float y2 = ceil(pos.y*res.y)/res.y;
    float w11 = ((x2-pos.x)*(y2-pos.y))/((x2-x1)*(y2-y1));
    float w12 = ((x2-pos.x)*(pos.y-y1))/((x2-x1)*(y2-y1));
    float w21 = ((pos.x-x1)*(y2-pos.y))/((x2-x1)*(y2-y1));
    float w22 = ((pos.x-x1)*(pos.y-y1))/((x2-x1)*(y2-y1));
    vec4 final = w11*texture(q,vec2(x1,y1))+w12*texture(q,vec2(x1,y2))+w21*texture(q,vec2(x2,y1))+w22*texture(q,vec2(x2,y2));
    return final;
}


void main() {
    vec2 uv = gl_FragCoord.xy/res.xy;
    vec2 North = vec2(1.0/res.x,0.0);
    vec2 South = vec2(0.0,1.0/res.y);

    vec2 prev = uv-(dx*dt*texture(texture1,uv).rg);

    //color = vec4(t1(uv).rgb-0.15*col.rgb,1.0);


    vec3 vel = BilinearInterpolation(prev,texture3).rgb;

    //if (599.0<gl_FragCoord.x || gl_FragCoord.x<1.0) {
    //    vel = vec3(0.0,0.0,0.0);
    //}
    //if (599.0<gl_FragCoord.y || gl_FragCoord.y<1.0) {
    //    vel = vec3(0.0,0.0,0.0);
    //}

    float F = 0.0;
    vec2 Mdx = normalize(mouse_delta);
    float dis = distance(gl_FragCoord.xy,mouse);

    //if (dis<15.0 && 1.0<length(mouse_delta)) {
    //    F = 0.5;
    //}


    //vec2 source = 0.5*res-vec2(0.0,0.5*res.y);
    //if (distance(gl_FragCoord.xy,source)<5.0) {
    //    F += 0.2;
    //}
    //
    //dis = dis;
    //float t = gl_FragCoord.x/100.0;
    //vec3 col = vec3(sin(t),sin(t+3.14/4.0),sin(t+3.14/2.0));

    color = vec4(vel.rgb,1.0);

}
