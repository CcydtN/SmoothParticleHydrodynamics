use macroquad::prelude::*;

use crate::kernel;
use crate::util_3d::*;

pub struct SurfaceTension<T: kernel::Kernel> {
    kernel: T,
    mass: f32,
    surface_tension_coefficient: f32,
}

impl<T: kernel::Kernel> SurfaceTension<T> {
    pub fn new(kernel: T, surface_tension_coefficient: f32, mass: f32) -> Self {
        Self {
            kernel,
            surface_tension_coefficient,
            mass,
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
            let mut color_field_lapacian = Vec3::ZERO;
            for &j in grid.lookup(&position[i], self.kernel.support_radius()) {
                let r = position[i] - position[j];
                color_field_gradient += self.mass / density[j] * self.kernel.gradient(r);
                color_field_lapacian += self.mass / density[j] * self.kernel.laplacian(r);
            }
            if color_field_gradient.is_nan() {
                panic!()
            }
            let n = color_field_gradient.length();
            if n <= 0.7 {
                accelration.push(Vec3::ZERO);
                continue;
            }
            accelration.push(
                self.surface_tension_coefficient / density[i]
                    * color_field_lapacian.length_squared()
                    * color_field_gradient.normalize_or_zero(),
            );
        }
        accelration.iter().for_each(|p| assert!(p.is_finite()));
        accelration
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::model::density::Density;
//     use std::f32::consts::PI;

//     // surface_tension should all point to the (0.,0.,0.)
//     #[test]
//     fn direction_check() {
//         let h = 5.;
//         let kernel = kernel::Poly6::new(h);
//         let mass = 1.;

//         let density_model = Density::new(kernel, mass);
//         let surface_tension_model = SurfaceTension::new(kernel, 1., mass);
//         let mut grid = SpatialHashGrid::new(h);

//         let mut position = vec![];
//         let split_count = 20;
//         let spacing_angle = 2. * PI / split_count as f32;
//         for n in 0..split_count / 2 {
//             let angle = spacing_angle * n as f32;
//             for offset in [0., PI] {
//                 let i = (angle + offset).sin();
//                 let j = (angle + offset).cos();
//                 position.push(vec3(i, j, 0.).normalize());
//                 position.push(vec3(i, 0., j).normalize());
//             }
//         }
//         grid.update(&position);
//         let density = density_model.compute(&grid, &position);
//         let surface_tension = surface_tension_model.compute_accelration(&grid, &position, &density);

//         for (pos, st) in position.iter().zip(surface_tension) {
//             let dot = pos.dot(st);
//             let magnitude = pos.length() * st.length();
//             // dot should be negetive
//             // magnitude should be positive
//             // same value, not same direction
//             let diff = (dot + magnitude).abs();
//             dbg!(pos, st, dot, magnitude, diff);
//             assert!(diff <= 1e-3, "Value of diff: {:?}", diff);
//         }
//     }
// }
