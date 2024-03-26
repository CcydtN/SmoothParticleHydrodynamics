use macroquad::prelude::*;

#[derive(Debug, Default)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
}

const RADIUS: f32 = 1.0;
const THICKNESS: f32 = RADIUS;

impl Particle {
    fn new(position: Vec2, velocity: Vec2) -> Self {
        Self {
            position,
            velocity,
            mass: 1.,
        }
    }

    fn draw(&self) {
        draw_circle(self.position[0], self.position[1], RADIUS, BLUE);
    }

    fn update_velocity(&mut self, acceleration: Vec2, dt: f32) {
        self.velocity += acceleration * dt;
    }

    fn update_position(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}

#[derive(Debug, Default)]
struct Boundary {
    offset: Vec2,
    normal: Vec2,
}

impl Boundary {
    fn new(offset: Vec2, normal: Vec2) -> Self {
        Self { offset, normal }
    }

    fn collide(&self, mut particle: Particle) -> Particle {
        let v = particle.velocity.dot(self.normal);
        // offset shift by distance of (Radius + thickness/2) because of actual hitbox
        let p = (particle.position - (self.offset + self.normal * (RADIUS + THICKNESS / 2.)))
            .dot(self.normal);

        if p < 0. && v < 0. {
            particle.velocity -= 2. * v * self.normal;
            particle.position -= 2. * p * self.normal;
        }
        particle
    }

    fn draw(&self, camera: &Camera2D) {
        let extend = 2. * self.normal.perp() / camera.zoom;
        let (p1, p2) = (self.offset + extend, self.offset - extend);
        draw_line(p1.x, p1.y, p2.x, p2.y, THICKNESS, BLACK);
    }
}

#[macroquad::main("bouncing_particle")]
async fn main() {
    // camara setting
    let get_current_camera = || Camera2D {
        offset: vec2(0., -650. / screen_height()),
        zoom: vec2(30. / screen_width(), -30. / screen_height()),
        ..Default::default()
    };

    let mut particle = Particle::new(vec2(0., 20.), vec2(8., 0.));

    let mut dt = 1. / 30.;
    let g = vec2(0., -9.81);
    // let k = 0.47;
    let k = 0.9;

    let boundaries = [
        Boundary::new(vec2(0., 0.), vec2(0., 1.)),
        Boundary::new(vec2(20., 0.), vec2(-1., 0.)),
        Boundary::new(vec2(-20., 0.), vec2(1., 0.)),
    ];

    loop {
        // dynamic time step
        // dt = get_frame_time();

        clear_background(LIGHTGRAY);

        let acceleration: Vec2 = {
            let m = particle.mass;
            let v = particle.velocity;
            -(v * k / m) + g
        };

        particle.update_velocity(acceleration, dt);
        particle.update_position(dt);

        for boundary in &boundaries {
            particle = boundary.collide(particle);
        }

        // Rendering
        let camera = get_current_camera();
        set_camera(&camera);

        for boundary in &boundaries {
            boundary.draw(&camera);
        }
        particle.draw();

        next_frame().await;
    }
}
