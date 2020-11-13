#define EPSILON 0.0001

struct Ray {
    vec3 origin;
    vec3 direction;
};

vec3 ray_at(in Ray ray, in float t) { return ray.origin + (ray.direction * t); }

struct Sphere {
    vec3 center;
    float radius;
    int mat_index;
};

float sphere_intersect(in Sphere sphere, in Ray ray) {
    const vec3 oc = ray.origin - sphere.center;
    const float a = dot(ray.direction, ray.direction);
    const float half_b = dot(ray.direction, oc);
    const float c = dot(oc, oc) - (sphere.radius * sphere.radius);
    const float disc = half_b * half_b - a * c;

    if (disc < 0.0) {
        return -1.0;
    }
    else {
        return (-half_b - sqrt(disc)) / a;
    }
}

vec3 sphere_normal(in Sphere sphere, in vec3 point) {
    return normalize((point - sphere.center) / sphere.radius);
}

struct Plane {
    vec3 normal;
    float dist;
    int mat_index;
};

float plane_intersect(in Plane plane, in Ray ray) {
    const float denom = dot(plane.normal, ray.direction);

    if (abs(denom) > EPSILON) {
        return (plane.dist - dot(plane.normal, ray.origin)) / denom;
    }
    else {
        return -1.0;
    }
}
