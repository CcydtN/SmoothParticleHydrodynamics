use std::{
    ops::{Range, RangeInclusive},
    time::{SystemTime, UNIX_EPOCH},
};

use super::Particle;
use itertools::Itertools;
use macroquad::{
    prelude::*,
    rand::{gen_range, srand},
};

pub fn diagonal_3_points() -> Vec<Particle> {
    let template = vec3(1., 1., 1.);
    let position = vec![template * 0., template * 1., template * 2.];
    let velocity = vec![template * 1., template * 0., template * -1.];
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
