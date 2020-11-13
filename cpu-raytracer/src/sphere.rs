use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn intersect(&self, ray: Ray) -> f32 {
        let oc = ray.origin.sub(self.center);
        let a = ray.direction.square_magnitude();
        let half_b = ray.direction.dot(oc);
        let c = oc.square_magnitude() - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            // let term = discriminant.sqrt() / a;
            // let t0 = -half_b + term;
            // let t1 = -half_b - term;
            // Some(if t1 > t0 { t0 } else { t1 })
            (-half_b - discriminant.sqrt()) / a
        }
    }

    pub fn normal(&self, point: Vec3) -> Vec3 {
        point.sub(self.center).normalize()
    }
}
