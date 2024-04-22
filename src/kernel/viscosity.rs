use std::f32::consts::PI;

use uom::ConversionFactor;

use crate::kernel::Kernel;

use super::definition::KernelImpl;

#[derive(Debug, Clone, Copy)]
pub struct Viscosity {
    h: f32,
    volume: f32,
}

impl Viscosity {
    pub fn new(h: f32) -> Self {
        // let volume = 2. * PI * h.powi(3) / 15.;
        let volume = 2. * PI / 15.;
        Self { h, volume }
    }
}

impl KernelImpl for Viscosity {
    fn support_radius_impl(&self) -> f32 {
        self.h
    }

    fn function_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0, "value of r: {}", r);
        if r > self.h {
            return 0.;
        }
        let value = (self.h.powi(4) / 2. - self.h.powi(3) * r + self.h * r.powi(3)
            - r.powi(4) / 2.)
            / (self.h.powi(6) * r);
        value / self.volume
        // (-r.powi(3) / 2. * self.h.powi(3) + (r / self.h).powi(2) + (self.h / (2. * r)) - 1.)
        // / self.volume
    }

    fn gradient_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        let value = (-self.h.powi(4) + 4. * self.h * r.powi(3) - 3. * r.powi(4))
            / (2. * r.powi(2) * self.h.powi(6));
        value / self.volume
    }

    fn lapacian_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        let value =
            (1. / (self.h.powi(2) * r.powi(3)) + 2. / self.h.powi(5) - 3. * r / self.h.powi(6));
        value / self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests_helper;
    use super::*;
    use std::path::PathBuf;

    const FILE_PATH: &str = "equation/samples/viscosity.json";
    type TestKernel = Viscosity;

    #[test]
    fn verify_function() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(FILE_PATH);
        let data = tests_helper::TestData::new(&path);
        let values = data.get_function();
        let kernel = TestKernel::new(data.get_h());
        tests_helper::check_function(kernel, &values);
    }
    #[test]
    fn verify_gradient() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(FILE_PATH);
        let data = tests_helper::TestData::new(&path);
        let values = data.get_gradient();
        let kernel = TestKernel::new(data.get_h());
        tests_helper::check_gradient(kernel, &values);
    }
    #[test]
    fn verify_laplacian() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(FILE_PATH);
        let data = tests_helper::TestData::new(&path);
        let values = data.get_laplacian();
        let kernel = TestKernel::new(data.get_h());
        tests_helper::check_lapcian(kernel, &values);
    }
}
