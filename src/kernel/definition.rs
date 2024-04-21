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

    pub struct Value {
        r: f32,
        w: f32,
    }

    impl From<(f32, f32)> for Value {
        fn from((r, w): (f32, f32)) -> Self {
            Self { r, w }
        }
    }

    fn check(func: impl Fn(f32) -> f32, values: &[Value]) {
        values.iter().for_each(|Value { r, w }| {
            let ret = func(*r);
            let diff = (ret - w).abs();
            assert!(
                diff <= f32::EPSILON,
                "Value not match, r = {r}\nret = {ret}, w = {w}"
            );
        })
    }

    pub fn check_function(kernel: impl KernelImpl, values: &[Value]) {
        let wrapper = |r| kernel.function_impl(r);
        check(wrapper, values)
    }

    pub fn check_gradient(kernel: impl KernelImpl, values: &[Value]) {
        let wrapper = |r| kernel.gradient_impl(r);
        check(wrapper, values)
    }

    pub fn check_lapcian(kernel: impl KernelImpl, values: &[Value]) {
        let wrapper = |r| kernel.lapacian_impl(r);
        check(wrapper, values)
    }
}
