#[derive(Copy, Clone)]
#[repr(packed)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub const fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn one() -> Vec3 {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub const fn vertical(scalar: f32) -> Vec3 {
        Vec3 {
            x: 0.0,
            y: scalar,
            z: 0.0,
        }
    }

    pub const fn horizontal(scalar: f32) -> Vec3 {
        Vec3 {
            x: scalar,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn forward_back(scalar: f32) -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: scalar,
        }
    }

    pub fn as_array(&self) -> [f32; 3] {
        unsafe { std::mem::transmute(*self) }
    }

    pub fn square_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        self.square_magnitude().sqrt()
    }

    pub fn add(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    pub fn sub(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn hadamard(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

    pub fn normalize(&self) -> Vec3 {
        let inv_mag = 1.0 / self.magnitude();
        Vec3 {
            x: self.x * inv_mag,
            y: self.y * inv_mag,
            z: self.z * inv_mag,
        }
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn lerp(&self, rhs: Vec3, t: f32) -> Vec3 {
        self.scale(1.0 - t).add(rhs.scale(t))
    }
}
