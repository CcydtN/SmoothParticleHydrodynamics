use std::iter::Sum;

use macroquad::math::Vec3;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub density: f32,
    pub pressure: f32,
}

impl Particle {
    pub fn new(position: Vec3, velocity: Vec3, mass: f32) -> Self {
        Self {
            position,
            velocity,
            mass,
            ..Default::default()
        }
    }
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

impl From<(Vec3, Vec3, f32)> for Particle {
    fn from(value: (Vec3, Vec3, f32)) -> Self {
        Self {
            position: value.0,
            velocity: value.1,
            mass: value.2,
            ..Default::default()
        }
    }
}
impl From<((Vec3, Vec3), f32)> for Particle {
    fn from(value: ((Vec3, Vec3), f32)) -> Self {
        Self {
            position: value.0 .0,
            velocity: value.0 .1,
            mass: value.1,
            ..Default::default()
        }
    }
}
