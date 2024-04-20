use macroquad::math::Vec3;

pub trait Kernel {
    fn function_scaler(&self, r: f32) -> f32;

    fn gradient_scaler(&self, r: f32) -> f32;

    fn lapacian_scaler(&self, r: f32) -> f32;

    fn function(&self, r: Vec3) -> f32 {
        self.function_scaler(r.length())
    }

    fn gradient(&self, r: Vec3) -> Vec3 {
        self.gradient_scaler(r.length()) * r.normalize()
    }

    fn lapacian(&self, r: Vec3) -> Vec3 {
        self.lapacian_scaler(r.length()) * r.normalize()
    }
}
