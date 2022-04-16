#version 420

in vec2 texture_coords;
out vec4 color;
uniform sampler2D render_texture1;

#define itter 300
#define PI 3.14159265
void main() {
    //vec4 c = texture(render_texture1,texture_coords);
    vec2 uv = 4.0*gl_FragCoord.xy/500.0;
    vec2 Z1 = vec2(0.0,0.0);
    vec2 C = uv-vec2(1.0,1.0);
    vec2 temp;
    vec2 Z = Z1;
    int m = 0;


    for (int n = 0; n<itter; n++) {
        temp = Z;

        Z.x = (Z.x*Z.x - Z.y*Z.y) + C.x;
        Z.y = 2.0*temp.x*temp.y + C.y;

        m += 1;

        if (8.0<abs(Z.x*Z.x+Z.y*Z.y)) {
            m = n;
            break;
        }

    }



    float cc = m*1.0/float(itter);

    float c = cc;//0.01*gl_FragCoord.x;
    float dist = length(C);
    vec3 col = 0.1/dist*vec3(sin(c),sin(c+PI/4.0),sin(c+PI/2.0));

    if (length(gl_FragCoord.xy-vec2(200.0))<70.0) {
        color = vec4((col*col),1.0);//
    }
    else {
        color = vec4(0.0,0.0,0.0,1.0);
    }

    color = vec4(col,1.0);
}
