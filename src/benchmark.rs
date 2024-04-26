mod kernel;
mod model;
mod util_3d;

use itertools::{iproduct, izip};
use macroquad::{color::Color, prelude::*};
use model::{density::Density, pressure, surface_tension, viscosity};
use uom::si::{f32::MassDensity, mass_density};
use util_3d::*;

struct Material {
    density: MassDensity,
    viscosity: f32,
    surface_tension: f32,
}

impl Material {
    fn get_density(&self) -> f32 {
        self.density
            .get::<mass_density::gram_per_cubic_centimeter>()
    }

    fn get_viscosity(&self) -> f32 {
        self.viscosity
    }

    fn get_surface_tension(&self) -> f32 {
        self.surface_tension
    }
}

fn main() {
    // Constant for water
    let water = Material {
        density: MassDensity::new::<mass_density::kilogram_per_cubic_meter>(1000.),
        viscosity: 100.,
        surface_tension: 75.,
    };

    let rest_density = water.get_density();
    let pressure_constant = 100.;
    let viscosity_constant = water.get_viscosity();
    let surface_tension_coefficient = water.get_surface_tension();
    let gravity = 9.81 * 10.;

    let mass = 1.;
    let particle_per_side = 12i32;
    let particle_count = particle_per_side.pow(3); // total 1000;
    let total_mass = mass * particle_count as f32;
    let spacing = (total_mass / rest_density).powf(1. / 3.) / particle_per_side as f32;

    // nice to have around 25-80 particle in the radius, which is between [3,4) (27 - 64 in count)
    let kernel_radius = 27f32.powf(1. / 3.) * spacing;

    dbg!(total_mass, spacing, kernel_radius);

    let mut position = vec![];
    let mut velocity = vec![Vec3::ZERO; particle_count as usize];
    let offset = particle_per_side / 2;
    for (i, j, k) in iproduct!(-offset..offset, -offset..offset, -offset..offset) {
        position.push(
            vec3(
                spacing * (i as f32 + 0.5 * ((particle_per_side + 1) % 2) as f32),
                spacing * (j as f32 + 0.5 * ((particle_per_side + 1) % 2) as f32),
                spacing * (k as f32 + 0.5 * ((particle_per_side + 1) % 2) as f32),
            ) * 1.0,
        );
    }

    let poly6_kernel = kernel::Poly6::new(kernel_radius);
    let spiky_kernel = kernel::Spiky::new(kernel_radius);
    let viscosity_kernel = kernel::Viscosity::new(kernel_radius);
    let cubic_spline = kernel::CubicSpline::new(kernel_radius);
    let speed_of_sound = f32::sqrt(200. * gravity * spacing * particle_per_side as f32 / 2.);

    // let density_model = Density::new(poly6_kernel, mass);
    // let pressure_model = pressure::Simple::new(spiky_kernel, mass, pressure_constant, rest_density);
    // let viscoity_model = viscosity::Simple::new(viscosity_kernel, mass, viscosity_constant);
    // let surface_tension_model =
    // surface_tension::SurfaceTension::new(poly6_kernel, surface_tension_coefficient, mass);

    let density_model = Density::new(cubic_spline, mass);
    let pressure_model = pressure::Tait::new(cubic_spline, mass, rest_density, 7, speed_of_sound);
    let viscoity_model = viscosity::Artificial::new(cubic_spline, mass, speed_of_sound);
    let surface_tension_model = surface_tension::BeakerTeschner07::new(cubic_spline, mass);

    let mut grid = SpatialHashGrid::new(kernel_radius);

    // grid.update(&position);
    // let density = density_model.compute(&grid, &position);
    // dbg!(density);
    // return;

    // let time_step = 1. / 10000.;
    // let time_step = 1. / 1000.;
    // let time_step = 1. / 100.;
    let time_step = 0.4 * kernel_radius / (speed_of_sound * (1. + 0.6 * 0.1));
    let mut t: f32 = 0.;

    let mut next_render = std::time::Instant::now();
    let frame_period = ((1. / 30.) * 1000.) as u128;

    dbg!(time_step);
    while t <= 2. {
        dbg!(t);
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let pressure_acc = pressure_model.compute_accelraction(&grid, &position, &density);
        let viscosity_acc = viscoity_model.compute_accelration(
            &grid,
            &position,
            &velocity,
            &density,
            kernel_radius,
        );
        let surface_tension_acc =
            surface_tension_model.compute_accelration(&grid, &position, &density);

        let count = position.len();
        debug_assert_eq!(velocity.len(), count);
        debug_assert_eq!(density.len(), count);
        debug_assert_eq!(pressure_acc.len(), count);
        debug_assert_eq!(viscosity_acc.len(), count);
        debug_assert_eq!(surface_tension_acc.len(), count);

        let acceleration =
            izip!(pressure_acc, viscosity_acc, surface_tension_acc).map(|t| t.0 + t.1 + t.2);

        izip!(position.iter_mut(), velocity.iter_mut(), acceleration).for_each(|(d, v, a)| {
            *v += a * time_step / 2.;
            *d += *v * time_step;
            *v += a * time_step / 2.;
        });

        t += time_step;
    }
}

async fn rendering(
    spacing: f32,
    particle_per_side: i32,
    t: f32,
    position: &Vec<Vec3>,
) -> std::time::Instant {
    clear_background(WHITE);
    let base_dist = spacing * particle_per_side as f32;
    let pos = vec3((t).cos(), (t / 3.).sin(), (t / 2.).cos()).normalize();

    let lerp = |a: Color, b: Color, c: Color, t: f32| {
        let inv_t = 1.0 - t;
        Color {
            r: ((a.r * inv_t.powi(2)) + (2. * b.r * t * inv_t) + (c.r * t.powi(2))),
            g: ((a.g * inv_t.powi(2)) + (2. * b.g * t * inv_t) + (c.g * t.powi(2))),
            b: ((a.b * inv_t.powi(2)) + (2. * b.b * t * inv_t) + (c.b * t.powi(2))),
            a: 1.0,
        }
    };
    set_camera(&Camera3D {
        position: pos * 2. * base_dist,
        target: vec3(0., 0., 0.),
        ..Default::default()
    });
    draw_line_3d(Vec3::ZERO, Vec3::X, RED);
    draw_line_3d(Vec3::ZERO, Vec3::Y, BLUE);
    draw_line_3d(Vec3::ZERO, Vec3::Z, GREEN);
    for &pos in position {
        let t = (pos.length() / base_dist).clamp(0., 1.);
        let color = lerp(LIME, YELLOW, ORANGE, t);
        draw_sphere_wires(pos, spacing / 8., None, color);
    }
    next_frame().await;
    std::time::Instant::now()
}
