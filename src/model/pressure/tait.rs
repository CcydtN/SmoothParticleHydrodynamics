use macroquad::prelude::*;

use crate::kernel::Kernel;
use crate::util_3d::*;

#[derive(Debug)]
pub struct Tait<T: Kernel> {
    kernel: T,
    mass: f32,
    rest_density: f32,
    gamma: i32,
    pressure_constant: f32,
}

impl<T: Kernel + std::fmt::Debug> Tait<T> {
    pub fn new(
        kernel: T,
        mass: f32,
        rest_density: f32,
        gamma: i32,
        height: f32,
        gravity: f32,
    ) -> Self {
        let max_v_sq = 2. * gravity * height;
        let speed_of_flow_sq = 100. * max_v_sq;
        let pressure_constant = rest_density * speed_of_flow_sq / (gamma as f32);
        Self {
            kernel,
            mass,
            rest_density,
            gamma,
            pressure_constant,
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
        let p_func =
            |&d: &f32| self.pressure_constant * ((d / self.rest_density).powi(self.gamma) - 1.);
        let p = density.iter().map(p_func).collect::<Vec<_>>();

        let mut pressure = vec![];
        for i in 0..n {
            let mut tmp = Vec3::ZERO;
            for &j in grid.lookup(&position[i], self.kernel.support_radius()) {
                if i == j {
                    continue;
                }
                let gradient = self.kernel.gradient(position[i] - position[j]);
                tmp +=
                    -self.mass * (p[i] / density[i].powi(2) + p[j] / density[j].powi(2)) * gradient;
                debug_assert!(!tmp.is_nan());
            }
            pressure.push(tmp);
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
        let pressure_model = Tait::new(kernel, mass, 100.0, 7, 1., 9.81);
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
        let pressure_model = Tait::new(kernel, mass, 100.0, 7, 1., 9.81);
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
