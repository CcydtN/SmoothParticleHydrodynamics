use crate::kernel;
use crate::util_3d::*;
use itertools::Itertools;
use macroquad::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct Density<T: kernel::Kernel> {
    kernel: T,
}

impl<T: kernel::Kernel> Density<T> {
    pub fn new(kernel: T) -> Self {
        Self { kernel }
    }

    pub fn update_density(&self, space: &mut Space) {
        let density = space
            .particles_with_neighbour(self.kernel.support_radius())
            .map(|(a, others)| {
                others
                    .map(|b| {
                        let r = a.position - b.position;
                        b.mass * self.kernel.function(r)
                    })
                    .sum::<f32>()
            })
            .collect_vec();

        space
            .particles_mut()
            .zip(density)
            .for_each(|(particle, d)| particle.density = d);
    }
}
