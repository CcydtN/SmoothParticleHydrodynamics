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
        let x = (pos / self.grid_size).ceil().to_array();
        [x[0] as i32, x[1] as i32, x[2] as i32]
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

    pub fn lookup(&self, position: &Vec3, support_radius: f32) -> impl Iterator<Item = &usize> {
        let r: i32 = (support_radius / self.grid_size).ceil() as i32;
        let key = self.hash(*position);
        let x = key[0] - r..=key[0] + r;
        let y = key[1] - r..=key[1] + r;
        let z = key[2] - r..=key[2] + r;
        iproduct!(x, y, z)
            .map(move |v| [v.0, v.1, v.2])
            .filter_map(|key| self.info.get(&key).cloned())
            .filter_map(|index| self.table.get(index))
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
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
        let search_size = 2. * grid_size;
        let position = generate_points(1000);
        type T = HashSet<usize>;

        let mut grid = SpatialHashGrid::new(grid_size);
        grid.update(&position);

        for &pos in &position {
            let answer = position
                .iter()
                .enumerate()
                .filter(|(_, &val)| pos.distance(val) <= search_size)
                .map(|(idx, _)| idx)
                .collect::<T>();

            let ret = grid.lookup(&pos, search_size).cloned().collect::<T>();
            assert_eq!(ret.intersection(&answer).count(), answer.len());
        }
    }
}
