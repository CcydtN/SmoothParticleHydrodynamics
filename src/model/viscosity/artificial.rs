use itertools::Itertools;
use macroquad::prelude::*;

use crate::kernel;
use crate::util_3d::spatial_hash_grid::SpatialHashGrid;

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
            alpha: 0.1,
        }
    }

    pub fn compute_accelration(
        &self,
        grid: &SpatialHashGrid,
        position: &Vec<Vec3>,
        velocity: &Vec<Vec3>,
        density: &Vec<f32>,
    ) -> Vec<Vec3> {
        let pi_ab =
            |&pos_a: &Vec3, &pos_b: &Vec3, &v_a: &Vec3, &v_b: &Vec3, d_a: &f32, d_b: f32| -> f32 {
                let x = pos_a - pos_b;
                let v = v_a - v_b;
                let h = pos_a.y - pos_b.y;
                let numerator = x.dot(v);
                let denominator = x.length_squared() + 0.01 * h.powi(2);
                let constant = self.alpha * h.abs() * self.speed_sound.abs() / (d_a + d_b);
                numerator / denominator * constant
            };
        let viscosity = |((pos, v), d)| {
            grid.lookup(pos)
                .map(|&j| {
                    let r = *pos - position[j];
                    if r == Vec3::ZERO {
                        return r;
                    }
                    -self.mass
                        * pi_ab(pos, &position[j], v, &velocity[j], d, density[j])
                        * self.kernel.gradient(r)
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
        let viscosity = viscoity_model.compute_accelration(&grid, &position, &velocity, &density);

        assert_eq!(viscosity[0], -viscosity[2]);
        assert_eq!(viscosity[1], Vec3::ZERO);
        // cross product of two equal dir vector is 0
        assert_eq!(viscosity[0].cross(vec3(1., 1., 1.)), Vec3::ZERO);
        assert_eq!(viscosity[2].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
    }
}
