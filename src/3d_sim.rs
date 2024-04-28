mod kernel;
mod model;
mod util_3d;

use itertools::{izip, Itertools};
use macroquad::{color::Color, prelude::*};
use model::pressure;
use rayon::prelude::*;
use std::{f32::consts::PI, iter};
use uom::si::{
    acceleration,
    f32::{Acceleration, MassDensity},
    mass_density,
};
use util_3d::*;

use crate::{
    kernel::Kernel,
    model::{density, surface_tension, viscosity},
};

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
    let particle_per_side = 13isize;

    let particle_count = particle_per_side.pow(3);
    let total_mass = mass * particle_count as f32;
    let spacing = (total_mass / rest_density).powf(1. / 3.) / particle_per_side as f32;

    // nice to have around 25-80 particle in the radius, which is between [3,4) (27 - 64 in count)
    let default_kernel_radius = 1.3 * (mass / rest_density).powf(1. / 3.);

    dbg!(rest_density, total_mass, spacing, default_kernel_radius);
    let particles = init_setup::create_cube(
        spacing,
        particle_per_side,
        Vec3::ZERO,
        mass,
        default_kernel_radius,
    );
    // let particles = init_setup::create_sphere(
    //     mass,
    //     spacing * particle_per_side as f32 / 2.,
    //     particle_count as usize,
    //     Vec3::ZERO,
    // );

    let cubic_spline = kernel::CubicSpline::new(default_kernel_radius);
    let speed_of_sound = f32::sqrt(200. * gravity * spacing * particle_per_side as f32 / 2.);

    let density_model = density::Density::<kernel::CubicSpline>::new();
    let pressure_model =
        pressure::Tait::<kernel::CubicSpline>::new(rest_density, 7, speed_of_sound);
    let viscosity_model = viscosity::Artificial::new(cubic_spline, mass, speed_of_sound);
    let surface_tension_model = surface_tension::BeakerTeschner07::new(cubic_spline, mass);

    let mut space = Space::new(default_kernel_radius, particles);

    // update_density(mass, &mut grid, cubic_spline);
    // dbg!(grid
    //     .particles()
    //     .take((particle_count / 10).try_into().unwrap())
    //     .map(|p| p.density)
    //     .collect_vec());
    // return;

    let time_step = 0.4 * default_kernel_radius / (speed_of_sound * (1. + 0.6 * 0.1));
    let mut t: f32 = 0.;

    let frame_period = ((1. / 30.) * 1000.) as u128;
    let mut next_render = std::time::Instant::now();
    let mut render_var = 0.;

    dbg!(time_step);
    loop {
        dbg!(t);

        density_model.update_density(&mut space);
        pressure_model.update_pressure(&mut space);

        let pressure_acc = pressure_model.accelration(&space);
        let viscosity_acc = viscosity_model.accelration(&space, default_kernel_radius);
        let surface_tension_acc = surface_tension_model.accelration(&space);

        let acceleration = izip!(pressure_acc, viscosity_acc, surface_tension_acc)
            // .par_bridge()
            .map(|t| t.0 + t.1 + t.2);

        space.particles_mut().zip(acceleration).for_each(|(p, a)| {
            p.velocity += a * time_step / 2.;
            p.position += p.velocity * time_step;
            p.velocity += a * time_step / 2.;
        });

        if next_render.elapsed().as_millis() >= frame_period {
            next_render = rendering(spacing, particle_per_side, &space, &mut render_var).await;
        }

        t += time_step;
        space.update();
    }
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
