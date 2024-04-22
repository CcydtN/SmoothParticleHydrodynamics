use super::definition::KernelImpl;
use core::panic;
use itertools::Itertools;
use serde::Deserialize;
use serde_json;
use std::{fmt::Debug, fs, path::PathBuf};

#[derive(Deserialize, Clone)]
pub(crate) struct Value {
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

#[macro_export]
macro_rules! tests_impl {
    () => {};
}
