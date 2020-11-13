#version 450

#include "geometry.glsl"
#include "random.glsl"

#define EPSILON 0.0001

struct Material {
    vec3 reflection;
    vec3 emission;
    float scattering;
};

layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

layout(binding = 0, rgba8) uniform writeonly image2D img;

layout(binding = 1) buffer Seeds { uint seeds[]; };
layout(binding = 2) buffer Materials { Material materials[]; };
layout(binding = 3) buffer Spheres { Sphere spheres[]; };
layout(binding = 4) buffer Planes { Plane planes[]; };

layout(push_constant) uniform Camera {
    vec3 origin;
    vec3 horizontal;
    vec3 vertical;
    vec3 lower_left;
    uint bounces;
    uint samples;
}
camera;

Ray ray_from_uv(float u, float v) {
    return Ray(camera.origin, normalize(camera.lower_left + (u * camera.horizontal) +
                                  (v * camera.vertical) - camera.origin));
}

vec3 sky_color(in Ray ray) {
    const float t = (normalize(ray.direction).y + 1.0) * 0.5;
    return mix(vec3(1.0), vec3(0.5, 0.7, 1.0), t);
}

vec3 ray_color(Ray ray, inout uint seed) {
    vec3 attenuation = vec3(1.0);
    vec3 color = vec3(0.0);
    for (int b = 0; b < camera.bounces; ++b) {
        float min_hit = 1000000.0;
        int material_index = -1;
        vec3 normal = vec3(0.0);

        for (int i = 0; i < spheres.length(); ++i) {
            const Sphere sphere = spheres[i];
            const float t = sphere_intersect(sphere, ray);
            if (t > EPSILON && t < min_hit) {
                min_hit = t;
                material_index = sphere.mat_index;
                normal = sphere_normal(sphere, ray_at(ray, min_hit));
            }
        }

        for (int i = 0; i < planes.length(); ++i) {
            const Plane plane = planes[i];
            const float t = plane_intersect(plane, ray);
            if (t > EPSILON && t < min_hit) {
                min_hit = t;
                material_index = plane.mat_index;
                ;
                normal = plane.normal;
            }
        }

        if (material_index == -1) {
            color += (attenuation * sky_color(ray)) / (float(b) * PI);
            break;
        }
        else {
            const Material material = materials[material_index];

            const vec3 diffuse = normal + rand_unit_sphere(seed);
            const vec3 mirror =
                ray.direction - (normal * 2.0 * dot(normal, ray.direction));

            const vec3 direction =
                normalize(mix(mirror, diffuse, material.scattering));

            ray = Ray(ray_at(ray, min_hit), direction);

            color += attenuation * material.emission;
            attenuation = (attenuation * material.reflection * (1.0 / PI));
        }
    }

    return color;
}

void main() {
    const ivec2 size = imageSize(img);
    const vec2 size_inverse = vec2(1.0) / (vec2(size) - vec2(1.0));

    const vec2 coord = vec2(gl_GlobalInvocationID.xy) * vec2(1.0, -1.0) +
                       vec2(0.0, float(size.y));

    uint index = (gl_GlobalInvocationID.y * size.x) + gl_GlobalInvocationID.x;
    uint seed = seeds[index];

    const float samples_inverse = 1.0 / camera.samples;

    vec3 color = vec3(0.0);
    for (int s = 0; s < camera.samples; ++s) {
        const vec2 dither = vec2(rand_bi(seed), rand_bi(seed));
        const vec2 uv = (coord + dither) * size_inverse;
        const Ray ray = ray_from_uv(uv.x, uv.y);
        color += ray_color(ray, seed);
    }

    const vec3 adjusted_color =
        clamp(sqrt(color * samples_inverse), vec3(0.0), vec3(1.0));
    imageStore(img, ivec2(gl_GlobalInvocationID.xy), vec4(adjusted_color, 1.0));
}
