use std::f32::consts::PI;

use crate::kernel::common::Kernel;

#[derive(Debug, Clone, Copy)]
pub struct Spiky {
    h: f32,
}

impl Spiky {
    pub fn new(h: f32) -> Self {
        Self { h }
    }
}

impl Kernel for Spiky {
    fn function(&self, r: f32) -> f32 {
        if r >= self.h {
            return 0.;
        }
        let constant = 15. / (PI * self.h.powi(6));
        constant * (self.h - r).powi(3)
    }

    fn gradient(&self, r: f32) -> f32 {
        if r >= self.h {
            return 0.;
        }
        let constant = -45. / (PI * self.h.powi(6));
        constant * (self.h - r).powi(2)
    }

    fn lapacian(&self, _r: f32) -> f32 {
        todo!()
    }
}
