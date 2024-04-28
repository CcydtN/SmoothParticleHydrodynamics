use crate::kernel::*;
use crate::model::*;
use crate::util_3d::*;
use crate::Space;
use itertools::{izip, Itertools};
use macroquad::prelude::*;
use rayon::prelude::*;
use uom::si::{
    acceleration,
    f32::{Acceleration, MassDensity},
    mass_density,
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

#[derive(Debug)]
pub struct Simulator {
    t: f32,
    time_step: f32,
    space: Space,
    density_model: density::Density<CubicSpline>,
    pressure_model: pressure::Tait<CubicSpline>,
    viscosity_model: viscosity::Artificial<CubicSpline>,
    surface_tension_model: surface_tension::BeakerTeschner07<CubicSpline>,
    display_distance: f32,
}

impl Simulator {
    pub fn setup() -> Self {
        // Constant for water
        let water = Material {
            density: MassDensity::new::<mass_density::kilogram_per_cubic_meter>(1000.),
        };

        let rest_density = water.get_density();
        let gravity: Acceleration = Acceleration::new::<acceleration::standard_gravity>(1.);
        let gravity = gravity.get::<acceleration::centimeter_per_second_squared>();

        let mass = 1.; // gram
        let particle_per_side = 15isize;

        let particle_count = particle_per_side.pow(3);
        let total_mass = mass * particle_count as f32;
        let spacing = (total_mass / rest_density).powf(1. / 3.) / particle_per_side as f32;

        let default_kernel_radius = 1.3 * (mass / rest_density).powf(1. / 3.);

        dbg!(rest_density, total_mass, spacing, default_kernel_radius);
        let particles = init_setup::create_cube(
            spacing,
            particle_per_side,
            Vec3::ZERO,
            mass,
            default_kernel_radius,
        );

        let speed_of_sound = f32::sqrt(200. * gravity * spacing * particle_per_side as f32 / 2.);

        let space = Space::new(default_kernel_radius, particles);
        let time_step = 0.4 * default_kernel_radius / (speed_of_sound * (1. + 0.6 * 0.1));

        let mut obj = Self {
            t: 0.,
            time_step,
            space,
            density_model: density::Density::new(),
            pressure_model: pressure::Tait::new(rest_density, 7, speed_of_sound),
            viscosity_model: viscosity::Artificial::new(0.08, speed_of_sound),
            surface_tension_model: surface_tension::BeakerTeschner07::new(),
            display_distance: particle_per_side as f32 * spacing,
        };

        obj.density_model.update_density(&mut obj.space);
        obj.density_model.update_density(&mut obj.space);
        obj
    }

    pub fn update(&mut self) {
        self.density_model.update_density(&mut self.space);
        self.pressure_model.update_pressure(&mut self.space);

        let pressure_acc = self.pressure_model.accelration(&self.space);
        let viscosity_acc = self.viscosity_model.accelration(&self.space);
        let surface_tension_acc = self.surface_tension_model.accelration(&self.space);

        let acceleration =
            izip!(pressure_acc, viscosity_acc, surface_tension_acc).map(|t| t.0 + t.1 + t.2);

        self.space
            .particles_mut()
            .zip(acceleration)
            .for_each(|(p, a)| {
                p.velocity += a * self.time_step / 2.;
                p.position += p.velocity * self.time_step;
                p.velocity += a * self.time_step / 2.;
            });

        self.t += self.time_step;
        self.space.update();
    }

    pub fn get_display_distance(&self) -> f32 {
        self.display_distance
    }

    pub fn get_space(&self) -> &Space {
        &self.space
    }

    pub fn get_time(&self) -> f32 {
        self.t
    }
}
