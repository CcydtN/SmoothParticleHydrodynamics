mod kernel;
mod model;
mod util_3d;

use std::f32::consts::PI;

use itertools::{iproduct, izip, Itertools};
use macroquad::{color::Color, prelude::*};
use model::{density::Density, pressure, surface_tension, viscosity};
use uom::si::{
    acceleration,
    f32::{Acceleration, MassDensity},
    mass_density,
};
use util_3d::*;

struct Material {
    density: MassDensity,
}

impl Material {
    fn get_density(&self) -> f32 {
        self.density
            .get::<mass_density::gram_per_cubic_centimeter>()
    }
}

#[macroquad::main("simulation")]
async fn main() {
    // Constant for water
    let water = Material {
        density: MassDensity::new::<mass_density::kilogram_per_cubic_meter>(1000.),
    };

    let rest_density = water.get_density();
    let gravity: Acceleration = Acceleration::new::<acceleration::standard_gravity>(1.);
    let gravity = gravity.get::<acceleration::centimeter_per_second_squared>();

    let mass = 1.; // gram
    let particle_per_side = 12isize;

    let particle_count = particle_per_side.pow(3); // total 1000;
    let total_mass = mass * particle_count as f32;
    let spacing = (total_mass / rest_density).powf(1. / 3.) / particle_per_side as f32;

    // nice to have around 25-80 particle in the radius, which is between [3,4) (27 - 64 in count)
    let kernel_radius = 27f32.powf(1. / 3.) * spacing;

    dbg!(rest_density, total_mass, spacing, kernel_radius);
    let particles = init_setup::create_cube(spacing, particle_per_side, Vec3::ZERO);

    let cubic_spline = kernel::CubicSpline::new(kernel_radius);
    let speed_of_sound = f32::sqrt(200. * gravity * spacing * particle_per_side as f32 / 2.);

    let mut grid = Space::new(kernel_radius, particles);

    // update_density(mass, &mut grid, cubic_spline);
    // dbg!(grid
    //     .particles()
    //     .take((particle_count / 10).try_into().unwrap())
    //     .map(|p| p.density)
    //     .collect_vec());
    // return;

    let time_step = 0.4 * kernel_radius / (speed_of_sound * (1. + 0.6 * 0.1));
    let mut t: f32 = 0.;

    let frame_period = ((1. / 30.) * 1000.) as u128;
    let mut next_render = std::time::Instant::now();
    let mut render_var = 0.;

    dbg!(time_step);
    loop {
        dbg!(t);

        update_density(mass, &mut grid, cubic_spline);
        let acceleration = grid
            .particles_with_neighbour(kernel_radius)
            .map(|_| Vec3::ZERO)
            .collect_vec();

        grid.particles_mut().zip(acceleration).for_each(|(p, a)| {
            p.velocity += a * time_step / 2.;
            p.position += p.velocity * time_step;
            p.velocity += a * time_step / 2.;
        });

        if next_render.elapsed().as_millis() >= frame_period {
            next_render = rendering(spacing, particle_per_side, &grid, &mut render_var).await;
        }

        t += time_step;
        grid.update();
    }
}

fn update_density(mass: f32, space: &mut Space, kernel: impl kernel::Kernel) {
    let density = space
        .particles_with_neighbour(kernel.support_radius())
        .map(|(a, others)| {
            others
                .map(|b| {
                    let r = a.position - b.position;
                    mass * kernel.function(r)
                })
                .sum::<f32>()
        })
        .collect_vec();

    space
        .particles_mut()
        .zip(density)
        .for_each(|(particle, d)| particle.density = d);
}

fn compute_accelration(mass: f32, space: &mut Space, kernel: impl kernel::Kernel) -> Vec<Vec3> {
    let mut acceleration = vec![];
    acceleration.reserve_exact(space.count());
    for (a, others) in space.particles_with_neighbour(kernel.support_radius()) {
        for b in others {}
    }
    acceleration
}

async fn rendering(
    spacing: f32,
    particle_per_side: isize,
    space: &Space,
    render_var: &mut f32,
) -> std::time::Instant {
    clear_background(WHITE);
    // camera setting
    let base_dist = spacing * particle_per_side as f32;
    let pos = vec3(render_var.cos(), render_var.sin(), 0.5);
    *render_var += 2. * PI / 360. / 2.;
    *render_var %= 6. * PI;
    set_camera(&Camera3D {
        position: pos * 2. * base_dist,
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
    draw_line_3d(Vec3::ZERO, Vec3::X, RED);
    draw_line_3d(Vec3::ZERO, Vec3::Y, GREEN);
    draw_line_3d(Vec3::ZERO, Vec3::Z, BLUE);
    for particle in space.particles() {
        let t = (particle.position.length() / base_dist).clamp(0., 1.);
        let color = lerp(LIME, YELLOW, ORANGE, t);
        // draw_sphere_wires(particle.position, spacing / 8., None, color);
        draw_sphere(particle.position, spacing / 8., None, color);
    }
    next_frame().await;
    std::time::Instant::now()
}
