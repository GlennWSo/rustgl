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


// https://iquilezles.org/articles/rmshadows
float calcSoftshadow( in vec3 ro, in vec3 rd, float tmin, float tmax, const float k ){
	float res = 1.0;
    float t = tmin;
    for( int i=0; i<64; i++ )
    {
		float h = sdf( ro + rd*t );
        res = min( res, k*h/t );
        t += clamp( h, 0.01, 0.10 );
        if( res<0.002 || t>tmax ) break;
    }
    return clamp( res, 0.0, 1.0 );
}

// https://iquilezles.org/articles/normalsSDF
vec3 calcNormal( in vec3 pos )
{
    vec2 e = vec2(1.0,-1.0)*0.5773;
    const float eps = 0.0005;
    return normalize( e.xyy*sdf( pos + e.xyy*eps ) + 
					  e.yyx*sdf( pos + e.yyx*eps ) + 
					  e.yxy*sdf( pos + e.yxy*eps ) + 
					  e.xxx*sdf( pos + e.xxx*eps ) );
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
    if( t<tmax ){
        vec3 pos = rayOrigin + t*ray_dir;
        vec3 nor = calcNormal(pos);
        vec3 lig = vec3(0.57703);
        float dif = clamp( dot(nor,lig), 0.0, 1.0 );
        if( dif>0.001 ) dif *= calcSoftshadow( pos+nor*0.001, lig, 0.001, 1.0, 32.0 );
        float amb = 0.5 + 0.5*dot(nor,vec3(0.0,1.0,0.0));
        color = vec3(0.2,0.3,0.4)*amb + vec3(0.8,0.7,0.5)*dif;
    }    

    fragColor = vec4(color, 1.0);
}

