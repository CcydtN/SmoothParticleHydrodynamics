use std::f32::consts::PI;

use crate::kernel::Kernel;

#[derive(Debug, Clone, Copy)]
pub struct Viscosity {
    h: f32,
    volume: f32,
}

impl Viscosity {
    pub fn new(h: f32) -> Self {
        let volume = 2. * PI * h.powi(3) / 15.;
        Self { h, volume }
    }
}

impl Kernel for Viscosity {
    fn function_scaler(&self, r: f32) -> f32 {
        assert!(r >= 0.0, "value of r: {}", r);
        if r > self.h {
            return 0.;
        }
        (-r.powi(3) / 2. * self.h.powi(3) + (r / self.h).powi(2) + (self.h / (2. * r)) - 1.)
            / self.volume
    }

    fn gradient_scaler(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        (-self.h.powi(4) + 4. * self.h * r.powi(3) - 3. * r.powi(4))
            / (r.powi(2) * self.h.powi(3) * 2. * self.volume)
    }

    fn lapacian_scaler(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        (self.h / r.powi(3) + 2. / self.h.powi(2) - 3. * r / self.h.powi(3)) / self.volume
    }
}
