use itertools::{iproduct, Itertools};
use std::{collections::HashMap, iter};

use crate::kernel;

use super::Particle;

type Key = [i32; 3];

#[derive(Debug, Default)]
pub struct Space {
    grid_size: f32,
    table: HashMap<Key, Vec<Particle>>,
    update_count: usize,
}

#[inline]
fn hash(grid_size: f32, particle: &Particle) -> Key {
    (particle.position / grid_size).as_ivec3().to_array()
}

impl Space {
    pub fn new<T>(grid_size: f32, particles: T) -> Self
    where
        T: IntoIterator<Item = Particle>,
    {
        let mut obj = Self {
            grid_size,
            ..Default::default()
        };
        obj.table = particles
            .into_iter()
            .into_group_map_by(|v| hash(grid_size, v));
        obj
    }

    #[inline]
    pub fn add_one(&mut self, particle: Particle) {
        let key = hash(self.grid_size, &particle);
        let entry = self.table.entry(key).or_default();
        entry.push(particle);
    }

    #[inline]
    pub fn add_bulk<T>(&mut self, particles: T)
    where
        T: IntoIterator<Item = Particle>,
    {
        particles.into_iter().for_each(|p| self.add_one(p))
    }

    pub fn update(&mut self) {
        let mut dropped = vec![];
        self.table.iter_mut().for_each(|(key, val)| {
            let (stay, mut drop): (Vec<_>, Vec<_>) =
                val.drain(..).partition(|p| &hash(self.grid_size, p) == key);
            *val = stay;
            dropped.append(&mut drop);
        });
        self.add_bulk(dropped);

        self.update_count += 1;
        if self.update_count == 100 {
            self.table.retain(|_, v| v.len() != 0);
            self.update_count = 0;
        }
    }

    pub fn particles(&self) -> impl Iterator<Item = &Particle> {
        self.table.values().flatten()
    }

    pub fn particles_mut(&mut self) -> impl Iterator<Item = &mut Particle> {
        self.table.values_mut().flatten()
    }

    pub fn particles_with_neighbour(
        &self,
        radius: f32,
    ) -> impl Iterator<Item = (&Particle, impl Iterator<Item = &Particle>)> {
        self.table.iter().flat_map(move |(key, value)| {
            let nei = self.neighbour(key, radius);
            value.iter().map(move |v| (v, nei.clone()))
        })
    }

    fn neighbour(&self, key: &Key, radius: f32) -> impl Iterator<Item = &Particle> + Clone {
        let r: i32 = (radius / self.grid_size).ceil() as i32;
        let x = key[0] - r..=key[0] + r;
        let y = key[1] - r..=key[1] + r;
        let z = key[2] - r..=key[2] + r;
        iproduct!(x, y, z)
            .map(move |v| [v.0, v.1, v.2])
            .filter_map(|index| self.table.get(&index))
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::super::init_setup;
    use super::*;
    #[test]
    fn random_point_cover_test() {
        let grid_size = 1.;
        let search_size = 2. * grid_size;

        let particles = init_setup::random_points(1000, -5., 5.);
        let grid = Space::new(grid_size, particles.clone());

        for (a, iter) in grid.particles_with_neighbour(search_size) {
            let expect = particles
                .iter()
                .filter(|b| a.position.distance(b.position) <= search_size)
                .collect_vec();

            let ret = iter.collect_vec();
            assert!(ret.len() >= expect.len());
            assert_eq!(
                ret.into_iter().filter(|x| expect.contains(&x)).count(),
                expect.len()
            );
        }
    }
}
