/*
 * Fragment shader for every equation rendered
 */

#if GL_ES
precision highp float;
#endif

// Uncomment to render surface normals instead of graph color

uniform vec4 graphColor;
uniform vec3 globalLightPosition;

// Transparency Constants
#define CLAMP_Z 10.
#define CLAMP_SCATTER .05

// Lighting Constants
#define MIN_LIGHT 0.1
#define MAX_LIGHT 1.05

// Vertex position in transformed space (without perspective applied)
varying vec3 v_Vertex;
// Symbolic Vertex position on graph
varying vec3 v_GraphVertex;
// Vertex Normal
varying vec3 v_Normal;

// mathematical sigmoid function
float sigmoid(float x) {
  float denom = 1. + exp(-x);
  return 1. / denom;
}

// clamps sigmoid between min & max
float sigmoid_clamp(float x, float minimum, float maximum) {
  return minimum + sigmoid(x) * (maximum - minimum);
}

void main() {
  // get normal direction to light
  vec3 toLight = normalize(globalLightPosition - v_Vertex);

  // get dot between surface normal & direction to light, essentially gives a 
  // number where the closer to 0 the more the normal is facing away from the lightsource 
  float cosAngle = dot(normalize(v_Normal), toLight);

  // sigmoid clamp to prevent full darkness areas 
  cosAngle = sigmoid_clamp(cosAngle, MIN_LIGHT, MAX_LIGHT);

  // Scale the color of this fragment based on its angle to the light.

  float alpha = 1.;
  float plotZ = abs(v_GraphVertex.z);

  if(plotZ > CLAMP_Z) {
    alpha -= CLAMP_SCATTER * (plotZ - CLAMP_Z);
  }
  alpha = clamp(alpha, 0., 1.);

  // multiplies color by cosAngle 
  gl_FragColor = vec4(vec3(graphColor) * cosAngle, graphColor.a * alpha);
}
