in vec3 position; // 3D point!

uniform mat4 projection;
uniform mat4 view;


void main() {
  gl_Position = projection * view * vec4(position, 1.);
}
