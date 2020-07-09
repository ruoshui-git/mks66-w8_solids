//! Implements fn that add shapes to a vertex matrix

use super::Matrix;
use std::f64::consts::PI;

// constructor
impl Matrix {
    pub fn new_polygon_matrix() -> Self {
        Matrix {
            nrows: 0,
            ncols: 4,
            data: vec![],
        }
    }

    /// Add the three vertices of a triangle into the polygon list.
    ///
    /// Note: The vertices must be added in counter-clockwise order
    pub fn append_polygon(
        &mut self,
        (x0, y0, z0): (f64, f64, f64),
        (x1, y1, z1): (f64, f64, f64),
        (x2, y2, z2): (f64, f64, f64),
    ) {
        self.data
            .extend_from_slice(&[x0, y0, z0, 1., x1, y1, z1, 1., x2, y2, z2, 1.]);
        self.nrows += 3;
    }
}

// box, sphere, torus
impl Matrix {
    /// Add a 3d rectangular box to the matrix
    pub fn add_box(&mut self, (x, y, z): (f64, f64, f64), dx: f64, dy: f64, dz: f64) {
        // let (x0, y0, z0) = point;
        // define the four points in the front
        let p1 = (x, y, z);
        let p2 = (x, y - dy, z);
        let p3 = (x + dx, y, z);
        let p4 = (x + dx, y - dy, z);

        // define the four points in the back, lined up with front
        let p5 = (x, y, z - dz);
        let p6 = (x, y - dy, z - dz);
        let p7 = (x + dx, y, z - dz);
        let p8 = (x + dx, y - dy, z - dz);

        // front
        self.append_polygon(p1, p2, p3);
        self.append_polygon(p3, p2, p4);

        // right
        self.append_polygon(p3, p4, p8);
        self.append_polygon(p3, p8, p7);

        // back
        self.append_polygon(p7, p8, p6);
        self.append_polygon(p7, p6, p5);

        // left
        self.append_polygon(p5, p6, p2);
        self.append_polygon(p5, p2, p1);

        // top
        self.append_polygon(p7, p1, p3);
        self.append_polygon(p7, p5, p1);

        // btm
        self.append_polygon(p6, p4, p2);
        self.append_polygon(p6, p8, p4);

        /*
        // old
        // self.append_edge(&[x0, y0, z0, x0 + dx, y0, z0]);
        // self.append_edge(&[x0, y0, z0, x0, y0 - dy, z0]);
        // self.append_edge(&[x0 + dx, y0, z0, x0 + dx, y0 - dy, z0]);
        // self.append_edge(&[x0, y0 - dy, z0, x0 + dx, y0 - dy, z0]);

        // self.append_edge(&[x0, y0, z0 - dz, x0 + dx, y0, z0 - dz]);
        // self.append_edge(&[x0, y0, z0 - dz, x0, y0 - dy, z0 - dz]);
        // self.append_edge(&[x0 + dx, y0, z0 - dz, x0 + dx, y0 - dy, z0 - dz]);
        // self.append_edge(&[x0, y0 - dy, z0 - dz, x0 + dx, y0 - dy, z0 - dz]);

        // self.append_edge(&[x0, y0, z0, x0, y0, z0 - dz]);
        // self.append_edge(&[x0 + dx, y0, z0, x0 + dx, y0, z0 - dz]);
        // self.append_edge(&[x0, y0 - dy, z0, x0, y0 - dy, z0 - dz]);
        // self.append_edge(&[x0 + dx, y0 - dy, z0, x0 + dx, y0 - dy, z0 - dz]);
        */
    }

    pub fn add_sphere(&mut self, center: (f64, f64, f64), radius: f64) {
        let steps = 24;

        let (cx, cy, cz) = center;
        //  for rot: 0 -> 1
        //   for cir: 0 -> 1
        //     x = r * cos(π * cir) + Cx
        //     y = r * sin(π * cir) * cos(2π * rot) + Cy
        //     z = r * sin(π * cir) * sin(2π * rot) + Cz
        //

        // points per semicircle
        let pps = steps + 1;

        let mut points = Vec::with_capacity(steps * pps);
        for rot in 0..steps {
            let rotc = rot as f64 * 2. * PI / steps as f64;
            for cir in 0..=steps {
                let circ = cir as f64 * PI / steps as f64;
                let x = radius * circ.cos() + cx;
                let y = radius * circ.sin() * rotc.cos() + cy;
                let z = radius * circ.sin() * rotc.sin() + cz;
                points.push((x, y, z));
            }
        }
        /*
        // while rot <= 1. {
        //     let rotc = rot * 2. * PI;
        //     let mut cir = 0.;
        //     while cir <= 1. {
        //         let circ = cir * PI;
        //         let x = radius * circ.cos() + cx;
        //         let y = radius * circ.sin() * rotc.cos() + cy;
        //         let z = radius * circ.sin() * rotc.sin() + cz;
        //         points.push((x, y, z));
        //         cir += step;
        //     }
        //     rot += step;
        // }
        // add all points to matrix
        // for point in points {
        //     let (x, y, z) = point;
        //     self.append_edge(&[x, y, z, x + 1., y, z]);
        // }
         */
        for (i, pointi) in points.iter().enumerate() {
            // we are at the poles if i is multiple of pps or pps-1

            let rem = i % pps;

            if rem == 0 {
                // i + 1 will never overflow points.len() here
                self.append_polygon(*pointi, points[i + 1], points[(i + pps + 1) % points.len()]);
            } else if rem != pps - 1 {
                // all normal places along the sphere

                let index = (i + pps + 1) % points.len();
                // index i + 1 will not overflow here as well
                self.append_polygon(*pointi, points[i + 1], points[index]);
                self.append_polygon(*pointi, points[index], points[(i + pps) % points.len()]);
            } else {
                // rem == pps - 1, at the other end of the pole, do nothing
            }
        }
    }

    /// radius1: inner radius
    /// radius2: big radius
    pub fn add_torus(&mut self, center: (f64, f64, f64), radius1: f64, radius2: f64) {
        let mut points = vec![];
        // x = cos(p) * (rcos(t) + R) + Cx
        // y = rsin(t) + Cy
        // z = -sin(p) * (rcos(t) + R) + Cz
        let (cx, cy, cz) = center;
        let steps = 21;
        for torus_angle_norm in 0..steps {
            let torus_angle = torus_angle_norm as f64 * 2. * PI / steps as f64;
            for circ_angle_norm in 0..steps {
                let circ_angle = circ_angle_norm as f64 * 2. * PI / steps as f64;
                let x = circ_angle.cos() * (radius1 * torus_angle.cos() + radius2) + cx;
                let y = radius1 * torus_angle.sin() + cy;
                let z = -circ_angle.sin() * (radius1 * torus_angle.cos() + radius2) + cz;
                points.push((x, y, z));
            }
        }

        for (i, pointi) in points.iter().enumerate() {
            let index = (i + steps + 1) % points.len();

            self.append_polygon(*pointi, points[(i + 1) % points.len()], points[index]);
            self.append_polygon(*pointi, points[index], points[(i + steps) % points.len()]);
        }

        /*
        // // add all points to matrix
        // for (i, point) in points.into_iter().enumerate() {
        //     let (x, y, z) = point;
        //     self.append_edge(&[x, y, z, x + 1., y, z]);
        //     if i % steps == 0 {
        //         super::super::utils::display_edge_matrix(&self, false);
        //     }
        // }
         */
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::{matrix::transform, utils::display_polygon_matrix};

    #[test]
    fn draw_sphere() {
        let mut m = Matrix::new_polygon_matrix();
        m.add_sphere((250., 250., 0.), 400.);
        m *= transform::rotatex(40.) * transform::rotatey(90.);
        println!("len of array: {}", m.data.len());
        display_polygon_matrix(&m, false);
    }

    #[test]
    fn test_no_degenerate_triangles_in_sphere() {
        for radius in (0..1000).into_iter().step_by(100) {
            let mut m = Matrix::new_polygon_matrix();
            m.add_sphere((0., 0., 0.), radius as f64 + 0.3);

            for chunk in m.data.chunks_exact(m.ncols * 3) {
                if let [x0, y0, z0, _w0, x1, y1, z1, _w1, x2, y2, z2, _w2] = chunk {
                    assert_ne!((x0, y0, z0), (x1, y1, z1));
                    assert_ne!((x1, y1, z1), (x2, y2, z2));
                    assert_ne!((x0, y0, z0), (x2, y2, z2));
                } else {
                    panic!("Something is wrong");
                }
            }
        }
    }

    #[test]
    fn draw_torus() {
        let mut m = Matrix::new_edge_matrix();
        m.add_torus((250., 250., 0.), 30., 100.);
        m *= transform::rotatex(40.);
        display_polygon_matrix(&m, false);
    }
    #[test]
    fn draw_cube() {
        let mut m = Matrix::new_polygon_matrix();
        m.add_box((220., 100., 100.), 100., -100., 100.);
        // println!("{}", m);
        m *= Matrix::ident(4)
            * transform::mv(120., 20., 40.)
            * transform::rotatex(40.)
            * transform::rotatey(20.);

        display_polygon_matrix(&m, false);
    }
}
