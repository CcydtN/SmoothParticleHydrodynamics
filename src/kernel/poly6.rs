use crate::kernel::definition::KernelImpl;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Poly6 {
    h: f32,
    volume: f32,
}

impl Poly6 {
    pub fn new(h: f32) -> Self {
        let volume = h.powi(9) * PI * 64. / 315.;
        Self { h, volume }
    }
}

impl KernelImpl for Poly6 {
    fn support_radius_impl(&self) -> f32 {
        self.h
    }

    fn function_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0, "value of r: {}", r);
        if r > self.h {
            return 0.;
        }
        (self.h.powi(2) - r.powi(2)).powi(3) / self.volume
    }

    fn gradient_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        -6. * r * (self.h.powi(2) - r.powi(2)).powi(2) / self.volume
    }

    fn lapacian_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        -6. * (self.h.powi(2) - 5. * r.powi(2)) * (self.h.powi(2) - r.powi(2)) / self.volume
    }
}

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;
    use super::super::tests_helper;
    use std::path::PathBuf;

    const FILE_PATH: &str = "equation/samples/poly6.json";
    type TestKernel = Poly6;

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
