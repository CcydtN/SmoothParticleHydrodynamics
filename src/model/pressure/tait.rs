use itertools::Itertools;
use macroquad::prelude::*;

use crate::kernel::Kernel;
use crate::util_3d::*;

#[derive(Debug)]
pub struct Tait<T: Kernel> {
    kernel: T,
    rest_density: f32,
    gamma: i32,
    pressure_constant: f32,
}

impl<T: Kernel + std::fmt::Debug> Tait<T> {
    pub fn new(kernel: T, rest_density: f32, gamma: i32, speed_of_sound: f32) -> Self {
        let pressure_constant = rest_density * (10. * speed_of_sound) / (gamma as f32);
        Self {
            kernel,
            rest_density,
            gamma,
            pressure_constant,
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
            .particles_with_neighbour(self.kernel.support_radius())
            .map(|(a, others)| -> Vec3 {
                others
                    .map(|b| {
                        let r = a.position - b.position;
                        -b.mass
                            * (a.pressure / a.density.powi(2) + b.pressure / b.density.powi(2))
                            * self.kernel.gradient(r)
                    })
                    .fold(Vec3::ZERO, |a, b| a + b)
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel;
    use crate::model::density::Density;

    // density > rest_density
    #[test]
    fn repulsion() {
        let h = 5.;
        let kernel = kernel::CubicSpline::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel);
        let pressure_model = Tait::new(kernel, 2., 7, 2. * 9.81);
        let particle = init_setup::diagonal_3_points(mass);
        let mut space = Space::new(h, particle);

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
        let kernel = kernel::Spiky::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel);
        let pressure_model = Tait::new(kernel, 0.5, 7, 2. * 9.81);
        let particle = init_setup::diagonal_3_points(mass);
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
