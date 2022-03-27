#version 420

in vec2 texture_coords;
out vec4 color;
uniform sampler2D render_texture1;
uniform vec2 draw;
uniform vec2 delta;

#define dt 0.15f
#define dx 0.5f
#define t(a) texture(render_texture1,texture_coords+a)
#define K 0.2f

void main() {
    float s=K/dt;

    vec2 xy = gl_FragCoord.xy/600.0;
    float Step = 1.0/600.0;

    vec4 original = t(vec2(0.0,0.0));
    vec3 N = t(vec2(0.0,Step)).xyz;
    vec3 E = t(vec2(Step,0.0)).xyz;
    vec3 S = t(vec2(0.0,-Step)).xyz;
    vec3 W = t(vec2(-Step,0.0)).xyz;
    
    vec3 UdX = dx*(E-W);
    vec3 UdY = dx*(N-S);

    float divergence = UdX.x+UdY.y;
    vec2 DdX = vec2(UdX.z,UdY.z);

    original.z -= dt*dot(vec3(DdX.xy,divergence),original.xyz);
    original.z = clamp(original.z,0.5f,3.0f);

    vec2 PdX = s*DdX;
    vec3 Laplacian = (N+E+S+W)-4.0*original.xyz;
    vec2 visForce = 1.0*Laplacian.xy;
    

    vec2 Was = xy - dt*original.xy*Step;
    original.xy = texture(render_texture1,Was).rg;
    original[3] = texture(render_texture1,Was).a;

    vec2 force = vec2(0.0);

    if (distance(gl_FragCoord.xy,draw)<20.0){
        float g = distance(gl_FragCoord.xy,draw);
        force = 0.1/g*delta;
        //original.xy += 0.1;
    }
    original.xy += dt*(visForce - PdX + force);
    original.xy = clamp(original.xy,-1.0,1.0);


    if (gl_FragCoord.x==599.0 || gl_FragCoord.y==599.0 || gl_FragCoord.x==1.0 || gl_FragCoord.y==1.0) {
      original.xy *= 0.0;
    }
    
    color = vec4(original);
    

}
