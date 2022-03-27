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

float curl(sampler2D tex, vec2 pos) {
    return texture(tex,pos+vec2(0.0,dx)).r-texture(tex,pos-vec2(0.0,dx)).r+texture(tex,pos+vec2(dx,0.0)).g-texture(tex,pos-vec2(dx,0.0)).g;
}

void main() {
    vec2 uv = gl_FragCoord.xy/res.xy;
    vec2 velocity = texture(texture2,uv).rg-(grad(texture4,uv));

    //if (gl_FragCoord.x<3.0 || res.x<gl_FragCoord.x-3.0) {velocity.x = 0.0;}
    //if (gl_FragCoord.y<3.0 || res.y<gl_FragCoord.y-3.0) {velocity.y = 0.0;}


    
    vec2 X = vec2(1.0/res.x,0.0);
    vec2 Y = vec2(0.0,1.0/res.y);


    float vorticity = 0.5;
    vec2 direction;
    direction.x = (abs(curl(texture2,uv-Y))-abs(curl(texture2,uv+Y)));
    direction.y = (abs(curl(texture2,uv+X))-abs(curl(texture2,uv-X)));

    direction = direction*vorticity/(length(direction)+0.00001);
    velocity.xy += direction*curl(texture2,uv)*dt;


    color = vec4(clamp(velocity,-1.0,1.0),0.0,0.0);
}
