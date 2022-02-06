#if GL_ES
precision highp float;
#endif

// ----------------------------------------------------------------------------
// Math Constants
// ----------------------------------------------------------------------------

#define PI 3.1415926538
#define TAU 6.283185307179586

// ----------------------------------------------------------------------------
// Graph Constants 
// ----------------------------------------------------------------------------

// Vertical Stretch
#define AMPLITUDE 1.

// Determines normal sampling detail 
// Lower means more accuracy but you need to be careful because it can cause funky
// floating point rounding errors
#define EPSILON 0.001

// ----------------------------------------------------------------------------
// Uniforms
// ----------------------------------------------------------------------------

// Perspective & Trasformation Matrix Uniforms
uniform mat4 mWorld;
uniform mat4 mView;
uniform mat4 mProj;

uniform float graphFrequency;

// Color for entire graph (constant)
uniform vec4 graphColor;

// Fun variables for user to play around with
uniform float TIME;

uniform float oldToNew;
// ----------------------------------------------------------------------------
// Vertex Attributes
// ----------------------------------------------------------------------------

attribute vec3 vertexPosition;

// ----------------------------------------------------------------------------
// Varying vars for fragment shader
// ----------------------------------------------------------------------------

// Vertex position in transformed space (without perspective applied)
varying vec3 v_Vertex;
// Symbolic Vertex position on graph
varying vec3 v_GraphVertex;
// Vertex Normal
varying vec3 v_Normal;

// ----------------------------------------------------------------------------
// Code
// ----------------------------------------------------------------------------

vec3 func(vec2 pos);
vec3 normal(vec2 pos);

void main() {
    vec2 graphVert = vertexPosition.xy * graphFrequency;
    vec4 pos4 = vec4(func(graphVert) / graphFrequency, 1.);

    // calculate fragment color
    v_Vertex = vec3(mView * mWorld * pos4); // transformed matrix
    v_Normal = normal(graphVert);
    v_GraphVertex = vec3(graphVert, pos4.z);

    gl_Position = mProj * mView * mWorld * pos4; // applies projection
}

vec3 normalTriPos(in vec2 origin, float ang) {
    float x = origin.x + cos(ang) * EPSILON;
    float y = origin.y + sin(ang) * EPSILON;
    return func(vec2(x, y));
}

vec3 normal(in vec2 pos) {
    // gets 3 points circularly around point to form a triangle with the
    // pos at the center
    vec3 triPoint1 = normalTriPos(pos, 0.);
    vec3 triPoint2 = normalTriPos(pos, TAU / 3.);
    vec3 triPoint3 = normalTriPos(pos, 2. * TAU / 3.);

    // gets 2 lines of the triangle
    vec3 a = triPoint2 - triPoint1;
    vec3 b = triPoint3 - triPoint1;

    // gets cross product of the 2 lines to get the normal of the triangle, 
    return -cross(a, b);
}

// Predefined functions
float hypot(float x, float y) {
    return length(vec2(x, y));
}

float lerp(float from, float to, float travel) {
    return from + (to - from) * travel;
}

float time() {
    return TIME;
}

// used to trick glsl optimizer

$EXTERN_FUNCTIONS$ 

vec3 func(vec2 pos) {
    float x = pos.x;
    float y = pos.y;
    float z;


    // template $$ replaced in rust
    if(oldToNew < 1.) {
        z = lerp($OLD_FUNCTION$, $CURRENT_FUNCTION$, oldToNew);
    } else {
        z = $CURRENT_FUNCTION$;
    }

    z *= - AMPLITUDE;
    return vec3(x, y, z);
}
