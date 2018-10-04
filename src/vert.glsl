#version 450

layout(location = 0) in vec4 position;
layout(location = 1) in vec4 normal;
layout(location = 2) in float specular_intensity;

layout(location = 0) out vec4 color;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 view;
    mat4 proj;

    vec4 ambient_color;
    vec4 diffuse_color;
    vec4 diffuse_direction;

    float ambient_intensity;
    float diffuse_intensity;

    float shape_opacity;
} uniforms;

vec4 find_diffuse_color() {
    vec4 norm = normalize(uniforms.model * normalize(normal));
    vec4 dir = normalize(uniforms.diffuse_direction);
    // diffuse_weight is based on the andle of the face compared to the angle
    // of the incoming light.
    float diffuse_weight = max(dot(norm, dir), 0.);
    return uniforms.diffuse_color * diffuse_weight * uniforms.diffuse_intensity;
}

void main() {
    // gl_Position is a builtin name used to output the projected point.
    gl_Position = uniforms.proj * uniforms.view * uniforms.model * position;

    vec4 diffuse_color = find_diffuse_color();

//        view_posit = positioned_pt;
//        diffuse_direction = uniforms.diffuse_direction;
//        diffuse_color = uniforms.diffuse_color;
//        specular_intensity = uniforms.specular_intensity;

//        normal_ = uniforms.model * normal;
//        frag_pos = positioned_pt;
//        color = diffuse_color;
    color = vec4(1., 0., 0., 1.);
}