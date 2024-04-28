use std::marker::PhantomData;

use itertools::Itertools;
use macroquad::prelude::*;
use rayon::prelude::*;

use crate::kernel;
use crate::kernel::Kernel;
use crate::util_3d::*;

pub struct BeakerTeschner07<T: kernel::Kernel + Sync + Send> {
    _kernel: PhantomData<T>,
}

impl<T: kernel::Kernel + Sync + Send> BeakerTeschner07<T> {
    pub fn new() -> Self {
        Self {
            _kernel: PhantomData::default(),
        }
    }

    pub fn accelration(&self, space: &Space) -> Vec<Vec3> {
        space
            .particles()
            .map(|a| {
                let kernel = T::new(a.kernel_radius);
                let others = space.neighbour(a, kernel.support_radius());

                let mut sum = Vec3::ZERO;
                let mut color_field_gradient = Vec3::ZERO;
                let mut color_field_laplacian = Vec3::ZERO;
                others.for_each(|b| {
                    let r = a.position - b.position;
                    sum += b.mass * kernel.function(r) * r;
                    color_field_gradient += b.mass * kernel.gradient(r) / b.density;
                    color_field_laplacian += b.mass * kernel.laplacian(r) / b.density;
                });

                let kappa = -color_field_laplacian.length_squared() / color_field_gradient.length();
                kappa / a.mass * sum
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use self::{init_setup::create_sphere, kernel::CubicSpline};

    use super::*;
    use crate::model::density::Density;

    // surface_tension should all point to the (0.,0.,0.)
    #[test]
    fn direction_check() {
        let h = 5.;
        let mass = 1.;

        let density_model = Density::<CubicSpline>::new();
        let surface_tension_model = BeakerTeschner07::<CubicSpline>::new();
        let particle = create_sphere(mass, 1., 50, Vec3::ZERO, h);
        let mut space = Space::new(h, particle);

        density_model.update_density(&mut space);
        let surface_tension = surface_tension_model.accelration(&space);

        for (p, st) in space.particles().zip(surface_tension) {
            let pos = p.position;
            let dot = pos.dot(st);
            let magnitude = pos.length() * st.length();
            let diff = (dot + magnitude).abs();
            dbg!(pos, st, dot, magnitude, diff);
            assert!(diff <= 1e-3, "Value of diff: {:?}", diff);
        }
    }
}
