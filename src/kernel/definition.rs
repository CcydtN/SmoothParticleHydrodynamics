use macroquad::math::Vec3;
use std::fmt::Debug;

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

impl<T: KernelImpl + Debug> Kernel for T {
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
