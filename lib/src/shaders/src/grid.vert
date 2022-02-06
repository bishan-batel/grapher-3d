#if GL_ES
precision mediump float;
#endif

attribute vec3 vertexPosition;
attribute vec3 vertexColor;

// Perspective & Trasformation Matrix Uniforms
uniform mat4 mModel;
uniform mat4 mView;
uniform mat4 mProj;

varying vec3 color;
varying vec2 UV;

void main() {
    color = vertexColor;

    vec4 pos4 = vec4(vertexPosition, 1.);
    gl_Position = mProj * mView * mModel * pos4; // applies projection
    UV = vertexPosition.xy;
    //gl_Position = vec4(vertexPosition.x, vertexPosition.y, 0., 0.);
}
