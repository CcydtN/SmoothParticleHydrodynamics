use itertools::Itertools;
use macroquad::prelude::*;

use crate::kernel;
use crate::util_3d::*;

pub struct Artificial<T: kernel::Kernel> {
    kernel: T,
    mass: f32,
    alpha: f32,
    speed_sound: f32,
}

impl<T: kernel::Kernel> Artificial<T> {
    pub fn new(kernel: T, mass: f32, speed_sound: f32) -> Self {
        assert!(speed_sound > 0.0);
        // alpha between 0.08 and 0.5
        Self {
            kernel,
            mass,
            speed_sound,
            alpha: 0.2,
        }
    }

    pub fn compute_accelration(
        &self,
        grid: &SpatialHashGrid,
        position: &Vec<Vec3>,
        velocity: &Vec<Vec3>,
        density: &Vec<f32>,
        h: f32,
    ) -> Vec<Vec3> {
        // h should be (h_a+h_b)/2 in the original paper
        let pi_ab = |&r: &Vec3, &v: &Vec3, d_a: &f32, d_b: f32, h: f32| -> f32 {
            let numerator = r.dot(v);
            let denominator = r.length_squared() + 0.01 * h.powi(2);
            let constant = (2. * self.alpha * h * self.speed_sound) / (d_a + d_b);
            -constant * numerator / denominator
        };

        let viscosity = |((pos, vel), d): ((&Vec3, &Vec3), &f32)| {
            grid.lookup(pos, self.kernel.support_radius())
                .map(|&j| {
                    let r = *pos - position[j];
                    let v = *vel - velocity[j];
                    if r.dot(v) >= 0. {
                        return Vec3::ZERO;
                    }
                    -self.mass * pi_ab(&r, &v, d, density[j], h) * self.kernel.gradient(r)
                })
                .fold(Vec3::ZERO, |a, b| a + b)
        };

        position
            .iter()
            .zip(velocity.iter())
            .zip(density.iter())
            .map(viscosity)
            .collect_vec()
    }

    pub fn accelration(
        &self,
        a: &Particle,
        b: &Particle,
        kernel_radius: f32,
        gradient: Vec3,
    ) -> Vec3 {
        let r = a.position - b.position;
        let v = a.velocity - b.velocity;
        if r.dot(v) >= 0. {
            return Vec3::ZERO;
        }
        let numerator = r.dot(v);
        let denominator = r.length_squared() + 0.01 * kernel_radius.powi(2);
        let constant =
            (2. * self.alpha * kernel_radius * self.speed_sound) / (a.density + b.density);
        self.mass * gradient * constant * numerator / denominator
    }

    pub fn accelration_(&self, space: &Space, kernel_radius: f32) -> Vec<Vec3> {
        space
            .particles_with_neighbour(self.kernel.support_radius())
            .map(|(a, others)| -> Vec3 {
                others
                    .map(|b| {
                        let r = a.position - b.position;
                        let v = a.velocity - b.velocity;
                        if r.dot(v) >= 0. {
                            return Vec3::ZERO;
                        }
                        let numerator = r.dot(v);
                        let denominator = r.length_squared() + 0.01 * kernel_radius.powi(2);
                        let constant = (2. * self.alpha * kernel_radius * self.speed_sound)
                            / (a.density + b.density);
                        self.mass * self.kernel.gradient(r) * constant * numerator / denominator
                    })
                    .fold(Vec3::ZERO, |a, b| a + b)
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::density::Density;

    #[test]
    fn force_direction() {
        let h = 5.;
        let kernel = kernel::Viscosity::new(h);
        let mass = 1.;
        let speed_sound = 10. * ((2. * 9.81 * 0.5) as f32).sqrt();

        let density_model = Density::new(kernel, mass);
        let viscoity_model = Artificial::new(kernel, 1., speed_sound);
        let mut grid = SpatialHashGrid::new(h);

        let position = vec![vec3(0., 0., 0.), vec3(0.5, 0.5, 0.5), vec3(1., 1., 1.)];
        let velocity = vec![vec3(-1., -1., -1.), vec3(0., 0., 0.), vec3(1., 1., 1.)];
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let viscosity =
            viscoity_model.compute_accelration(&grid, &position, &velocity, &density, h);

        assert_eq!(viscosity[0], -viscosity[2]);
        assert_eq!(viscosity[1], Vec3::ZERO);
        // cross product of two equal dir vector is 0
        assert_eq!(viscosity[0].cross(vec3(1., 1., 1.)), Vec3::ZERO);
        assert_eq!(viscosity[2].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
    }
}
