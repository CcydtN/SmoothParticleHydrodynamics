use itertools::Itertools;
use macroquad::prelude::*;
use rayon::prelude::*;

use crate::kernel;
use crate::kernel::Kernel;
use crate::util_3d::*;

pub struct BeakerTeschner07<T: kernel::Kernel + Sync + Send> {
    kernel: T,
    mass: f32,
}

impl<T: kernel::Kernel + Sync + Send> BeakerTeschner07<T> {
    pub fn new(kernel: T, mass: f32) -> Self {
        Self { kernel, mass }
    }

    pub fn accelration(&self, space: &Space) -> Vec<Vec3> {
        space
            .particles()
            .map(|a| {
                let kernel = T::new(a.kernel_radius);
                let others = space.neighbour(a, kernel.support_radius());

                let sum = others.clone().fold(Vec3::ZERO, |acc, b| {
                    let r = a.position - b.position;
                    acc + b.mass * self.kernel.function(r) * r
                });

                let color_field_gradient = others.clone().fold(Vec3::ZERO, |acc, b| {
                    let r = a.position - b.position;
                    acc + b.mass * self.kernel.gradient(r) / b.density
                });

                let color_field_laplacian = others.clone().fold(Vec3::ZERO, |acc, b| {
                    let r = a.position - b.position;
                    acc + b.mass * self.kernel.gradient(r) / b.density
                });

                let kappa = -color_field_laplacian.length_squared() / color_field_gradient.length();
                kappa / self.mass * sum
            })
            .collect_vec()
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
        let kernel = kernel::CubicSpline::new(h);
        let mass = 1.;

        let density_model = Density::<CubicSpline>::new();
        let surface_tension_model = BeakerTeschner07::new(kernel, mass);
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
