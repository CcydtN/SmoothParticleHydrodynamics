extern crate uom;
mod util;

use std::f32::consts::PI;

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use uom::si::{acceleration, areal_mass_density, f32::*, length, mass};

use util::boundary::Boundary
use util::spike_kernel::{smoothing_kernel, smoothing_kernel_gradient}

const RADIUS: f32 = 0.01;
const THICKNESS: f32 = 0.05;
const KERNEL_RADIUS: f32 = 0.2;

fn calculate_density(displacements: &[Vec2], mass: f32) -> Vec<f32> {
    let n = displacements.len();
    let mut ret = vec![0.; n];
    for i in 0..n {
        for j in 0..n {
            let r = displacements[i].distance(displacements[j]);
            ret[i] += mass * smoothing_kernel(r, KERNEL_RADIUS);
        }
    }
    ret
}

fn calculate_pressures(densities: &[f32], rest_density: f32) -> Vec<f32> {
    // let k = 461.5 * (273.+25.); // J kg^-1 K^-1 * K
    // let k = 20.;
    let k = 0.05;
    densities.iter().map(|d| (d - rest_density) * k).collect()
}

fn calculate_pressures_force(
    pressures: &[f32],
    displacements: &[Vec2],
    densities: &[f32],
    mass: f32,
) -> Vec<Vec2> {
    let n = displacements.len();
    let mut ret = vec![Vec2::ZERO; n];
    let mut rng = thread_rng();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let r = displacements[i].distance(displacements[j]);
            let dir = if r != 0.0 {
                (displacements[j] - displacements[i]).normalize()
            } else {
                vec2(rng.gen(), rng.gen())
            };
            let slope = smoothing_kernel_gradient(r, KERNEL_RADIUS);
            let pressure = (pressures[i] + pressures[j]) / 2.;
            ret[i] += -dir * pressure * mass * slope / densities[j];
        }
    }
    ret
}

#[macroquad::main("pressure_2d")]
async fn main() {
    let mut rng = thread_rng();
    // Unit
    // Distance: m
    // Mass: kg
    // Acceleration: m*s^(-2)

    // camara setting
    let get_current_camera = || Camera2D {
        zoom: vec2(500. / screen_width(), -500. / screen_height()),
        ..Default::default()
    };

    // Important Constant
    // In 2D space, use ArealMassDensity(100) instead of MassDensity(1000)
    let density: ArealMassDensity =
        ArealMassDensity::new::<areal_mass_density::kilogram_per_square_meter>(100.);
    // let gravity: Acceleration = Acceleration::new::<acceleration::standard_gravity>(1.);

    let width: Length = Length::new::<length::centimeter>(200.);
    let height: Length = Length::new::<length::centimeter>(200.);
    let area: Area = width * height;
    let total_mass: Mass = density * area;

    let particle_count = 1000.;
    let particle_mass: Mass = total_mass / particle_count;
    let particle_mass = particle_mass.get::<mass::kilogram>();

    let width = width.get::<length::meter>();
    let height = height.get::<length::meter>();
    // let gravity = gravity.get::<acceleration::meter_per_second_squared>();
    let rest_density = density.get::<areal_mass_density::kilogram_per_square_meter>();

    let boundaries = [
        Boundary::new(vec2(0., -height * 0.6), vec2(0., 1.)),
        Boundary::new(vec2(width * 0.6, 0.), vec2(-1., 0.)),
        Boundary::new(vec2(-width * 0.6, 0.), vec2(1., 0.)),
        Boundary::new(vec2(0., height * 0.6), vec2(0., -1.)),
    ];

    let volume = width * height;
    let mass = rest_density * volume / (particle_count as f32);

    let mut displacements = vec![];
    let mut velocities = vec![];
    for _ in 0..particle_count as i32 {
        let init_position = vec2(0.0, 0.0);
        let init_velocity = vec2(0., 0.);
        displacements.push(init_position);
        velocities.push(init_velocity);
    }

    let mut dt = 1. / 30.;
    loop {
        // dynamic time step
        // dt = get_frame_time();
        let pred = displacements
            .iter()
            .zip(velocities.iter())
            .map(|(&s, &v)| s + v * dt)
            .collect::<Vec<_>>();
        let densities = calculate_density(&pred, particle_mass);
        let pressures = calculate_pressures(&densities, rest_density);
        let pressures_forces = calculate_pressures_force(&pressures, &pred, &densities, mass);
        let pressures_acceleration = pressures_forces
            .iter()
            .enumerate()
            .map(|(i, x)| *x / densities[i])
            .collect::<Vec<_>>();

        // Rendering
        {
            // clear_background(LIGHTGRAY);
            // clear_background(YELLOW);
            clear_background(BLACK);
            //
            let camera = get_current_camera();
            set_camera(&camera);

            boundaries.iter().for_each(|boundary| {
                boundary.draw(THICKNESS);
            });

            // Density visualize
            // for (disp, density) in displacements.iter().zip(densities.iter()) {
            //     let delta = density - rest_density;
            //     let delta_rate = (delta / rest_density).clamp(-1.0, 1.0);
            //     let color_to_vec3 = |c: Color| -> Vec3 { vec3(c.r, c.g, c.b) };

            //     let blue = color_to_vec3(BLUE);
            //     let red = color_to_vec3(RED);
            //     let white = color_to_vec3(WHITE);
            //     let color = white.lerp(if delta <= 0.0 { blue } else { red }, delta_rate.abs());
            //     let color = Color {
            //         r: color.x,
            //         g: color.y,
            //         b: color.z,
            //         a: 0.1,
            //     };

            //     draw_circle(disp.x, disp.y, RADIUS, color);
            //     draw_circle(disp.x, disp.y, 0.5 * (RADIUS + KERNEL_RADIUS), color);
            // }

            // Velocity visualize
            for (disp, vel) in displacements.iter().zip(velocities.iter()) {
                let max_render_v = 0.8;
                let v = vel.length().min(max_render_v) / max_render_v;
                let color_to_vec3 = |c: Color| -> Vec3 { vec3(c.r, c.g, c.b) };
                let blue = color_to_vec3(BLUE);
                let red = color_to_vec3(RED);
                let lime = color_to_vec3(LIME);
                let lerp1 = blue.lerp(lime, v);
                let lerp2 = lime.lerp(red, v);
                let color = lerp1.lerp(lerp2, v);
                let color = Color {
                    r: color.x,
                    g: color.y,
                    b: color.z,
                    a: 1.0,
                };
                draw_circle(disp.x, disp.y, RADIUS, color);
            }
        }

        for i in 0..displacements.len() {
            velocities[i] += pressures_acceleration[i] * dt;
            // velocities[i] += vec2(0.0, -1.0) * gravity * dt;
            displacements[i] += velocities[i] * dt;

            for boundary in &boundaries {
                let (p, v) = boundary.particle_collision(displacements[i], velocities[i], RADIUS);
                displacements[i] = p;
                velocities[i] = v;
            }
        }

        next_frame().await;
    }
}
