use crate::kernel::common::Kernel;
use macroquad::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct Density<T: Kernel> {
    kernel: T,
}

impl<T: Kernel> Density<T> {
    pub fn new(kernel: T) -> Self {
        Self { kernel }
    }

    pub fn compute(&self, disp: &Vec<Vec3>, mass: f32) -> Vec<f32> {
        let mut density = vec![];
        for i in disp {
            let mut tmp = 0.;
            for j in disp {
                tmp += mass * self.kernel.function(i.distance(*j))
            }
            density.push(tmp)
        }
        density
    }
}
