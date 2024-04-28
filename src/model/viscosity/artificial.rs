use itertools::Itertools;
use macroquad::prelude::*;

use crate::kernel;
use crate::util_3d::*;

pub struct Artificial<T: kernel::Kernel> {
    kernel: T,
    alpha: f32,
    speed_sound: f32,
}

impl<T: kernel::Kernel> Artificial<T> {
    pub fn new(kernel: T, mass: f32, speed_sound: f32) -> Self {
        assert!(speed_sound > 0.0);
        // alpha between 0.08 and 0.5
        Self {
            kernel,
            speed_sound,
            alpha: 0.2,
        }
    }

    pub fn accelration(&self, space: &Space) -> Vec<Vec3> {
        space
            .particles()
            .map(|a| {
                let kernel = T::new(a.kernel_radius);
                let others = space.neighbour(a, kernel.support_radius());
                others
                    .map(|b| {
                        let r = a.position - b.position;
                        let v = a.velocity - b.velocity;
                        if r.dot(v) >= 0. {
                            return Vec3::ZERO;
                        }
                        let h = (a.kernel_radius + b.kernel_radius) / 2.;
                        let numerator = r.dot(v);
                        let denominator = r.length_squared() + 0.01 * h.powi(2);
                        let constant =
                            (2. * self.alpha * h * self.speed_sound) / (a.density + b.density);
                        b.mass * self.kernel.gradient(r) * constant * numerator / denominator
                    })
                    .fold(Vec3::ZERO, |a, b| a + b)
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{kernel::*, model::density::Density};

    #[test]
    fn direction_check() {
        let h = 5.;
        let kernel = CubicSpline::new(h);
        let mass = 1.;
        let speed_sound = 10. * ((2. * 9.81 * 0.5) as f32).sqrt();

        let density_model = Density::<CubicSpline>::new();
        let viscoity_model = Artificial::new(kernel, 1., speed_sound);

        let particle = init_setup::diagonal_test(mass, h);
        let mut space = Space::new(h, particle);

        density_model.update_density(&mut space);
        let viscosity = viscoity_model.accelration(&space);

        dbg!(&viscosity, &space);
        assert!(viscosity[1].length() <= f32::EPSILON);
        assert!(viscosity[0].normalize().dot(Vec3::ONE) <= f32::EPSILON);
        assert!(viscosity[2].normalize().dot(Vec3::NEG_ONE) <= f32::EPSILON);
    }
}
