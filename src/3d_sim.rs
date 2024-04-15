mod kernel;
mod model;
mod util_3d;

use itertools::iproduct;
use kernel::spiky::Spiky;
use macroquad::prelude::*;
use model::{density::Density, pressure::Pressure};
use uom::si::{f32::MassDensity, mass_density};
use util_3d::spatial_hash_grid::SpatialHashGrid;

struct Material {
    density: MassDensity,
}

impl Material {
    fn get_density(&self) -> f32 {
        self.density.get::<mass_density::kilogram_per_cubic_meter>()
    }
}

#[macroquad::main("simulation")]
async fn main() {
    // Constant for water
    let water = Material {
        density: MassDensity::new::<mass_density::kilogram_per_cubic_meter>(1000.),
    };

    let rest_density = water.get_density();
    let pressure_constant = 0.0001;

    let mass = 1. / 1000.; //1 gram or 0.001 kg
    let particle_per_side = 10i32;
    let particle_count = particle_per_side.pow(3); // total 1000;
    let spacing = (mass * particle_count as f32 / rest_density).powf(1. / 3.);
    let kernel_radius = spacing * 4.0;

    println!("{:?}, {:?}", spacing, kernel_radius);

    let mut position = vec![];
    let mut velocity = vec![Vec3::ZERO; particle_count as usize];
    let offset = particle_per_side / 2;
    for (i, j, k) in iproduct!(-offset..offset, -offset..offset, -offset..offset) {
        position.push(
            vec3(
                spacing * (i as f32 + 0.5),
                spacing * (j as f32 + 0.5),
                spacing * (k as f32 + 0.5),
            ) * 1.0,
        );
    }

    let kernel = Spiky::new(kernel_radius);
    let density_model = Density::new(kernel, mass);
    let pressure_model = Pressure::new(pressure_constant, rest_density, mass, kernel);
    let mut grid = SpatialHashGrid::new(kernel_radius);

    let time_step = 1. / 60.;
    let mut t: f32 = 0.;
    loop {
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let pressure = pressure_model.compute(&grid, &position, &density);

        // println!("{:?}", &pressure[0..10]);

        let acceleration = pressure
            .into_iter()
            .zip(density.iter())
            .map(|(p, &rho)| p / rho)
            .collect::<Vec<_>>();

        velocity
            .iter_mut()
            .zip(acceleration.iter())
            .for_each(|(v, &a)| *v += a * time_step);

        position
            .iter_mut()
            .zip(velocity.iter())
            .for_each(|(d, &v)| *d += v * time_step);

        clear_background(WHITE);
        set_camera(&Camera3D {
            position: vec3((2. * t).sin(), (3. * t).cos(), (t * 0.5).sin()).normalize() * 4.,
            target: vec3(0., 0., 0.),
            ..Default::default()
        });
        for &pos in &position {
            draw_sphere(pos, spacing / 5., None, SKYBLUE);
        }
        next_frame().await;
        t += time_step;
    }
}
