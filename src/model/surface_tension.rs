use macroquad::prelude::*;

use crate::kernel;
use crate::util_3d::spatial_hash_grid::SpatialHashGrid;

pub struct SurfaceTension<T: kernel::Kernel> {
    surface_tension_coeffecient: f32,
    mass: f32,
    kernel: T,
}

impl<T: kernel::Kernel> SurfaceTension<T> {
    pub fn new(surface_tension_coeffecient: f32, mass: f32, kernel: T) -> Self {
        Self {
            surface_tension_coeffecient,
            mass,
            kernel,
        }
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
            let mut color_field_lapacian = 0.;
            for &j in grid.lookup(&position[i]) {
                if i == j {
                    continue;
                }
                let r = position[i].distance(position[j]);
                let dir = (position[i] - position[j]).normalize_or_zero();
                if dir == Vec3::ZERO {
                    continue;
                }
                color_field_gradient += self.mass * self.kernel.gradient(r) / density[j] * dir;
                color_field_lapacian += self.mass * self.kernel.lapacian(r) / density[j];
            }
            if color_field_gradient == Vec3::ZERO {
                accelration.push(Vec3::ZERO);
                continue;
            }
            let kappa = -color_field_lapacian / color_field_gradient.distance(Vec3::ZERO);
            if kappa.is_nan() {
                accelration.push(Vec3::ZERO);
                continue;
            }
            accelration.push(
                -kappa * color_field_gradient * self.surface_tension_coeffecient / density[i],
            );
        }
        accelration.iter().for_each(|p| assert!(!p.is_nan()));
        accelration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::density::Density;

    #[test]
    fn direction_check() {
        let h = 5.;
        let kernel = kernel::Spiky::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel, mass);
        let surface_tension_model = SurfaceTension::new(1., mass, kernel);
        let mut grid = SpatialHashGrid::new(h);

        let position = vec![vec3(0., 0., 0.), vec3(0.5, 0.5, 0.5), vec3(1., 1., 1.)];
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let viscosity = surface_tension_model.compute_accelration(&grid, &position, &density);

        assert_eq!(viscosity[0], -viscosity[2]);
        assert_eq!(viscosity[1], Vec3::ZERO);
        // cross product of two equal dir vector is 0
        assert_eq!(viscosity[0].cross(vec3(1., 1., 1.)), Vec3::ZERO);
        assert_eq!(viscosity[2].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
    }
}
