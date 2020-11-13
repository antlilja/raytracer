use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(origin: Vec3, focal_length: f32, fov: f32, aspect_ratio: f32) -> Camera {
        let viewport_height = 2.0 * (fov * 0.5).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let horizontal = Vec3::horizontal(viewport_width);
        let vertical = Vec3::vertical(viewport_height);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left: origin
                .sub(horizontal.scale(0.5))
                .sub(vertical.scale(0.5))
                .sub(Vec3::forward_back(focal_length)),
        }
    }

    pub fn look_at(origin: Vec3, target: Vec3, fov: f32, aspect_ratio: f32) -> Camera {
        let viewport_height = 2.0 * (fov * 0.5).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let f = origin.sub(target).normalize();
        let h = Vec3::vertical(1.0).cross(f).normalize();
        let v = f.cross(h);

        let horizontal = h.scale(viewport_width);
        let vertical = v.scale(viewport_height);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left: origin
                .sub(horizontal.scale(0.5))
                .sub(vertical.scale(0.5))
                .sub(f),
        }
    }

    pub fn ray_from_uv(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self
                .lower_left
                .add(self.horizontal.scale(u))
                .add(self.vertical.scale(v))
                .sub(self.origin),
        }
    }
}
