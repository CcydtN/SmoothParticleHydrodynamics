use std::marker::PhantomData;

use crate::kernel;
use crate::util_3d::*;
use itertools::Itertools;
use macroquad::prelude::*;
use rayon::prelude::*;

#[derive(Debug)]
pub(crate) struct Density<T: kernel::Kernel> {
    _phantom: PhantomData<T>,
}

impl<T: kernel::Kernel> Density<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }

    pub fn update_density(&self, space: &mut Space) {
        let density = space
            .particles()
            .map(|a| {
                let kernel = T::new(a.kernel_radius);
                let others = space.neighbour(a, kernel.support_radius());
                others
                    .map(|b| {
                        let r = a.position - b.position;
                        b.mass * kernel.function(r)
                    })
                    .sum::<f32>()
            })
            .collect::<Vec<_>>();

        space
            .particles_mut()
            .zip(density)
            .for_each(|(particle, d)| {
                particle.density = d;
                particle.kernel_radius =
                    1.3 * (particle.mass / (0.1 + particle.density)).powf(1. / 3.);
            });
    }
}
