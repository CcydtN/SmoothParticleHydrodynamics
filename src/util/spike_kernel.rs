use std::f32::consts::PI;

pub fn smoothing_kernel(r: f32, h: f32) -> f32 {
    if r >= h {
        return 0.;
    }
    let volume = PI * h.powi(4) / 6.;
    (h - r).powi(2) / volume
}

pub fn smoothing_kernel_gradient(r: f32, h: f32) -> f32 {
    if r >= h {
        return 0.;
    }
    let volume = -PI * h.powi(4) / 12.;
    (h - r) / volume
}

pub fn smoothing_kernel_laplacian(r: f32, h: f32) -> f32 {
    if r >= h {
        return 0.;
    }
    let volume = PI * h.powi(4) / 12.;
    1. / volume
}
