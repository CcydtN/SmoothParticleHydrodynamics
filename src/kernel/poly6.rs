use std::f32::consts::PI;

use crate::kernel::common::Kernel;

#[derive(Debug, Clone, Copy)]
pub struct Poly6 {
    h: f32,
}

impl Poly6 {
    pub fn new(h: f32) -> Self {
        Self { h }
    }
}

impl Kernel for Poly6 {
    fn function(&self, r: f32) -> f32 {
        assert!(r >= 0.0, "value of r: {}", r);
        if r > self.h {
            return 0.;
        }
        let constant = 315. / (64. * PI * self.h.powi(9));
        constant * (self.h.powi(2) - r.powi(2)).powi(3)
    }

    fn gradient(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        let constant = -945. / (32. * PI * self.h.powi(9));
        constant * r * (self.h.powi(2) - r.powi(2)).powi(2)
    }

    fn lapacian(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        let constant = 945. / (32. * PI * self.h.powi(9));
        constant * (-self.h.powi(2) + 5. * r.powi(2)) * (self.h.powi(2) - r.powi(2))
    }
}
