use macroquad::prelude::*;

use crate::kernel;
use crate::util_3d::*;

pub struct Simple<T: kernel::Kernel> {
    kernel: T,
    mass: f32,
    viscosity_constant: f32,
}

impl<T: kernel::Kernel> Simple<T> {
    pub fn new(kernel: T, mass: f32, viscosity_constant: f32) -> Self {
        Self {
            kernel,
            mass,
            viscosity_constant,
        }
    }
    pub fn compute_accelration(
        &self,
        grid: &SpatialHashGrid,
        position: &Vec<Vec3>,
        velocity: &Vec<Vec3>,
        density: &Vec<f32>,
    ) -> Vec<Vec3> {
        let mut force = vec![];
        for i in 0..position.len() {
            let mut sum = Vec3::ZERO;
            for &j in grid.lookup(&position[i], self.kernel.support_radius()) {
                sum += self.mass * (velocity[j] - velocity[i]) / density[j]
                    * self.kernel.laplacian(position[i] - position[j]);
            }
            force.push(sum * self.viscosity_constant / density[i]);
        }
        force.iter().for_each(|p| assert!(!p.is_nan()));
        force
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::density::Density;

    // #[test]
    // fn force_direction() {
    //     let h = 5.;
    //     let kernel = kernel::Viscosity::new(h);
    //     let mass = 1.;

    //     let density_model = Density::new(kernel, mass);
    //     let viscoity_model = Simple::new(kernel, 1., mass);
    //     let mut grid = SpatialHashGrid::new(h);

    //     let position = vec![vec3(0., 0., 0.), vec3(0.5, 0.5, 0.5), vec3(1., 1., 1.)];
    //     let velocity = vec![vec3(-1., -1., -1.), vec3(0., 0., 0.), vec3(1., 1., 1.)];
    //     grid.update(&position);
    //     let density = density_model.compute(&grid, &position);
    //     let viscosity = viscoity_model.compute_accelration(&grid, &position, &velocity, &density);

    //     assert_eq!(viscosity[0], -viscosity[2]);
    //     assert_eq!(viscosity[1], Vec3::ZERO);
    //     // cross product of two equal dir vector is 0
    //     assert_eq!(viscosity[0].cross(vec3(1., 1., 1.)), Vec3::ZERO);
    //     assert_eq!(viscosity[2].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
    // }
}
