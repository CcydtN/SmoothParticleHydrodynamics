mod kernel;
mod model;
mod util_3d;

use itertools::{iproduct, izip};
use macroquad::prelude::*;
use model::{density::Density, pressure, viscosity::Viscosity};
use uom::si::{
    dynamic_viscosity,
    f32::{DynamicViscosity, MassDensity},
    mass_density,
};
use util_3d::spatial_hash_grid::SpatialHashGrid;

use crate::model::surface_tension::SurfaceTension;

struct Material {
    density: MassDensity,
    viscosity: DynamicViscosity,
    surface_tension: f32,
}

impl Material {
    fn get_density(&self) -> f32 {
        self.density.get::<mass_density::kilogram_per_cubic_meter>()
    }

    fn get_viscosity(&self) -> f32 {
        self.viscosity.get::<dynamic_viscosity::pascal_second>()
    }

    fn get_surface_tension(&self) -> f32 {
        self.surface_tension
    }
}

#[macroquad::main("simulation")]
async fn main() {
    // Constant for water
    let water = Material {
        density: MassDensity::new::<mass_density::kilogram_per_cubic_meter>(1000.),
        viscosity: DynamicViscosity::new::<dynamic_viscosity::micropascal_second>(0.89), // at 25 degree C
        surface_tension: 72.53 / 1000.,
    };

    let rest_density = water.get_density();
    let pressure_constant = 0.01;
    let viscosity_constant = water.get_viscosity() * 100.;
    let surface_tension_coefficient = water.get_surface_tension() * 100.;

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

    let poly6_kernel = kernel::Poly6::new(kernel_radius);
    let spiky_kernel = kernel::Spiky::new(kernel_radius);
    let viscosity_kernel = kernel::Viscosity::new(kernel_radius);

    let density_model = Density::new(poly6_kernel, mass);
    // let pressure_model = pressure::Simple::new(spiky_kernel, mass, pressure_constant, rest_density);
    let pressure_model = pressure::Tait::new(spiky_kernel, mass, rest_density, 7., 0.1, 9.81);
    let viscoity_model = Viscosity::new(viscosity_kernel, mass, viscosity_constant);
    let surface_tension_model =
        SurfaceTension::new(poly6_kernel, surface_tension_coefficient, mass);

    let mut grid = SpatialHashGrid::new(kernel_radius);

    let time_step = 1. / 1000.;
    let mut t: f32 = 0.;
    loop {
        grid.update(&position);
        let density = density_model.compute(&grid, &position);
        let pressure_acc = pressure_model.compute_accelraction(&grid, &position, &density);
        let viscosity_acc =
            viscoity_model.compute_accelration(&grid, &position, &velocity, &density);
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
