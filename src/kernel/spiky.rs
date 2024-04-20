use std::f32::consts::PI;

use crate::kernel::common::Kernel;

#[derive(Debug, Clone, Copy)]
pub struct Spiky {
    h: f32,
    volume: f32,
}

impl Spiky {
    pub fn new(h: f32) -> Self {
        let volume = h.powi(6) * PI / 15.;
        Self { h, volume }
    }
}

impl Kernel for Spiky {
    fn function_scaler(&self, r: f32) -> f32 {
        assert!(r >= 0.0, "value of r: {}", r);
        if r > self.h {
            return 0.;
        }
        (self.h - r).powi(3) / self.volume
    }

    fn gradient_scaler(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        -3. * (self.h - r).powi(2) / self.volume
    }

    fn lapacian_scaler(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        6. * (self.h - r) / self.volume
    }
}
