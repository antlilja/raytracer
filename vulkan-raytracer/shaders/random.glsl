#define PI 3.14159265359

void xor_shift(inout uint seed) {
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
}

float rand_uni(inout uint seed) {
    xor_shift(seed);
    return (uintBitsToFloat((seed & 0x007fffff) | 0x40000000) - 2.0) * 0.5;
}

float rand_bi(inout uint seed) {
    xor_shift(seed);
    return uintBitsToFloat((seed & 0x007fffff) | 0x40000000) - 3.0;
}

vec3 rand_unit_sphere(inout uint seed) {
    const float a = rand_uni(seed) * 2.0 * PI;
    const float z = rand_bi(seed);
    const float r = sqrt(1.0 - (z * z));
    return vec3(r * cos(a), r * sin(a), z);
}
