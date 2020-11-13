use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    pub fn intersect(&self, ray: Ray) -> f32 {
        const TOLERENCE: f32 = 0.0001;
        let denom = self.normal.dot(ray.direction);

        if denom < -TOLERENCE || denom > TOLERENCE {
            (self.distance - self.normal.dot(ray.origin)) / denom
        } else {
            -1.0
        }
    }
}
