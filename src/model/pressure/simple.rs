use macroquad::prelude::*;

use crate::kernel;
use crate::util_3d::spatial_hash_grid::SpatialHashGrid;

pub struct Simple<T: kernel::Kernel> {
    kernel: T,
    mass: f32,
    rest_density: f32,
    k: f32,
}

impl<T: kernel::Kernel> Simple<T> {
    pub fn new(kernel: T, mass: f32, k: f32, rest_density: f32) -> Self {
        Self {
            kernel,
            mass,
            rest_density,
            k,
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
        pressure.iter().for_each(|p| assert!(!p.is_nan()));
        pressure
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel;
    use crate::model::density::Density;

    // density > rest_density
    #[test]
    fn pressure_repulsion_test() {
        let h = 5.;
        let kernel = kernel::Spiky::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel, mass);
        let pressure_model = Simple::new(kernel, mass, 100.0, 1.);
        let mut grid = SpatialHashGrid::new(h);

        let position = vec![vec3(0., 0., 0.), vec3(0.5, 0.5, 0.5), vec3(1., 1., 1.)];
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let pressure = pressure_model.compute_accelraction(&grid, &position, &density);

        assert!(pressure[1].distance(Vec3::ZERO) <= 0.999);
        assert_eq!(pressure[0].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
        assert_eq!(pressure[2].cross(vec3(1., 1., 1.)), Vec3::ZERO);
    }

    // density < rest_density
    #[test]
    fn pressure_attraction_test() {
        let h = 5.;
        let kernel = kernel::Spiky::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel, mass);
        let pressure_model = Simple::new(kernel, mass, 100.0, 1.);
        let mut grid = SpatialHashGrid::new(h);

        let position = vec![vec3(0., 0., 0.), vec3(0.5, 0.5, 0.5), vec3(1., 1., 1.)];
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let pressure = pressure_model.compute_accelraction(&grid, &position, &density);

        assert!(pressure[1].distance(Vec3::ZERO) <= 0.999);
        assert_eq!(pressure[0].cross(vec3(1., 1., 1.)), Vec3::ZERO);
        assert_eq!(pressure[2].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
    }
}
