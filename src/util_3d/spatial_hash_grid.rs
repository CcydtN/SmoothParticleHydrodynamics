use std::{collections::HashMap, ops::Range};

use itertools::iproduct;
use macroquad::prelude::*;

type Key = [i32; 3];

#[derive(Debug, Default)]
pub struct SpatialHashGrid {
    grid_size: f32,
    table: Vec<usize>,
    info: HashMap<Key, Range<usize>>,
}

impl SpatialHashGrid {
    pub fn new(grid_size: f32) -> Self {
        Self {
            grid_size,
            ..Default::default()
        }
    }

    fn hash(&self, pos: Vec3) -> Key {
        (pos / self.grid_size).ceil().as_ivec3().to_array()
    }

    pub fn update(&mut self, position: &Vec<Vec3>) {
        let mut tmp: HashMap<Key, Vec<usize>> = Default::default();
        for (i, pos) in position.iter().enumerate() {
            let hash = self.hash(*pos);
            let entry = tmp.entry(hash).or_default();
            entry.push(i);
        }

        self.info = tmp
            .iter()
            .scan(0usize, |state, (hash, val)| {
                let begin = *state;
                *state += val.len();
                Some((*hash, begin..*state))
            })
            .collect();
        self.table = tmp.into_values().flatten().collect();
    }

    pub fn lookup(&self, position: &Vec3) -> impl Iterator<Item = &usize> {
        let r = 1i32;
        let coor = self.hash(position.clone());
        iproduct!(-r..=r, -r..=r, -r..=r)
            .map(move |(i, j, k)| [coor[0] + i, coor[1] + j, coor[2] + k])
            .filter_map(|key| self.info.get(&key).cloned())
            .filter_map(|index| self.table.get(index))
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macroquad::rand::gen_range;
    use std::collections::HashSet;

    fn generate_points(count: usize) -> Vec<Vec3> {
        let random_pos =
            || Vec3::from_array([gen_range(-5., 5.), gen_range(-5., 5.), gen_range(-5., 5.)]);
        let mut position = vec![];
        for _ in 0..count {
            position.push(random_pos())
        }
        position
    }

    #[test]
    fn random_point_cover_test() {
        let grid_size = 2.;
        let position = generate_points(1000);
        type T = HashSet<usize>;

        let mut grid = SpatialHashGrid::new(grid_size);
        grid.update(&position);

        for &pos in &position {
            let answer = position
                .iter()
                .enumerate()
                .filter(|(_, &val)| pos.distance(val) < grid_size)
                .map(|(idx, _)| idx)
                .collect::<T>();

            let ret = grid.lookup(&pos).cloned().collect::<T>();
            assert_eq!(ret.intersection(&answer).cloned().collect::<T>(), answer);
        }
    }
}
