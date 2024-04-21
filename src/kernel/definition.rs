use macroquad::math::Vec3;

pub trait Kernel {
    fn function_scaler(&self, r: f32) -> f32;

    fn gradient_scaler(&self, r: f32) -> f32;

    fn lapacian_scaler(&self, r: f32) -> f32;

    fn function(&self, r: Vec3) -> f32 {
        self.function_scaler(r.length())
    }

    fn gradient(&self, r: Vec3) -> Vec3 {
        self.gradient_scaler(r.length()) * r
    }

    fn lapacian(&self, r: Vec3) -> Vec3 {
        self.lapacian_scaler(r.length()) * r
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub struct Value {
        r: f32,
        w: f32,
    }

    impl From<(f32, f32)> for Value {
        fn from((r, w): (f32, f32)) -> Self {
            Self { r, w }
        }
    }

    pub fn check_function(kernel: impl Kernel, values: &[Value]) {
        values.iter().for_each(|Value { r, w }| {
            let ret = kernel.function_scaler(*r);
            let diff = (ret - w).abs();
            assert!(
                diff <= f32::EPSILON,
                "Value not match, ret = {ret}, w = {w}"
            );
        })
    }
}
