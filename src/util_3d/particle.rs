use macroquad::math::Vec3;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub density: f32,
}

impl From<Vec3> for Particle {
    fn from(value: Vec3) -> Self {
        Self {
            position: value,
            ..Default::default()
        }
    }
}

impl From<(Vec3, Vec3)> for Particle {
    fn from(value: (Vec3, Vec3)) -> Self {
        Self {
            position: value.0,
            velocity: value.1,
            ..Default::default()
        }
    }
}
