use std::{
    ops::{Range, RangeInclusive},
    time::{SystemTime, UNIX_EPOCH},
};

use super::Particle;
use itertools::{iproduct, Itertools};
use macroquad::{
    prelude::*,
    rand::{gen_range, srand},
};

pub fn diagonal_3_points() -> Vec<Particle> {
    let template = Vec3::ONE;
    let position = vec![Vec3::NEG_ONE, Vec3::ZERO, Vec3::ONE];
    let velocity = vec![Vec3::ONE, Vec3::ZERO, Vec3::NEG_ONE];
    position
        .into_iter()
        .zip(velocity.into_iter())
        .map(Into::into)
        .collect()
}

pub fn random_points(count: usize, low: f32, high: f32) -> Vec<Particle> {
    srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let random_value = || gen_range(low, high);
    let random_pos = || vec3(random_value(), random_value(), random_value());
    let mut position = (0..count).map(|_| random_pos()).collect_vec();
    position.into_iter().map(Into::into).collect()
}

pub fn create_cube(spacing: f32, particle_per_side: isize, centre_offset: Vec3) -> Vec<Particle> {
    let center = centre_offset + (spacing * (particle_per_side - 1) as f32) / 2. * Vec3::NEG_ONE;

    iproduct!(
        0..particle_per_side,
        0..particle_per_side,
        0..particle_per_side
    )
    .map(|(i, j, k)| center + vec3(i as f32, j as f32, k as f32) * spacing)
    .map(Into::into)
    .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_side_cube() {
        let particles = create_cube(1., 2, Vec3::ZERO);
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
        let particles = create_cube(1., 3, Vec3::ZERO);
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
