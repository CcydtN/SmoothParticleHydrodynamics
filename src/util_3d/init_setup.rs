use std::{
    f32::consts::PI,
    iter,
    time::{SystemTime, UNIX_EPOCH},
};

use super::Particle;
use itertools::{iproduct, izip, Itertools};
use macroquad::{
    prelude::*,
    rand::{gen_range, srand},
};

pub fn diagonal_test(mass: f32, default_kernel_radius: f32) -> Vec<Particle> {
    let position = vec![Vec3::NEG_ONE, Vec3::ZERO, Vec3::ONE];
    let velocity = vec![Vec3::ONE, Vec3::ZERO, Vec3::NEG_ONE];
    izip!(
        position,
        velocity,
        iter::repeat(mass),
        iter::repeat(default_kernel_radius),
    )
    .map(Into::into)
    .collect()
}

pub fn random_points(
    count: usize,
    low: f32,
    high: f32,
    mass: f32,
    default_kernel_radius: f32,
) -> Vec<Particle> {
    srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let random_value = || gen_range(low, high);
    let random_pos = || vec3(random_value(), random_value(), random_value());
    let mut position = (0..count).map(|_| random_pos()).collect_vec();
    izip!(
        position,
        iter::repeat(Vec3::ZERO),
        iter::repeat(mass),
        iter::repeat(default_kernel_radius),
    )
    .map(Into::into)
    .collect()
}

pub fn create_cube(
    spacing: f32,
    particle_per_side: isize,
    center_offset: Vec3,
    mass: f32,
    default_kernel_radius: f32,
) -> Vec<Particle> {
    let center = center_offset + (spacing * (particle_per_side - 1) as f32) / 2. * Vec3::NEG_ONE;

    let position = iproduct!(
        0..particle_per_side,
        0..particle_per_side,
        0..particle_per_side
    )
    .map(|(i, j, k)| center + vec3(i as f32, j as f32, k as f32) * spacing);

    izip!(
        position,
        iter::repeat(Vec3::ZERO),
        iter::repeat(mass),
        iter::repeat(default_kernel_radius),
    )
    .map(Into::into)
    .collect_vec()
}

pub fn create_sphere(
    mass: f32,
    radius: f32,
    count: usize,
    center_offset: Vec3,
    default_kernel_radius: f32,
) -> Vec<Particle> {
    let golden_ratio = (1.0 + f32::sqrt(5.0)) / 2.0;
    let angle_increment = PI * (2.0 / golden_ratio);

    let position = (0..count).map(|i| {
        let y = (1.0 - (i as f32) / (count as f32) * 2.0) * radius;
        let radius_at_y = (radius.powi(2) - y.powi(2)).sqrt();
        let phi = i as f32 * angle_increment;

        let x = phi.cos() * radius_at_y;
        let z = phi.sin() * radius_at_y;
        vec3(x, y, z) + center_offset
    });
    izip!(
        position,
        iter::repeat(Vec3::ZERO),
        iter::repeat(mass),
        iter::repeat(default_kernel_radius),
    )
    .map(Into::into)
    .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_side_cube() {
        let particles = create_cube(1., 2, Vec3::ZERO, 1., 1.);
        let expect = vec![
            vec3(-0.5, -0.5, -0.5),
            vec3(-0.5, -0.5, 0.5),
            vec3(-0.5, 0.5, -0.5),
            vec3(-0.5, 0.5, 0.5),
            vec3(0.5, -0.5, -0.5),
            vec3(0.5, -0.5, 0.5),
            vec3(0.5, 0.5, -0.5),
            vec3(0.5, 0.5, 0.5),
        ];
        let position = particles.into_iter().map(|p| p.position).collect_vec();

        dbg!(&position);
        assert_eq!(position.len(), 8);
        position.into_iter().zip(expect).for_each(|(a, b)| {
            let len = (a - b).length();
            assert!(len <= f32::EPSILON, "left: {:?}, right: {:?}", a, b);
        });
    }

    #[test]
    fn odd_side_cube() {
        let particles = create_cube(1., 3, Vec3::ZERO, 1., 1.);
        let expect = vec![
            vec3(-1., -1., -1.),
            vec3(-1., -1., 0.),
            vec3(-1., -1., 1.),
            vec3(-1., 0., -1.),
            vec3(-1., 0., 0.),
            vec3(-1., 0., 1.),
            vec3(-1., 1., -1.),
            vec3(-1., 1., 0.),
            vec3(-1., 1., 1.),
            vec3(0., -1., -1.),
            vec3(0., -1., 0.),
            vec3(0., -1., 1.),
            vec3(0., 0., -1.),
            vec3(0., 0., 0.),
            vec3(0., 0., 1.),
            vec3(0., 1., -1.),
            vec3(0., 1., 0.),
            vec3(0., 1., 1.),
            vec3(1., -1., -1.),
            vec3(1., -1., 0.),
            vec3(1., -1., 1.),
            vec3(1., 0., -1.),
            vec3(1., 0., 0.),
            vec3(1., 0., 1.),
            vec3(1., 1., -1.),
            vec3(1., 1., 0.),
            vec3(1., 1., 1.),
        ];

        let position = particles.into_iter().map(|p| p.position).collect_vec();
        dbg!(&position);
        assert_eq!(position.len(), 27);
        position.into_iter().zip(expect).for_each(|(a, b)| {
            let len = (a - b).length();
            assert!(len <= f32::EPSILON, "left: {:?}, right: {:?}", a, b);
        });
    }
}
