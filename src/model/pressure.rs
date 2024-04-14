use macroquad::prelude::*;

use crate::kernel::common::Kernel;

pub struct Pressure<T: Kernel> {
    k: f32,
    rest_density: f32,
    kernel: T,
}

impl<T: Kernel> Pressure<T> {
    pub fn new(k: f32, rest_density: f32, kernel: T) -> Self {
        Self {
            k,
            rest_density,
            kernel,
        }
    }

    pub fn compute(&self, disp: &Vec<Vec3>, density: &Vec<f32>, mass: f32) -> Vec<Vec3> {
        assert_eq!(disp.len(), density.len());
        let n = disp.len();
        let p_func = |d| self.k * (d - self.rest_density);
        let p = density.iter().map(p_func).collect::<Vec<_>>();

        let mut pressure = vec![];
        for i in 0..n {
            let mut tmp = vec3(0., 0., 0.);
            for j in 0..n {
                if i == j {
                    continue;
                }
                let pressure = -mass * (p[i] + p[j]) / (2. * density[j])
                    * self.kernel.gradient((disp[i]).distance(disp[j]));
                // direction
                tmp += -pressure * (disp[i] - disp[j]).normalize();
            }
            pressure.push(tmp);
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
        let kernel = Spiky::new(5.);
        let density_model = Density::new(kernel);
        let pressure_model = Pressure::new(100.0, 1., kernel);

        let disp = vec![vec3(0., 0., 0.), vec3(0.5, 0.5, 0.5), vec3(1., 1., 1.)];
        let mass = 1.;
        let density = density_model.compute(&disp, mass);
        let pressure = pressure_model.compute(&disp, &density, mass);

        assert_eq!(pressure[0], -pressure[2]);
        assert_eq!(pressure[1], Vec3::ZERO);

        assert_eq!(pressure[0].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
        assert_eq!(pressure[2].cross(vec3(1., 1., 1.)), Vec3::ZERO);
    }

    // density < rest_density
    #[test]
    fn pressure_attraction_test() {
        let kernel = Spiky::new(5.);
        let density_model = Density::new(kernel);
        let pressure_model = Pressure::new(100.0, 0.5, kernel);

        let disp = vec![vec3(0., 0., 0.), vec3(1., 1., 1.), vec3(2., 2., 2.)];
        let mass = 1.;
        let density = density_model.compute(&disp, mass);
        let pressure = pressure_model.compute(&disp, &density, mass);

        assert_eq!(pressure[0], -pressure[2]);
        assert_eq!(pressure[1], Vec3::ZERO);
        // cross product of two equal dir vector is 0
        assert_eq!(pressure[0].cross(vec3(1., 1., 1.)), Vec3::ZERO);
        assert_eq!(pressure[2].cross(vec3(-1., -1., -1.)), Vec3::ZERO);
    }
}
