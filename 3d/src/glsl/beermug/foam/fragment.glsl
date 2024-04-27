varying vec2 vUv;
varying float vElevation;
uniform vec3 uColor;

float random(vec2 st)
{
    return fract(sin(dot(st.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

void main() {
    vec3 color = mix(uColor, vec3(0.0, 0.0, 0.0), vElevation * 2.0 + 0.05);
    gl_FragColor = vec4(color, 1.0);
}
