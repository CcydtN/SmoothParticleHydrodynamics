use itertools::iproduct;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct HashGrid {
    grid_size: f32,
    map: HashMap<[u32; 2], Vec<usize>>,
    // Can have a index_table storing all index and sorted by key.
    // Then, the map points to the start and size of that.
}

impl HashGrid {
    pub fn new(grid_size: f32) -> Self {
        Self {
            grid_size,
            ..Default::default()
        }
    }

    fn get_key(&self, position: &Vec2) -> [u32; 2] {
        let Vec2 { x, y } = (*position / self.grid_size).ceil();
        [x.to_bits(), y.to_bits()]
    }

    pub fn update(&mut self, positions: &[Vec2]) {
        self.map.clear();
        for (i, position) in positions.iter().enumerate() {
            let key = self.get_key(position);
            self.map.entry(key).or_default().push(i);
        }
    }

    pub fn lookup(&self, position: &Vec2) -> Vec<usize> {
        let r = 1i16;
        iproduct!(-r..=r, -r..=r)
            .map(|(i, j)| *position + self.grid_size * vec2(i.into(), j.into()))
            .map(|pos| self.get_key(&pos))
            .filter_map(|key| self.map.get(&key).cloned())
            .flatten()
            .collect()
    }
}
