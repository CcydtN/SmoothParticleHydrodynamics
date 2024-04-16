use std::f32::consts::PI;

use crate::kernel::common::Kernel;

#[derive(Debug, Clone, Copy)]
pub struct Poly6 {
    h: f32,
    volume: f32,
}

impl Poly6 {
    pub fn new(h: f32) -> Self {
        let volume = h.powi(8) * PI / 4.;
        Self { h, volume }
    }
}

impl Kernel for Poly6 {
    fn function(&self, r: f32) -> f32 {
        assert!(r >= 0.0, "value of r: {}", r);
        if r > self.h {
            return 0.;
        }
        (self.h.powi(2) - r.powi(2)).powi(3) / self.volume
    }

    fn gradient(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        -6. * r * (self.h.powi(2) - r.powi(2)).powi(2) / self.volume
    }

    fn lapacian(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        -6. * (self.h.powi(2) - 5. * r.powi(2)) * (self.h.powi(2) - r.powi(2)) / self.volume
    }
}
