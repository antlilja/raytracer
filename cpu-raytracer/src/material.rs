use crate::Vec3;

#[derive(Copy, Clone)]
pub struct Material {
    pub reflection: Vec3,
    pub emission: Vec3,
    pub scattering: f32,
}
