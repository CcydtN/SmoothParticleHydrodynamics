use std::marker::PhantomData;

use itertools::Itertools;
use macroquad::prelude::*;

use crate::kernel::Kernel;
use crate::util_3d::*;

#[derive(Debug)]
pub struct Tait<T: Kernel> {
    kernel: PhantomData<T>,
    rest_density: f32,
    gamma: i32,
    pressure_constant: f32,
}

impl<T: Kernel + std::fmt::Debug> Tait<T> {
    pub fn new(rest_density: f32, gamma: i32, speed_of_sound: f32) -> Self {
        let pressure_constant = rest_density * (10. * speed_of_sound) / (gamma as f32);
        Self {
            rest_density,
            gamma,
            pressure_constant,
            kernel: PhantomData::default(),
        }
    }

    pub fn update_pressure(&self, space: &mut Space) {
        space.particles_mut().for_each(|particle| {
            particle.pressure = ((particle.density / self.rest_density).powi(self.gamma) - 1.)
                * self.pressure_constant;
        })
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
                        -b.mass
                            * (a.pressure / a.density.powi(2) + b.pressure / b.density.powi(2))
                            * kernel.gradient(r)
                    })
                    .fold(Vec3::ZERO, |a, b| a + b)
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::{self, CubicSpline};
    use crate::model::density::Density;

    // density > rest_density
    #[test]
    fn repulsion() {
        let h = 5.;
        let mass = 1.;

        let density_model = Density::<CubicSpline>::new();
        let pressure_model = Tait::<CubicSpline>::new(2., 7, 2. * 9.81);
        let particle = init_setup::diagonal_test(mass, h);
        let mut space = Space::new(h, particle);

        dbg!(&space);
        density_model.update_density(&mut space);
        pressure_model.update_pressure(&mut space);
        let pressure = pressure_model.accelration(&space);

        dbg!(&pressure, space);
        assert!(pressure[1].length() <= f32::EPSILON);
        assert!(pressure[0].normalize().dot(Vec3::NEG_ONE) <= f32::EPSILON);
        assert!(pressure[2].normalize().dot(Vec3::ONE) <= f32::EPSILON);
    }

    // density < rest_density
    #[test]
    fn attraction() {
        let h = 5.;
        let mass = 1.;

        let density_model = Density::<CubicSpline>::new();
        let pressure_model = Tait::<CubicSpline>::new(0.5, 7, 2. * 9.81);
        let particle = init_setup::diagonal_test(mass, h);
        let mut space = Space::new(h, particle);

        density_model.update_density(&mut space);
        pressure_model.update_pressure(&mut space);
        let pressure = pressure_model.accelration(&space);

        dbg!(&pressure, space);
        assert!(pressure[1].length() <= f32::EPSILON);
        assert!(pressure[0].normalize().dot(Vec3::NEG_ONE) <= f32::EPSILON);
        assert!(pressure[2].normalize().dot(Vec3::ONE) <= f32::EPSILON);
    }
}
