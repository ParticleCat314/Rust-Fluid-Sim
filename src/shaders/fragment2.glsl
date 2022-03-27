#version 420

in vec2 texture_coords;
out vec4 color;
uniform sampler2D texture1;

uniform vec2 mouse;
uniform vec2 mouse_delta;
uniform vec2 res;
uniform float dt;


#define dx 1.0/res.x

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


    vec2 previous_cell = uv-(dx*dt*texture(texture1,uv).rg);

    float Force = 0.0;
    vec2 mouse_force = normalize(mouse_delta);

    if (distance(gl_FragCoord.xy,mouse)<15.0 && 0.4<length(mouse_force)) {
        Force = -1.0;
    }

    //vec2 curl = vec2(texture(texture1,uv).r-texture(texture1,uv).g,-texture(texture1,uv).r+texture(texture1,uv).g);
    //curl = normalize(curl);
    
    vec3 new_velocity = BilinearInterpolation(previous_cell,texture1).rgb+vec3(Force*mouse_force,0.0);
    color = vec4(new_velocity,0.0);
}
