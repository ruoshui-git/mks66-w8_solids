use super::Matrix;
use crate::graphics::{parametrics::Parametric, utils};
use std::f64::consts;

// draw parametric
impl Matrix {
    /// Add a parametric curve
    /// # Arguments
    /// `x` - Function that takes in `t` from 0 to 1 and produces x
    /// `y` - Function that takes in `t` from 0 to 1 and produces y
    /// `z` - The z value that the curve will be on
    /// `step` - Controls the precision of the curves
    pub fn add_parametric<F1, F2>(&mut self, xf: F1, yf: F2, z: f64, step: f64)
    where
        F1: Fn(f64) -> f64,
        F2: Fn(f64) -> f64,
    {
        let p = Parametric::new(xf, yf);
        for points in p.points_iter(step).collect::<Vec<(f64, f64)>>().windows(2) {
            let (x0, y0) = points[0];
            let (x1, y1) = points[1];
            self.append_edge(&[x0, y0, z, x1, y1, z]);
        }
    }

    /// Add a circle with center c `(x, y, z)` and radius `r`
    pub fn add_circle(&mut self, c: (f64, f64, f64), r: f64) {
        let (x, y, z) = c;
        self.add_parametric(
            |t: f64| r * (t * 2.0 * consts::PI).cos() + x,
            |t: f64| r * (t * 2.0 * consts::PI).sin() + y,
            z,
            0.001,
        );
    }

    /// Add a cubic Bezier curve
    /// # Arguments
    /// `p[0-3]` - control points
    pub fn add_bezier3(&mut self, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) {
        let (ax, bx, cx, dx) = utils::compute_bezier3_coef(p0.0, p1.0, p2.0, p3.0);
        let (ay, by, cy, dy) = utils::compute_bezier3_coef(p0.1, p1.1, p2.1, p3.1);
        self.add_parametric(
            |t: f64| ax * t * t * t + bx * t * t + cx * t + dx,
            |t: f64| ay * t * t * t + by * t * t + cy * t + dy,
            0.0,
            0.001,
        );
    }

    pub fn add_hermite3(&mut self, p0: (f64, f64), p1: (f64, f64), r0: (f64, f64), r1: (f64, f64)) {
        let (ax, bx, cx, dx) = utils::compute_hermite3_coef(p0.0, p1.0, r0.0, r1.0);
        let (ay, by, cy, dy) = utils::compute_hermite3_coef(p0.1, p1.1, r0.1, r1.1);
        self.add_parametric(
            |t: f64| ax * t * t * t + bx * t * t + cx * t + dx,
            |t: f64| ay * t * t * t + by * t * t + cy * t + dy,
            0.0,
            0.0001,
        );
    }
}
