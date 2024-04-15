use std::f32::consts::PI;

use crate::kernel::common::Kernel;

#[derive(Debug, Clone, Copy)]
pub struct Viscosity {
    h: f32,
}

impl Viscosity {
    pub fn new(h: f32) -> Self {
        Self { h }
    }
}

impl Kernel for Viscosity {
    fn function(&self, r: f32) -> f32 {
        assert!(r >= 0.0, "value of r: {}", r);
        if r > self.h {
            return 0.;
        }
        let constant = 15. / (2. * PI * self.h.powi(3));
        constant
            * (-r.powi(3) / 2. * self.h.powi(3) + (r / self.h).powi(2) + (self.h / (2. * r)) - 1.)
    }

    fn gradient(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        let constant = 15. / (4. * PI * self.h.powi(6));
        constant * (-self.h.powi(4) + 4. * self.h * r.powi(3) - 3. * r.powi(4)) / r.powi(2)
    }

    fn lapacian(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        let constant = 15. / (2. * PI * self.h.powi(6));
        constant * (self.h.powi(4) + 2. * self.h * r.powi(3) - 3. * r.powi(4)) / r.powi(3)
    }
}
