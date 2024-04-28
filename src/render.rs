use std::f32::consts::PI;

use crate::Space;
use macroquad::prelude::*;

pub struct Render {
    current_angle: f32,
    angle_step: f32,
}

impl Render {
    pub fn new() -> Self {
        Self {
            current_angle: 0.,
            angle_step: 2. * PI / 360. / 2.,
        }
    }

    fn draw_anchor(&self) {
        draw_line_3d(Vec3::ZERO, Vec3::X, RED);
        draw_line_3d(Vec3::ZERO, Vec3::Y, GREEN);
        draw_line_3d(Vec3::ZERO, Vec3::Z, BLUE);
    }

    pub async fn render_distance_from_zero(
        &mut self,
        space: &Space,
        distance: f32,
    ) -> std::time::Instant {
        clear_background(WHITE);
        // camera setting
        let pos = vec3(self.current_angle.cos(), self.current_angle.sin(), 0.5);
        set_camera(&Camera3D {
            position: pos * distance * 2.,
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        // drawing
        let lerp = |a: Color, b: Color, c: Color, t: f32| {
            let inv_t = 1.0 - t;
            Color {
                r: ((a.r * inv_t.powi(2)) + (2. * b.r * t * inv_t) + (c.r * t.powi(2))),
                g: ((a.g * inv_t.powi(2)) + (2. * b.g * t * inv_t) + (c.g * t.powi(2))),
                b: ((a.b * inv_t.powi(2)) + (2. * b.b * t * inv_t) + (c.b * t.powi(2))),
                a: 1.0,
            }
        };
        self.draw_anchor();
        space.particles().for_each(|particle| {
            let t = (particle.position.length() / distance).clamp(0., 1.);
            let color = lerp(LIME, YELLOW, ORANGE, t);
            // draw_sphere_wires(particle.position, spacing / 8., None, color);
            draw_sphere(particle.position, particle.kernel_radius / 8., None, color);
        });

        next_frame().await;
        self.current_angle += self.angle_step;
        self.current_angle %= 2. * PI;
        std::time::Instant::now()
    }
}
