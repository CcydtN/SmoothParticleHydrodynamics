use std::f32::consts::PI;

use super::definition::KernelImpl;

#[derive(Debug, Clone, Copy)]
pub struct Spiky {
    h: f32,
    volume: f32,
}

impl Spiky {
    pub fn new(h: f32) -> Self {
        let volume = h.powi(6) * PI / 15.;
        Self { h, volume }
    }
}

impl KernelImpl for Spiky {
    fn support_radius_impl(&self) -> f32 {
        self.h
    }

    fn function_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0, "value of r: {}", r);
        if r > self.h {
            return 0.;
        }
        (self.h - r).powi(3) / self.volume
    }

    fn gradient_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        -3. * (self.h - r).powi(2) / self.volume
    }

    fn lapacian_impl(&self, r: f32) -> f32 {
        assert!(r >= 0.0);
        if r > self.h {
            return 0.;
        }
        6. * (self.h - r) / self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests_helper;
    use super::*;
    use std::path::PathBuf;

    const FILE_PATH: &str = "equation/samples/spiky.json";
    type TestKernel = Spiky;

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
