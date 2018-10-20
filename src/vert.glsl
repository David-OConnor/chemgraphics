#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec4 face_color;
layout(location = 3) in float specular_intensity;

layout(location = 0) out vec3 v_normal;
layout(location = 1) out vec4 face_color2;
layout(location = 2) out vec3 diffuse_direction;
layout(location = 3) out vec4 ambient_color;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 view;
    mat4 proj;

    mat4 r_model;
    mat4 t_model;

    vec4 ambient_color;
    vec4 diffuse_color;
    vec3 diffuse_direction;

    float ambient_intensity;
    float diffuse_intensity;

    float shape_opacity;
} uniforms;

void main() {
    // gl_Position is a builtin name used to output the projected point.
//    mat4 worldview = uniforms.view * uniforms.model;

//    mat4 worldview = uniforms.r * uniforms.t * uniforms.model;

    mat4 model = uniforms.t_model * uniforms.r_model;

    mat4 worldview = uniforms.view * model;

    v_normal = transpose(inverse(mat3(uniforms.r_model))) * -normal;

    gl_Position = uniforms.proj * worldview * vec4(position, 1.);

    face_color2 = face_color;
    diffuse_direction = uniforms.diffuse_direction;
    ambient_color = uniforms.ambient_color;
}