// / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
// simple shader
@vs simple_vertex
in vec3 position;
in vec3 v_color;

out vec3 f_color;

void main() {
    gl_Position = vec4(position, 1.);
    f_color = v_color;
}
@end

@fs simple_frag
in vec3 f_color;

out vec4 color;

void main() {
    color = vec4(f_color, 1.);
}
@end

@program simple simple_vertex simple_frag
