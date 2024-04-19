use crate::kernel;
use crate::util_3d::spatial_hash_grid::SpatialHashGrid;
use itertools::Itertools;
use macroquad::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct Density<T: kernel::Kernel> {
    kernel: T,
    mass: f32,
}

impl<T: kernel::Kernel> Density<T> {
    pub fn new(kernel: T, mass: f32) -> Self {
        Self { kernel, mass }
    }

    pub fn compute(&self, grid: &SpatialHashGrid, position: &Vec<Vec3>) -> Vec<f32> {
        let density_equation = |r| self.mass * self.kernel.function(r);

        let get_density = |i| {
            grid.lookup(i)
                .map(|&x| i.distance(position[x]))
                .map(density_equation)
                .sum()
        };
        position.into_iter().map(get_density).collect_vec()
    }
}
