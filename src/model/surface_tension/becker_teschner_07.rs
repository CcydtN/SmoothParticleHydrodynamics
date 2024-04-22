use macroquad::prelude::*;

use crate::kernel;
use crate::util_3d::*;

pub struct BeakerTeschner07<T: kernel::Kernel> {
    kernel: T,
    mass: f32,
}

impl<T: kernel::Kernel> BeakerTeschner07<T> {
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
                if i == j {
                    continue;
                }
                let r = position[i] - position[j];
                color_field_gradient += self.mass * self.kernel.gradient(r) / density[j];
                color_field_lapacian += self.mass * self.kernel.lapacian(r) / density[j];
                sum += self.mass * self.kernel.function(r) * r;
            }
            let n = color_field_gradient;
            if n.length() == 0. {
                accelration.push(Vec3::ZERO);
                continue;
            }
            let kappa = -color_field_lapacian.length() / n.length();
            accelration.push(kappa / self.mass * sum);
        }
        accelration.iter().for_each(|p| assert!(!p.is_nan()));
        accelration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::density::Density;
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
}
