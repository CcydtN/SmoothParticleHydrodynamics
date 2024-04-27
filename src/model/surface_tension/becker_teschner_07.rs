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
    pub fn compute_accelration(
        &self,
        grid: &SpatialHashGrid,
        position: &Vec<Vec3>,
        density: &Vec<f32>,
    ) -> Vec<Vec3> {
        let mut accelration = vec![];
        for i in 0..position.len() {
            let mut color_field_gradient = Vec3::ZERO;
            let mut color_field_lapacian = Vec3::ZERO;
            let mut sum = Vec3::ZERO;
            for &j in grid.lookup(&position[i], self.kernel.support_radius()) {
                let r = position[i] - position[j];
                color_field_gradient += self.mass * self.kernel.gradient(r) / density[j];
                color_field_lapacian += self.mass * self.kernel.laplacian(r) / density[j];
                sum += self.mass * self.kernel.function(r) * r;
            }
            let n = color_field_gradient.length();
            if n <= f32::EPSILON {
                accelration.push(Vec3::ZERO);
                continue;
            }
            let kappa = -color_field_lapacian.length_squared() / n;
            accelration.push(kappa / self.mass * sum);
        }
        accelration.iter().for_each(|p| assert!(!p.is_nan()));
        accelration
    }

    pub fn accelration(&self, space: &Space) -> Vec<Vec3> {
        space
            .particles_with_neighbour(self.kernel.support_radius())
            .map(|(a, others)| -> Vec3 {
                let (sum, color_field_gradient, color_field_lapacian) =
                    others.fold((Vec3::ZERO, Vec3::ZERO, Vec3::ZERO), |acc, b| {
                        let r = a.position - b.position;
                        (
                            acc.0 + self.mass * self.kernel.function(r) * r,
                            acc.1 + self.mass * self.kernel.gradient(r) / b.density,
                            acc.2 + self.mass * self.kernel.laplacian(r) / b.density,
                        )
                    });
                let kappa = -color_field_lapacian.length_squared() / color_field_gradient.length();
                kappa / self.mass * sum
            })
            .collect_vec()
    }

    pub fn par_accelration(&self, space: &Space) -> Vec<Vec3> {
        space
            .particles_with_neighbour(self.kernel.support_radius())
            // .map(|(a, others)| -> Vec3 {
            //     let (sum, color_field_gradient, color_field_laplacian) = others
            //         .map(|b| {
            //             let r = a.position - b.position;
            //             (
            //                 b.mass * self.kernel.function(r) * r,
            //                 b.mass * self.kernel.gradient(r) / b.density,
            //                 b.mass * self.kernel.laplacian(r) / b.density,
            //             )
            //         })
            //         .reduce(
            //             || (Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),
            //             |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2),
            //         );
            .map(|(a, others)| -> Vec3 {
                let (sum, color_field_gradient, color_field_laplacian) =
                    others.fold((Vec3::ZERO, Vec3::ZERO, Vec3::ZERO), |acc, b| {
                        let r = a.position - b.position;
                        (
                            acc.0 + self.mass * self.kernel.function(r) * r,
                            acc.1 + self.mass * self.kernel.gradient(r) / b.density,
                            acc.2 + self.mass * self.kernel.laplacian(r) / b.density,
                        )
                    });
                dbg!(sum, color_field_gradient, color_field_laplacian);
                let kappa = -color_field_laplacian.length_squared() / color_field_gradient.length();
                kappa / a.mass * sum
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use self::init_setup::create_sphere;

    use super::*;
    use crate::{model::density::Density, update_density};
    use std::f32::consts::PI;

    // surface_tension should all point to the (0.,0.,0.)
    #[test]
    fn direction_check() {
        let h = 5.;
        let kernel = kernel::Poly6::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel, mass);
        let surface_tension_model = BeakerTeschner07::new(kernel, mass);
        let mut grid = SpatialHashGrid::new(h);

        let mut position = vec![];
        let split_count = 20;
        let spacing_angle = 2. * PI / split_count as f32;
        for n in 0..split_count / 2 {
            let angle = spacing_angle * n as f32;
            for offset in [0., PI] {
                let i = (angle + offset).sin();
                let j = (angle + offset).cos();
                position.push(vec3(i, j, 0.).normalize());
                position.push(vec3(i, 0., j).normalize());
            }
        }
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let surface_tension = surface_tension_model.compute_accelration(&grid, &position, &density);

        for (pos, st) in position.iter().zip(surface_tension) {
            let dot = pos.dot(st);
            let magnitude = pos.length() * st.length();
            let diff = (dot + magnitude).abs();
            dbg!(pos, st, dot, magnitude, diff);
            assert!(diff <= 1e-3, "Value of diff: {:?}", diff);
        }
    }

    #[test]
    fn direction_check_1() {
        let h = 5.;
        let kernel = kernel::Poly6::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel, mass);
        let surface_tension_model = BeakerTeschner07::new(kernel, mass);
        let particle = create_sphere(mass, 1., 50, Vec3::ZERO);
        let mut space = Space::new(h, particle);

        update_density(mass, &mut space, kernel);
        let surface_tension = surface_tension_model.par_accelration(&space);

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
