use crate::kernel;
use crate::util_3d::spatial_hash_grid::SpatialHashGrid;
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
        let mut density = vec![];
        for i in position {
            let mut tmp = 0.;
            for j in grid.lookup(i).map(|&x| position[x]) {
                tmp += self.mass * self.kernel.function(i.distance(j))
            }
            density.push(tmp)
        }
        density
    }
}
