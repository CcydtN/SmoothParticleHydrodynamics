use macroquad::math::Vec3;

#[derive(Debug, Default)]
pub struct Particle {
    position: Vec3,
    velocity: Vec3,
    density: f32,
}
