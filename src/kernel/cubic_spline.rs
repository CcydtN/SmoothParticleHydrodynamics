use crate::kernel::definition::KernelImpl;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct CubicSpline {
    h: f32,
    constant: f32,
}

impl KernelImpl for CubicSpline {
    fn new(h: f32) -> Self {
        let volume = 4. * h.powi(3) * PI;
        Self {
            h,
            constant: 1. / volume,
        }
    }

    fn support_radius_impl(&self) -> f32 {
        2. * self.h
    }

    fn function_impl(&self, r: f32) -> f32 {
        debug_assert!(r >= 0.0, "value of r: {}", r);
        let value = match r {
            x if x <= self.h => {
                (-4. * (self.h - r).powi(3) + (2. * self.h - r).powi(3)) * self.h.powi(-3)
            }
            x if x <= 2. * self.h => (2. * self.h - r).powi(3) * self.h.powi(-3),
            _ => 0.,
        };
        value * self.constant
    }

    fn gradient_impl(&self, r: f32) -> f32 {
        debug_assert!(r >= 0.0, "value of r: {}", r);
        let value = if r <= self.h {
            3. * r * (-4. * self.h + 3. * r) * self.h.powi(-3)
        } else if r <= 2. * self.h {
            -3. * (2. * self.h - r).powi(2) * self.h.powi(-3)
        } else {
            0.
        };
        value * self.constant
    }

    fn laplacian_impl(&self, r: f32) -> f32 {
        debug_assert!(r >= 0.0, "value of r: {}", r);
        let value = match r {
            x if x <= self.h => 6. * (-2. * self.h + 3. * r) * self.h.powi(-3),
            x if x <= 2. * self.h => 6. * (2. * self.h - r) * self.h.powi(-3),
            _ => 0.,
        };
        value * self.constant
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests_helper;
    use super::*;
    use std::path::PathBuf;

    const FILE_PATH: &str = "equation/samples/cubic_spline.json";
    type TestKernel = CubicSpline;

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
