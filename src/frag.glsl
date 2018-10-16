#version 450

layout(location = 0) in vec3 v_normal;
layout(location = 1) in vec4 face_color2;
layout(location = 2) in vec3 diffuse_direction;
layout(location = 3) in vec4 ambient_color;
// todo diffuse_color unused

layout(location = 0) out vec4 f_color;

//    float diffuse_weight = max(dot(norm, dir), 0.);
//    return uniforms.diffuse_color * diffuse_weight * uniforms.diffuse_intensity;

void main() {
    float brightness = dot(normalize(v_normal), normalize(diffuse_direction));
    vec3 dark_color = vec3(ambient_color);
    vec3 regular_color = vec3(face_color2);

    f_color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}