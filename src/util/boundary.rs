use macroquad::{
    color::{BLACK, LIGHTGRAY},
    math::Vec2,
    shapes::draw_line,
};

#[derive(Debug, Default)]
pub struct Boundary {
    offset: Vec2,
    normal: Vec2,
}

impl Boundary {
    pub fn new(offset: Vec2, normal: Vec2) -> Self {
        Self { offset, normal }
    }

    pub fn particle_collision(
        &self,
        mut displacement: Vec2,
        mut velocity: Vec2,
        radius: f32,
    ) -> (Vec2, Vec2) {
        let v = velocity.dot(self.normal);
        let p = (displacement - (self.offset + self.normal * radius)).dot(self.normal);

        if p < 0. && v < 0. {
            velocity -= 2. * v * self.normal;
            // Put the particle at border
            displacement -= p * self.normal;

            // Simple bounce but not practical, may have wired behavir when having high velocity
            // displacement -= 2. * p * self.normal;
        }
        (displacement, velocity)
    }

    pub fn draw(&self, thickness: f32) {
        let extend = self.normal.perp() * 10000.;
        let shift = -self.normal * thickness / 2.;
        let (p1, p2) = (self.offset + extend + shift, self.offset - extend + shift);
        draw_line(p1.x, p1.y, p2.x, p2.y, thickness, LIGHTGRAY);
    }
}
