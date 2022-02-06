#if GL_ES
precision mediump float;
#endif

#define COLOR vec4(0., 0., 0., .5)

varying vec3 color;
varying vec2 UV;

uniform float frequency;

void main() {
    float fuzz = .5;
    float minorLineFreq = 1. / (frequency + 1.);

    float xd = mod(UV.x, minorLineFreq) * 88.1;
    float yd = mod(UV.y, minorLineFreq) * 88.1;

    if(xd < fuzz || yd < fuzz) {
        gl_FragColor = COLOR;
    } else {
        gl_FragColor = vec4(0.0);
    }
}
