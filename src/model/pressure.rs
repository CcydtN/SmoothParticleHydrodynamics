use macroquad::prelude::*;

use crate::{kernel::common::Kernel, util_3d::spatial_hash_grid::SpatialHashGrid};

pub struct Pressure<T: Kernel> {
    k: f32,
    rest_density: f32,
    mass: f32,
    kernel: T,
}

impl<T: Kernel> Pressure<T> {
    pub fn new(k: f32, rest_density: f32, mass: f32, kernel: T) -> Self {
        Self {
            k,
            rest_density,
            mass,
            kernel,
        }
    }

    pub fn compute_accelraction(
        &self,
        grid: &SpatialHashGrid,
        position: &Vec<Vec3>,
        density: &Vec<f32>,
    ) -> Vec<Vec3> {
        assert_eq!(position.len(), density.len());
        let n = position.len();
        let p_func = |d| self.k * (d - self.rest_density);
        let p = density.iter().map(p_func).collect::<Vec<_>>();

        let mut pressure = vec![];
        for i in 0..n {
            let mut tmp = vec3(0., 0., 0.);
            for &j in grid.lookup(&position[i]) {
                if i == j {
                    continue;
                }
                let pressure = -self.mass * (p[i] + p[j]) / (2. * density[j])
                    * self.kernel.gradient((position[i]).distance(position[j]));
                // direction
                tmp += -pressure * (position[i] - position[j]).normalize();
            }
            pressure.push(tmp / density[i]);
        }
        pressure
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::spiky::Spiky;
    use crate::model::density::Density;

    // density > rest_density
    #[test]
    fn pressure_repulsion_test() {
        let h = 5.;
        let kernel = Spiky::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel, mass);
        let pressure_model = Pressure::new(100.0, 1., mass, kernel);
        let mut grid = SpatialHashGrid::new(h);

        let position = vec![vec3(0., 0., 0.), vec3(0.5, 0.5, 0.5), vec3(1., 1., 1.)];
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let pressure = pressure_model.compute_accelraction(&grid, &position, &density);

        assert_eq!(pressure[0], -pressure[2]);
        assert_eq!(pressure[1], Vec3::ZERO);

        assert_eq!(pressure[0].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
        assert_eq!(pressure[2].cross(vec3(1., 1., 1.)), Vec3::ZERO);
    }

    // density < rest_density
    #[test]
    fn pressure_attraction_test() {
        let h = 5.;
        let kernel = Spiky::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel, mass);
        let pressure_model = Pressure::new(100.0, 1., mass, kernel);
        let mut grid = SpatialHashGrid::new(h);

        let position = vec![vec3(0., 0., 0.), vec3(0.5, 0.5, 0.5), vec3(1., 1., 1.)];
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let pressure = pressure_model.compute_accelraction(&grid, &position, &density);

        assert_eq!(pressure[0], -pressure[2]);
        assert_eq!(pressure[1], Vec3::ZERO);
        // cross product of two equal dir vector is 0
        assert_eq!(pressure[0].cross(vec3(1., 1., 1.)), Vec3::ZERO);
        assert_eq!(pressure[2].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
    }
}
