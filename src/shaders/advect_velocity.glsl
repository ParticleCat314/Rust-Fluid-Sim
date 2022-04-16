#version 420

in vec2 texture_coords;
out vec4 color;
uniform sampler2D texture1;

uniform vec2 mouse;
uniform vec2 mouse_delta;
uniform vec2 res;
uniform float dt;
uniform float rate;


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


float curl(sampler2D tex, vec2 pos) {
    return texture(tex,pos+vec2(0.0,dx)).r-texture(tex,pos-vec2(0.0,dx)).r+texture(tex,pos+vec2(dx,0.0)).g-texture(tex,pos-vec2(dx,0.0)).g;
}

void main() {
    vec2 uv = gl_FragCoord.xy/res.xy;


    vec2 previous_cell = uv-(dx*dt*texture(texture1,uv).rg);

    float Force = 0.0;
    vec2 mouse_force = normalize(mouse_delta);

    if (distance(gl_FragCoord.xy,mouse)<15.0 && 0.5<length(mouse_force)) {
        Force = -10.0;
    }

    
    vec2 X = vec2(1.0/res.x,0.0);
    vec2 Y = vec2(0.0,1.0/res.y);


    float vorticity = 0.0;
    vec2 direction;
    direction.x = (abs(curl(texture1,uv-Y))-abs(curl(texture1,uv+Y)));
    direction.y = (abs(curl(texture1,uv+X))-abs(curl(texture1,uv-X)));

    direction = direction*vorticity/(length(direction)+0.01);
    //velocity.xy += direction*curl(texture1,uv)*dt;

    //vec2 curl = vec2(texture(texture1,uv).r-texture(texture1,uv).g,-texture(texture1,uv).r+texture(texture1,uv).g);
    //curl = normalize(curl);
    
    vec3 new_velocity = BilinearInterpolation(previous_cell,texture1).rgb+vec3(Force*mouse_force,0.0)+vec3(direction*curl(texture1,uv)*dt,0.0);
    new_velocity.y -= 1.5*texture(texture1,uv).a;

    float temp = texture(texture1,uv).a-(texture(texture1,uv).a+0.25*(texture(texture1,uv+X).a+texture(texture1,uv-X).a+texture(texture1,uv+Y).a+texture(texture1,uv-Y).a));

    if (res.y-5.0<gl_FragCoord.y) {temp = 1.0;}
    if (gl_FragCoord.y<5.0) {temp = -1.0;}

    color = vec4(rate*new_velocity,clamp(0.01*Force+temp,-1.0,1.0));
}
