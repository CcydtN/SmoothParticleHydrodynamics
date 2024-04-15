extern crate uom;
mod util_2d;

use crate::util_2d::{
    boundary::Boundary, camera_control::get_camera, neighborhood_search::HashGrid,
};

use macroquad::{
    camera::set_camera,
    color::*,
    input,
    math::{vec2, vec3, Vec2, Vec3},
    rand,
    shapes::draw_circle,
    ui::root_ui,
    window::{clear_background, next_frame},
};

use uom::si::{
    areal_mass_density, diffusion_coefficient::square_centimeter_per_second, dynamic_viscosity,
    f32::*, length, mass,
};
use util_2d::spike_kernel::{
    smoothing_kernel, smoothing_kernel_gradient, smoothing_kernel_laplacian,
};

// Showing at least n unit on screen anytime
// 1.0 mean it will show -1/+1
const DISPLAY_UNIT: f32 = 7.0;

const RADIUS: f32 = 0.02;
const THICKNESS: f32 = 0.05;

fn calculate_density(
    position: &[Vec2],
    mass: f32,
    lookup_grid: &HashGrid,
    kernel_radius: f32,
) -> Vec<f32> {
    let get_density = |(i, target)| {
        lookup_grid
            .lookup(target)
            .iter()
            .map(|&j| {
                let r = target.distance(position[j]);
                mass * smoothing_kernel(r, kernel_radius)
            })
            .sum()
    };
    position.iter().enumerate().map(get_density).collect()
}

fn calculate_pressures(density: &[f32], rest_density: f32, multiplier: f32) -> Vec<f32> {
    density
        .iter()
        .map(|d| (d - rest_density) * multiplier)
        .collect()
}

fn calculate_pressures_force(
    pressure: &[f32],
    position: &[Vec2],
    density: &[f32],
    mass: f32,
    lookup_grid: &HashGrid,
    kernel_radius: f32,
) -> Vec<Vec2> {
    let get_pressure_force = |(i, target)| {
        lookup_grid
            .lookup(target)
            .iter()
            .fold(Vec2::ZERO, |acc, &j| {
                let r = target.distance(position[j]);
                if r == 0.0 {
                    return acc;
                }
                let dir = (position[j] - *target).normalize();
                let slope = smoothing_kernel_gradient(r, kernel_radius);
                let pressure = (pressure[i] + pressure[j]) / 2.;
                acc + dir * pressure * mass * slope / density[j]
            })
    };
    position
        .iter()
        .enumerate()
        .map(get_pressure_force)
        .collect()
}

fn calculate_viscosity_force(
    position: &[Vec2],
    velocity: &[Vec2],
    density: &[f32],
    mass: f32,
    lookup_grid: &HashGrid,
    viscosity_multiplier: f32,
    kernel_radius: f32,
) -> Vec<Vec2> {
    let get_viscosity = |(i, target)| {
        lookup_grid
            .lookup(target)
            .iter()
            .fold(Vec2::ZERO, |acc, &j| {
                let r = target.distance(position[j]);
                if r == 0.0 {
                    return acc;
                }
                // let dir = (position[j] - *target).normalize();
                let laplacian = smoothing_kernel_laplacian(r, kernel_radius);
                let velocity_diff = velocity[j] - velocity[i];
                acc + viscosity_multiplier * velocity_diff * mass * laplacian / density[j]
            })
    };
    position.iter().enumerate().map(get_viscosity).collect()
}

#[macroquad::main("simulation")]
async fn main() {
    let kernel_radius = 0.2;
    let particle_count = 3000.;

    let density: ArealMassDensity =
        ArealMassDensity::new::<areal_mass_density::kilogram_per_square_meter>(100.);
    let width: Length = Length::new::<length::centimeter>(600.);
    let height: Length = Length::new::<length::centimeter>(600.);
    let viscosity: DynamicViscosity = DynamicViscosity::new::<dynamic_viscosity::centipoise>(1.);
    let area: Area = width * height;
    let total_mass: Mass = density * area;
    let particle_mass: Mass = total_mass / particle_count;

    let width = width.get::<length::meter>();
    let height = height.get::<length::meter>();
    let particle_mass = particle_mass.get::<mass::kilogram>();
    let rest_density = density.get::<areal_mass_density::kilogram_per_square_meter>();
    let viscosity = viscosity.get::<dynamic_viscosity::poise>();
    println!("width: +/- {} unit.", width / 2.);
    println!("height: +/- {} unit.", height / 2.);

    let random_pos = || {
        vec2(
            rand::gen_range(-0.5 * width, 0.5 * width),
            rand::gen_range(-0.5 * height, 0.5 * height),
        )
    };
    let mut position = (0..particle_count as usize)
        // .map(|_| random_pos())
        .map(|i| vec2(i as f32 / 100. * 0.05, i as f32 % 100. * 0.05))
        .collect::<Vec<_>>();
    let mut velocity = vec![vec2(0.0, 0.0); particle_count as usize];
    let boundaries = [
        Boundary::new(vec2(0., -height), vec2(0., 1.)),
        Boundary::new(vec2(width, 0.), vec2(-1., 0.)),
        Boundary::new(vec2(-width, 0.), vec2(1., 0.)),
        Boundary::new(vec2(0., height), vec2(0., -1.)),
    ];

    let mut grid = HashGrid::new(kernel_radius);

    let dt = 1. / 60.;
    let mut gravity = 9.81;
    let mut pressure_multiplier = 50.;
    let mut viscosity_multiplier = 20.;
    loop {
        // dt = macroquad::time::get_frame_time();
        // Prep
        root_ui().slider(0, "Gravity", 0.0..10.0, &mut gravity);
        root_ui().slider(1, "Pressure", -50.0..50.0, &mut pressure_multiplier);
        root_ui().slider(2, "Viscosity", -50.0..50.0, &mut viscosity_multiplier);

        clear_background(DARKGRAY);
        let camera = get_camera(DISPLAY_UNIT);
        set_camera(&camera);
        let mouse_screen = Vec2::from_array(input::mouse_position().into());
        let mouse = camera.screen_to_world(mouse_screen);

        // Update value
        let pred = position
            .iter()
            .zip(velocity.iter())
            .map(|(&a, &b)| a + b * dt)
            .collect::<Vec<_>>();
        grid.update(&pred);
        let density = calculate_density(&pred, particle_mass, &grid, kernel_radius);
        let pressure = calculate_pressures(&density, rest_density, pressure_multiplier);
        let pressure_force = calculate_pressures_force(
            &pressure,
            &pred,
            &density,
            particle_mass,
            &grid,
            kernel_radius,
        );
        let pressure_accelration = pressure_force
            .iter()
            .zip(density.iter())
            .map(|(a, b)| *a / *b)
            .collect::<Vec<_>>();
        let viscosity_force = calculate_viscosity_force(
            &pred,
            &velocity,
            &density,
            particle_mass,
            &grid,
            viscosity_multiplier,
            kernel_radius,
        );
        let viscosity_accelration = viscosity_force
            .iter()
            .zip(density.iter())
            .map(|(a, b)| *a / *b)
            .collect::<Vec<_>>();

        velocity
            .iter_mut()
            .for_each(|x| *x += vec2(0., -gravity) * dt);
        velocity
            .iter_mut()
            .zip(pressure_accelration.iter())
            .for_each(|(x, p)| *x += *p * dt);
        velocity
            .iter_mut()
            .zip(viscosity_accelration.iter())
            .for_each(|(x, p)| *x += *p * dt);
        position
            .iter_mut()
            .zip(velocity.iter())
            .for_each(|(s, v)| *s += *v * dt);

        for (pos, vel) in position.iter_mut().zip(velocity.iter_mut()) {
            boundaries.iter().for_each(|b| {
                (*pos, *vel) = b.particle_collision(*pos, *vel, RADIUS);
            });
        }

        // draw
        {
            // draw_circle(mouse.x, mouse.y, RADIUS, YELLOW);
            // draw_circle(mouse.x, mouse.y, kernel_radius, Color { a: 0.2, ..WHITE });

            // let nei = grid.lookup(&mouse);
            // position.iter().enumerate().for_each(|(i, pos)| {
            // let color = if nei.contains(&i) { MAROON } else { SKYBLUE };
            // draw_circle(pos.x, pos.y, RADIUS, color);
            // });
            let max_render_v = 10.;
            let color_to_vec3 = |c: Color| -> Vec3 { vec3(c.r, c.g, c.b) };
            let blue = color_to_vec3(SKYBLUE);
            let yellow = color_to_vec3(YELLOW);
            let red = color_to_vec3(RED);
            position.iter().zip(velocity.iter()).for_each(|(pos, vel)| {
                let v = vel.length().min(max_render_v) / max_render_v;
                let lerp1 = blue.lerp(yellow, v);
                let lerp2 = yellow.lerp(red, v);
                let color = lerp1.lerp(lerp2, v);
                let color = Color {
                    r: color.x,
                    g: color.y,
                    b: color.z,
                    a: 1.0,
                };
                draw_circle(pos.x, pos.y, RADIUS, color);
            });

            boundaries.iter().for_each(|x| x.draw(THICKNESS));
        }

        next_frame().await;
    }
}
