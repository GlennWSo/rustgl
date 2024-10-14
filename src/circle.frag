#version 140
uniform vec2 u_resolution;

out vec4 fragColor;

void main() {
    vec2 st = fract(gl_FragCoord.xy/u_resolution*1.0);
    float maxd = max(u_resolution.x, u_resolution.y);
    vec2 pos = vec2(0.5) - st;
    pos.x *= u_resolution.x / u_resolution.y;
    float r = length(pos);
    float inside = step(0.5, r);
    vec3 spectra = vec3(st, 1.0);
    vec3 color = mix(vec3(0.0), spectra, inside);


    fragColor = vec4(color, 1.0);
}

