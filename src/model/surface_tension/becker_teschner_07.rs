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
            .particles()
            .map(|a| {
                let kernel = T::new(a.kernel_radius);
                let others = space.neighbour(a, kernel.support_radius());
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
}

#[cfg(test)]
mod tests {
    use self::init_setup::create_sphere;

    use super::*;
    use crate::model::density::Density;

    // surface_tension should all point to the (0.,0.,0.)
    #[test]
    fn direction_check() {
        let h = 5.;
        let kernel = kernel::Poly6::new(h);
        let mass = 1.;

        let density_model = Density::new(kernel);
        let surface_tension_model = BeakerTeschner07::new(kernel, mass);
        let particle = create_sphere(mass, 1., 50, Vec3::ZERO);
        let mut space = Space::new(h, particle);

        density_model.update_density(&mut space);
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
