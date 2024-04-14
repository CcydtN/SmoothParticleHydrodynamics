pub trait Kernel {
    fn function(&self, r: f32) -> f32;

    fn gradient(&self, r: f32) -> f32;

    fn lapacian(&self, r: f32) -> f32;
}
