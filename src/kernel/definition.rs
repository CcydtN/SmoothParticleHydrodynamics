use macroquad::math::Vec3;

pub trait KernelImpl {
    fn new(h: f32) -> Self;
    fn support_radius_impl(&self) -> f32;
    fn function_impl(&self, r: f32) -> f32;
    fn gradient_impl(&self, r: f32) -> f32;
    fn laplacian_impl(&self, r: f32) -> f32;
}

pub trait Kernel {
    fn new(h: f32) -> Self;
    fn support_radius(&self) -> f32;
    fn function(&self, r: Vec3) -> f32;
    fn gradient(&self, r: Vec3) -> Vec3;
    fn laplacian(&self, r: Vec3) -> Vec3;
}

impl<T: KernelImpl> Kernel for T {
    fn new(h: f32) -> Self {
        T::new(h)
    }

    fn support_radius(&self) -> f32 {
        self.support_radius_impl()
    }

    fn function(&self, r: Vec3) -> f32 {
        self.function_impl(r.length())
    }

    fn gradient(&self, r: Vec3) -> Vec3 {
        if r.length() == 0.0 {
            return Vec3::ZERO;
        }
        r * self.gradient_impl(r.length())
    }

    fn laplacian(&self, r: Vec3) -> Vec3 {
        if r.length() == 0.0 {
            return Vec3::ZERO;
        }
        r * self.laplacian_impl(r.length())
    }
}
