#version 140
uniform vec2 u_resolution;

out vec4 fragColor;


// r = sphere's radius
// h = cutting's plane's position
// t = thickness
float sdCutHollowSphere( vec3 p, float r, float h, float t ) {
    vec2 q = vec2( length(p.xz), p.y );
    
    float w = sqrt(r*r-h*h);
    
    return ((h*q.x<w*q.y) ? length(q-vec2(w,h)) : 
                            abs(length(q)-r) ) - t;
}

float sdf( in vec3 pos ) {
    pos.xy = (mat2(3,4,-4,3)/5.0)*pos.xy;
    float r = 0.5;
    float h = 0.2;
    float t = 0.01;
    return sdCutHollowSphere(pos, r, h, t );
}


void main() {

    // camera movement	
	float an = sin(0.0);
	vec3 rayOrigin = vec3( 1.0*cos(an), 0.0, 1.0*sin(an) );
    vec3 ta = vec3( 0.0, 0.0, 0.0 );
    // camera matrix
    vec3 ww = normalize( ta - rayOrigin);
    vec3 uu = normalize( cross(ww,vec3(0.0,1.0,0.0) ) );
    vec3 vv = normalize( cross(uu, ww));

    // pixel coordinate
    vec2 p = (2.0*gl_FragCoord.xy-u_resolution.xy)/u_resolution.y;

    // raymarch
    vec3 ray_dir = normalize( p.x*uu + p.y*vv + 1.5*ww );
    const float tmax = 5.0;
    float t = 0.0;
    for (int i =0; i<256; i++) {
        vec3 pos = rayOrigin + t*ray_dir;
        float h = sdf(pos);
        if (h < 0.0001 || t>tmax) break;
        t += h;
    }

    // shading/lighting	
    vec3 color = vec3(0.0);
    if( t<tmax ) {
        color = vec3(0.0, 0.0, 1.0) ;
    }
    

    fragColor = vec4(color, 1.0);
}

