use crate::Vec3;

pub struct RngXorShift {
    seed: u32,
}

impl RngXorShift {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    fn xor_shift(&mut self) -> u32 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        self.seed
    }

    pub fn uni(&mut self) -> f32 {
        self.xor_shift();
        let f: f32 = unsafe { std::mem::transmute((self.seed & 0x007fffff) | 0x40000000) };
        return (f - 2.0) * 0.5;
    }

    pub fn bi(&mut self) -> f32 {
        self.xor_shift();
        let f: f32 = unsafe { std::mem::transmute((self.seed & 0x007fffff) | 0x40000000) };
        return f - 3.0;
    }

    pub fn unit(&mut self) -> Vec3 {
        let a = self.uni() * 2.0 * std::f32::consts::PI;
        let z = self.bi();
        let r = (1.0 - z * z).sqrt();
        Vec3 {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }
}
