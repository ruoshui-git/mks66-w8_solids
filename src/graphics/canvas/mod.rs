extern crate rand;
use rand::Rng;
use crate::graphics::{
    matrix::Matrix,
    utils::{mapper, polar_to_xy},
    vector::Vec3,
    RGB,
};
use std::io;

// turtle will cause problems
// mod turtle;

pub trait Canvas {
    /// Plot a point on the screen at (`x`, `y`, `z`)
    fn plot(&mut self, x: i32, y: i32, z: f64);

    // fn index(&self, x: i32, y: i32) -> Option<usize>;
    fn set_fg_color(&mut self, color: RGB);
    fn set_bg_color(&mut self, color: RGB);
    fn get_fg_color(&self) -> RGB;
    fn get_bg_color(&self) -> RGB;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn save(&self, filepath: &str) -> io::Result<()>;
    fn write_to_buf(&self, writer: &mut dyn io::Write) -> io::Result<()>;

    /// Display the image with imagemagick
    fn display(&self);

    /// Clear the canvas of all drawings and resets configurations like z-buffer
    fn clear(&mut self);

    //----------------------------------------- default methods for drawing lines

    /// Draw a line from (x0, y0) to (x1, y1)
    /// #### impl note:
    ///    Always add 2A or 2B when updating D. Half of that value will distort line
    fn draw_line(&mut self, p0: (f64, f64, f64), p1: (f64, f64, f64)) {
        // swap variables if needed, since we are always going from left to right
        let (p0, p1) = if p0.0 > p1.0 { (p1, p0) } else { (p0, p1) };

        let (x0, y0, z0, x1, y1, z1) = (
            p0.0.round() as i32,
            p0.1.round() as i32,
            p0.2,
            p1.0.round() as i32,
            p1.1.round() as i32,
            p1.2,
        );

        // calculate  values and then truncate
        let (dy, ndx) = (y1 - y0, -(x1 - x0));
        let dz = z1 - z0;

        // check for horizontal life first in case we want to do scanline here
        if dy == 0 {
            // horizontal line
            // x vals are already in the right order, so we don't flip
            let mut z = z0;
            let z_inc = dz / -ndx as f64;
            for x in x0..=x1 {
                self.plot(x, y0, z);
                z += z_inc;
            }
            return;
        }

        // deal with special s:
        if ndx == 0 {
            // vertical line
            let (y0, y1) = if y0 < y1 { (y0, y1) } else { (y1, y0) };
            let mut z = z0;
            let z_inc = dz / dy as f64;
            for y in y0..=y1 {
                self.plot(x0, y, z);
                z += z_inc;
            }

            return;
        }

        // find A and B
        // let m  = -dely as f64 / ndelx as f64;

        let (x, mut y) = (x0, y0);

        if (y1 - y0).abs() < (x1 - x0).abs() {
            // octant 1 and 8
            let mut d = 2 * dy + ndx;
            let (y_inc, dy) = if dy > 0 {
                // octant 1
                (1, dy)
            } else {
                // octant 8
                // dy is (-) in octant 8, so flip it to balance out with ndx
                (-1, -dy)
            };

            let mut z = z0;
            let z_inc = dz / -ndx as f64;
            for x in x0..=x1 {
                self.plot(x, y, z);
                if d > 0 {
                    y += y_inc;
                    d += 2 * ndx;
                }
                d += 2 * dy;
                z += z_inc;
            }
        } else {
            // octant 2 and 7
            // flipping x and y should work out

            let mut d = 2 * -ndx - dy;

            let (x_inc, mut x, ystart, yend, dy) = if dy > 0 {
                // octant 2
                (1, x, y0, y1, dy)
            } else {
                // octant 7
                // swap -x and y to reflect over y=-x into octant 8
                // I think, since we're flipping y, we should also flip z
                (-1, x - ndx, y1, y0, -dy)
            };

            let mut z = z0;
            // dz might be flipped, so recalculate
            let z_inc = (dz) / (y1 - y0) as f64;
            for y in ystart..=yend {
                self.plot(x, y, z);
                if d > 0 {
                    x += x_inc;
                    d -= 2 * dy;
                }
                d -= 2 * ndx;
                z += z_inc;
            }
        }
    }

    /// Draw a line from (x, y, z) with a certain magnitude and angle, on the same z-plane as the point
    /// ## Note
    /// Angle goes counter clockwise from x axis.
    ///
    /// Returns the other endpoint of the line (x1, y1, z) as a tuple
    fn draw_line_degrees(
        &mut self,
        point: (f64, f64, f64),
        angle_degrees: f64,
        mag: f64,
    ) -> (f64, f64, f64) {
        let (dx, dy) = polar_to_xy(mag, angle_degrees);
        let (x1, y1, z) = (point.0 + dx, point.1 + dy, point.2);

        self.draw_line(point, (x1, y1, z));
        return (x1, y1, z);
    }

    //----------------------------------------- render edge matrix on screen

    /// Draws an edge matrix
    ///
    /// Number of edges must be a multiple of 2
    fn render_edge_matrix(&mut self, m: &Matrix) {
        let mut iter = m.iter_by_row();
        while let Some(point) = iter.next() {
            let p0 = (point[0], point[1], point[2]);
            let p1 = match iter.next() {
                Some(point1) => (point1[0], point1[1], point1[2]),
                None => panic!("Number of edges must be a multiple of 2"),
            };

            self.draw_line(p0, p1);
        }
    }

    fn render_ndc_edges_n1to1(&mut self, m: &Matrix) {
        let map_width = mapper(-1., 1., 0., self.width() as f64);
        let map_height = mapper(-1., 1., 0., self.height() as f64);
        let mut iter = m.iter_by_row();
        while let Some(point) = iter.next() {
            let (x0, y0, z0) = (point[0], point[1], point[2]);
            let (x1, y1, z1) = match iter.next() {
                Some(p1) => (p1[0], p1[1], p1[2]),
                None => panic!("Number of edges must be a multiple of 2"),
            };

            // we need to use inverse z to do depth buffer
            self.draw_line(
                (map_width(-x0), map_height(y0), 1. / z0),
                (map_width(-x1), map_height(y1), 1. / z1),
            );
        }
    }

    /// Renders polygon matrix `m` onto screen.
    ///
    /// Removes hidden surface with back-face culling
    /// Also draws scanlines
    fn render_polygon_matrix(&mut self, m: &Matrix) {
        // view vector for now: v = <0, 0, 1>, not needed for computation

        // store default img color for ref later on
        let orig_color = self.get_fg_color();

        let mut iter = m.iter_by_row();
        while let Some(point) = iter.next() {
            let p0 = (point[0], point[1], point[2]);
            let p1 = match iter.next() {
                Some(point1) => (point1[0], point1[1], point1[2]),
                None => panic!("Number of points must be a multiple of 2 for edge matrix"),
            };
            let p2 = match iter.next() {
                Some(point2) => (point2[0], point2[1], point2[2]),
                None => panic!("Number of points must be a multiple of 3 for polygon matrix"),
            };

            // cull back face
            let v0 = Vec3::from_pt(p0);
            let v1 = Vec3::from_pt(p1);
            let v2 = Vec3::from_pt(p2);

            let surface_normal = (v1 - v0).cross(v2 - v0);

            if surface_normal.2 <= 0. {
                continue;
            }
            // self.set_fg_color(RGB::new(255, 255, 255));
            // self.draw_line(p0, p1);
            // self.draw_line(p1, p2);
            // self.draw_line(p2, p0);

            // draw scanlines
            {
                let mut rng = rand::thread_rng();
                self.set_fg_color(RGB::new(
                    rng.gen_range(0, 255),
                    rng.gen_range(0, 255),
                    rng.gen_range(0, 255),
                ));

                // sort points by y value
                let mut points = [v0, v1, v2];
                points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let [vb, vm, vt] = points;

                if (vb.y() - vm.y()).abs() == 0. {
                    // special: two bottom vertices on same horizontal line
                    let d0: Vec3 = (vt - vb) / (vt.y() - vb.y());
                    let d1: Vec3 = (vt - vm) / (vt.y() - vm.y());

                    let yoffsetb = vb.y().ceil() - vb.y();
                    let yoffsetm = vm.y().ceil() - vm.y();

                    let mut x0 = vb.x() + yoffsetb * d0.x();
                    let mut x1 = vm.x() + yoffsetm * d1.x();

                    let mut z0 = vb.z() + yoffsetb * d0.z();
                    let mut z1 = vm.z() + yoffsetm * d1.z();

                    for y in (vb.y().ceil() as i64)..(vt.y().ceil() as i64) {
                        self.draw_scanline((x0, y as f64, z0), (x1, y as f64, z1));

                        x0 += d0.x();
                        z0 += d0.z();

                        x1 += d1.x();
                        z1 += d1.z();
                    }
                } else {
                    // overall diff
                    let dv: Vec3 = (vt - vb) / (vt.y() - vb.y());
                    // bottom diff
                    let dbottom: Vec3 = (vm - vb) / (vm.y() - vb.y());
                    // top diff
                    let dtop: Vec3 = (vt - vm) / (vt.y() - vm.y());

                    let yoffsetb = vb.y().ceil() - vb.y();
                    let yoffsetm = vm.y().ceil() - vm.y();

                    let mut x0 = vb.x() + yoffsetb * dv.x();
                    let mut x1 = vb.x() + yoffsetb * dbottom.x();
                    let mut x2 = vm.x() + yoffsetm * dtop.x();

                    let mut z0 = vb.z() + yoffsetb * dv.z();
                    let mut z1 = vb.z() + yoffsetb * dbottom.z();
                    let mut z2 = vm.z() + yoffsetm * dtop.z();

                    for y in (vb.y().ceil() as i64)..(vm.y().ceil() as i64) {
                        self.draw_scanline((x0, y as f64, z0), (x1, y as f64, z1));

                        x0 += dv.x();
                        x1 += dbottom.x();

                        z0 += dv.z();
                        z1 += dbottom.z();
                    }
                    for y in (vm.y().ceil() as i64)..(vt.y().ceil() as i64) {
                        self.draw_scanline((x0, y as f64, z0), (x2, y as f64, z2));

                        x0 += dv.x();
                        x2 += dtop.x();
                        z0 += dv.z();
                        z2 += dtop.z();
                    }
                }
            }
        }
        self.set_fg_color(orig_color);
    }

    fn draw_scanline(&mut self, p0: (f64, f64, f64), p1: (f64, f64, f64)) {
        assert_eq!(p0.1, p1.1, "Scanline should be horizontal");

        // swap variables if needed, since we are always going from left to right
        let (p0, p1) = if p0.0 > p1.0 { (p1, p0) } else { (p0, p1) };

        let (x0, y0, z0, x1, z1) = (
            p0.0,
            p0.1,
            p0.2,
            p1.0,
            p1.2,
        );

        let y0 = y0.ceil() as i64;
        let xoffset = x0.ceil() - x0;
        // calculate  values and then truncate
        // println!("x1 - x0: {} - {}", x1, x0);

        let z_inc = (z1 - z0) / (x1 - x0) as f64;
        
        let mut z = z0 + xoffset * z_inc;

        for x in (x0.ceil() as i64)..(x1.ceil() as i64) {
            self.plot(x as i32, y0 as i32, z);
            z += z_inc;
        }
    }
    // fn render_polygon_with_stack(&mut self, stack: &impl MStack<Matrix>, m: &Matrix) {
    //     self.render_polygon_matrix(&(m * stack.get_top()));
    // }

    // fn render_edges_with_stack(&mut self, stack: &impl MStack<Matrix>, m: &Matrix) {
    //     self.render_edge_matrix(&(m * stack.get_top()));
    // }
}

#[cfg(test)]
mod tests {
    use super::super::PPMImg;
    use super::*;
    use crate::graphics::utils::display_ppm;

    #[test]
    fn test_render_polygon_triangle() {
        let p0 = (10., 10., 10.);
        let p1 = (100., 100., 100.);
        let p2 = (0., 50., 10.);

        let (h, w, d) = (500, 500, 255);
        let mut img_ln = PPMImg::new(h, w, d);
        let mut img_polygon = PPMImg::new(h, w, d);

        let mut m = Matrix::new_polygon_matrix();
        m.append_polygon(p0, p1, p2);

        img_polygon.render_polygon_matrix(&m);
        img_ln.draw_line(p0, p1);
        img_ln.draw_line(p1, p2);
        img_ln.draw_line(p2, p0);

        display_ppm(&img_polygon);
        assert_ne!(
            img_ln, img_polygon,
            "Expect equivalent images by adding lines vs. drawing polygon"
        );
    }

    #[test]
    fn test_scanline_special() {
        let mut m = Matrix::new_polygon_matrix();
        m.append_polygon((150., 400., 0.), (100., 100., 0.), (300., 100., 0.));
        m.append_polygon((250., 400., 10.), (50., 50., -10.), (400., 400., 10.));
        let mut img = PPMImg::new(500, 500, 255);
        img.render_polygon_matrix(&m);
        img.save("spiky.png").expect("Error writing to file");
    }

    #[test]
    fn test_scanline_regression_streaks() {
        let mut m = Matrix::new_polygon_matrix();
        // these triangles have cause the scanline fn to render long streaks
        m.append_polygon(
            (206.6987298107782, 375.0000000000001, 149.99999999999997),
            (174.17488078277785, 353.87439555706896, 153.16873454901827),
            (198.44244641334058, 353.88486293176453, 162.94095225512604),
        );

        m.append_polygon(
            (347.3988316928974, 318.6997250899479, 160.60453093678348),
            (343.12506492285615, 307.7697258635287, 167.30326074756158),
            (368.7957771709034, 307.90279753888103, 150.1160529841243),
        );

        m.append_polygon(
            (334.51000066356454, 410.15257740702816, 84.90707707682657),
            (337.1191480798316, 381.94792168823415, 122.47448713915887),
            (356.97475792345404, 382.05562026569044, 105.44057247313383),
        );

        let mut img = PPMImg::new(500, 500, 255);
        img.render_polygon_matrix(&m);
        img.save("temp.png").expect("Error writing to file");
    }
}
