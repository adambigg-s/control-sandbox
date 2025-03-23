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

// / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / / /
// texture shader
@vs texture_vertex
in vec3 position;
in vec3 v_normal;
in vec2 v_texcoord;

layout (binding = 0) uniform vs_params {
    mat4 model;
    mat4 view;
    mat4 projection;
};

out vec2 f_texcoord;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.);
    f_texcoord = v_texcoord;
}
@end

@fs texture_frag
in vec2 f_texcoord;

layout (binding = 0) uniform texture2D tex;
layout (binding = 1) uniform sampler samp;

out vec4 color;

void main() {
    color = texture(sampler2D(tex, samp), f_texcoord);
}
@end

@program texture texture_vertex texture_frag
