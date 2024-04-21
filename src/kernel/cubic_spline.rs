use crate::kernel::definition::KernelImpl;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct CubicSpline {
    h: f32,
    volume: f32,
}

impl CubicSpline {
    pub fn new(h: f32) -> Self {
        let volume = 4. * h.powi(3) * PI;
        Self { h, volume }
    }
}

impl KernelImpl for CubicSpline {
    fn support_radius_impl(&self) -> f32 {
        2. * self.h
    }

    fn function_impl(&self, r: f32) -> f32 {
        debug_assert!(r >= 0.0, "value of r: {}", r);
        let value = match r {
            x if x <= self.h => {
                (-4. * (self.h - r).powi(3) + (2. * self.h - r).powi(3)) / self.h.powi(6)
            }
            x if x <= 2. * self.h => (2. * self.h - r).powi(3) / self.h.powi(6),
            _ => 0.,
        };
        value / self.volume
    }

    fn gradient_impl(&self, r: f32) -> f32 {
        debug_assert!(r >= 0.0, "value of r: {}", r);
        let value = match r {
            x if x <= self.h => {
                3. * (4. * self.h.powi(3) * (self.h - r).powi(2) - (2. * self.h - r).powi(2))
                    / self.h.powi(6)
            }
            x if x <= 2. * self.h => -3. * (2. * self.h - r).powi(2) / self.h.powi(6),
            _ => 0.,
        };
        value / self.volume
    }

    fn lapacian_impl(&self, r: f32) -> f32 {
        debug_assert!(r >= 0.0, "value of r: {}", r);
        let value = match r {
            x if x <= self.h => {
                6. * (4. * self.h.powi(3) * (-self.h + r) + 2. * self.h - r) / self.h.powi(6)
            }
            x if x <= 2. * self.h => 6. * (2. * self.h - r) / self.h.powi(6),
            _ => 0.,
        };
        value / self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::definition::tests;

    #[test]
    fn verify_function() {
        let function_values = [
            (0.0, 0.318309886183791).into(),
            (0.2, 0.301121152329866).into(),
            (0.4, 0.257194388036503).into(),
            (0.6, 0.197988749206318).into(),
            (0.8, 0.134963391741927).into(),
            (1.0, 0.0795774715459477).into(),
            (1.2, 0.0407436654315252).into(),
            (1.4, 0.0171887338539247).into(),
            (1.6, 0.00509295817894065).into(),
            (1.8, 0.000636619772367581).into(),
            (2.0, 0.).into(),
        ];
        let kernel = CubicSpline::new(1.);
        tests::check_function(kernel, &function_values);
    }

    #[test]
    fn verify_gradient() {
        let values = [
            (0.0, 0.).into(),
            (0.2, -0.162338041953733).into(),
            (0.4, -0.267380304394384).into(),
            (0.6000000000000001, -0.315126787321953).into(),
            (0.8, -0.305577490736439).into(),
            (1.0, -0.238732414637843).into(),
            (1.2, -0.152788745368220).into(),
            (1.4, -0.0859436692696235).into(),
            (1.6, -0.0381971863420549).into(),
            (1.8, -0.00954929658551372).into(),
            (2.0, 0.).into(),
        ];
        let kernel = CubicSpline::new(1.);
        tests::check_gradient(kernel, &values);
    }

    #[test]
    fn verify_lapcian() {
        let values = [
            (0.0, -0.954929658551372).into(),
            (0.2, -0.668450760985960).into(),
            (0.4, -0.381971863420549).into(),
            (0.6000000000000001, -0.0954929658551371).into(),
            (0.8, 0.190985931710274).into(),
            (1.0, 0.477464829275686).into(),
            (1.2, 0.381971863420549).into(),
            (1.4, 0.286478897565412).into(),
            (1.6, 0.190985931710274).into(),
            (1.8, 0.0954929658551372).into(),
            (2.0, 0.).into(),
        ];
        let kernel = CubicSpline::new(1.);
        tests::check_lapcian(kernel, &values);
    }
}
