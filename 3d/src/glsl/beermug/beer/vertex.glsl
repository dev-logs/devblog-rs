varying vec2 vUv;
attribute float aRandom;

varying float vRandom;

void main() {
    vec4 modelPosition = modelMatrix * vec4(position, 1.0);
    vec4 viewPosition = viewMatrix * modelPosition;
    vec4 projectedPositon = projectionMatrix * viewPosition;
    gl_Position = projectedPositon;

    vUv = vec2(modelPosition.y, modelPosition.z);
    vRandom = aRandom;
}
