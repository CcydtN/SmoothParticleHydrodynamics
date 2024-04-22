use macroquad::math::Vec3;

pub trait KernelImpl {
    fn support_radius_impl(&self) -> f32;
    fn function_impl(&self, r: f32) -> f32;
    fn gradient_impl(&self, r: f32) -> f32;
    fn lapacian_impl(&self, r: f32) -> f32;
}

pub trait Kernel {
    fn support_radius(&self) -> f32;
    fn function(&self, r: Vec3) -> f32;
    fn gradient(&self, r: Vec3) -> Vec3;
    fn lapacian(&self, r: Vec3) -> Vec3;
}

impl<T> Kernel for T
where
    T: KernelImpl,
{
    fn support_radius(&self) -> f32 {
        self.support_radius_impl()
    }

    fn function(&self, r: Vec3) -> f32 {
        self.function_impl(r.length())
    }

    fn gradient(&self, r: Vec3) -> Vec3 {
        let scaler = self.gradient_impl(r.length());
        let grad = r * scaler;
        // debug_assert!(grad.is_nan(), "r is {}, length is {}", r, r.length());
        if grad.is_nan() {
            Vec3::ZERO
        } else {
            grad
        }
    }

    fn lapacian(&self, r: Vec3) -> Vec3 {
        self.lapacian_impl(r.length()) * r
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use core::panic;
    use itertools::Itertools;
    use serde::Deserialize;
    use serde_json;
    use std::{fmt::Debug, fs, path::PathBuf};

    #[derive(Deserialize, Clone)]
    pub struct Value {
        r: f32,
        w: f32,
    }

    impl From<(f32, f32)> for Value {
        fn from(value: (f32, f32)) -> Self {
            Self {
                r: value.0,
                w: value.1,
            }
        }
    }

    impl From<Vec<f64>> for Value {
        fn from(value: Vec<f64>) -> Self {
            assert_eq!(value.len(), 2);
            let r = value[0] as f32;
            let w = value[1] as f32;
            assert!(r.is_finite());
            assert!(w.is_finite());
            Self { r, w }
        }
    }

    #[derive(Deserialize)]
    pub struct TestData {
        h_value: f64,
        function: Vec<Vec<f64>>,
        gradient: Vec<Vec<f64>>,
        laplacian: Vec<Vec<f64>>,
    }

    macro_rules! value_impl {
        ($member_function_name:ident, $member:ident) => {
            pub fn $member_function_name(&self) -> Vec<Value> {
                self.$member
                    .iter()
                    .map(|v| Into::into(v.clone()))
                    .collect_vec()
            }
        };
    }

    impl TestData {
        pub fn new(file_path: &PathBuf) -> Self {
            match fs::read_to_string(file_path) {
                Ok(text) => serde_json::from_str(&text).unwrap(),
                Err(_) => panic!("Uable locate the file, {:?}", file_path),
            }
        }

        pub fn get_h(&self) -> f32 {
            let ret = self.h_value as f32;
            assert!(ret.is_finite());
            ret
        }

        value_impl!(get_function, function);
        value_impl!(get_gradient, gradient);
        value_impl!(get_laplacian, laplacian);
    }

    macro_rules! check_impl {
        ( $function_name:ident, $call_function:ident ) => {
            pub fn $function_name(kernel: impl KernelImpl, values: &[Value]) {
                values.iter().for_each(|Value { r, w }| {
                    let ret = kernel.$call_function(*r);
                    let diff = (ret - w).abs();
                    assert!(
                        diff <= f32::EPSILON,
                        "Value not match, r = {r}\nret = {ret}, w = {w}"
                    );
                })
            }
        };
    }

    check_impl!(check_function, function_impl);
    check_impl!(check_gradient, gradient_impl);
    check_impl!(check_lapcian, lapacian_impl);
}
