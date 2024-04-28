use itertools::{iproduct, Itertools};
use rayon::prelude::*;
use std::{collections::HashMap, iter, ops::Deref};

use crate::kernel;

use super::Particle;

type Key = [i32; 3];

#[derive(Debug, Default)]
pub struct Space {
    grid_size: f32,
    table: HashMap<Key, Vec<Particle>>,
    update_count: usize,
    particle_count: usize,
}

#[inline]
fn hash(grid_size: f32, particle: &Particle) -> Key {
    (particle.position / grid_size).as_ivec3().to_array()
}

impl Space {
    pub fn new(grid_size: f32, particles: Vec<Particle>) -> Self {
        let mut obj = Self {
            grid_size,
            ..Default::default()
        };
        obj.particle_count += particles.len();
        obj.table = particles
            .into_iter()
            .into_group_map_by(|v| hash(grid_size, v));
        obj
    }

    #[inline]
    fn add(&mut self, particle: Particle) {
        let key = hash(self.grid_size, &particle);
        self.table.entry(key).or_default().push(particle)
    }

    #[inline]
    pub fn add_one(&mut self, particle: Particle) {
        self.particle_count += 1;
        self.add(particle);
    }

    #[inline]
    pub fn add_bulk(&mut self, particles: Vec<Particle>) {
        self.particle_count += particles.len();
        particles.into_iter().for_each(|p| self.add(p))
    }

    pub fn update(&mut self) {
        let mut dropped = vec![];
        self.table.iter_mut().for_each(|(key, val)| {
            let (stay, mut drop): (Vec<_>, Vec<_>) =
                val.drain(..).partition(|p| &hash(self.grid_size, p) == key);
            *val = stay;
            dropped.append(&mut drop);
        });

        self.particle_count -= dropped.len();
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

    pub fn par_particles_mut(&mut self) -> impl ParallelIterator<Item = &mut Particle> {
        self.table
            .par_iter_mut()
            .flat_map(|(_, v)| v.par_iter_mut())
    }

    pub fn neighbour(
        &self,
        particle: &Particle,
        radius: f32,
    ) -> impl Iterator<Item = &Particle> + Clone {
        let key = hash(self.grid_size, particle);
        self.neighbour_by_key(&key, radius)
    }

    fn neighbour_by_key(&self, key: &Key, radius: f32) -> impl Iterator<Item = &Particle> + Clone {
        let r: i32 = (radius / self.grid_size).ceil() as i32;
        let x = key[0] - r..=key[0] + r;
        let y = key[1] - r..=key[1] + r;
        let z = key[2] - r..=key[2] + r;
        iproduct!(x, y, z)
            .map(move |v| [v.0, v.1, v.2])
            .filter_map(|index| self.table.get(&index))
            .flatten()
    }

    pub fn count(&self) -> usize {
        self.particle_count
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
        let mass = 1.;

        let particles = init_setup::random_points(1000, -5., 5., mass);
        let grid = Space::new(grid_size, particles.clone());

        for a in grid.particles() {
            let expect = particles
                .iter()
                .filter(|b| a.position.distance(b.position) <= search_size)
                .collect_vec();

            let ret = grid.neighbour(a, search_size).collect_vec();
            assert!(ret.len() >= expect.len());
            assert_eq!(
                ret.into_iter().filter(|x| expect.contains(&x)).count(),
                expect.len()
            );
        }
    }
}
