use crate::graphics::{
    matrix::{mstack::MStack, Matrix},
    utils::{mapper, polar_to_xy},
    vector::Vec3,
    RGB,
};

// mod turtle;
// turtle will cause problems with

pub trait Canvas {
    /// Plot a point on the screen at (`x`, `y`), must be impl'ed
    fn plot(&mut self, x: i32, y: i32);

    // fn index(&self, x: i32, y: i32) -> Option<usize>;
    fn set_fg_color(&mut self, color: RGB);
    fn set_bg_color(&mut self, color: RGB);
    fn get_fg_color(&self) -> RGB;
    fn get_bg_color(&self) -> RGB;
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    //----------------------------------------- default methods for drawing lines

    /// Draw a line from (x0, y0) to (x1, y1)
    /// #### impl note:
    ///    Always add 2A or 2B when updating D. Half of that value will distort line
    fn draw_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        // swap variables if needed, since we are always going from left to right
        let (x0, y0, x1, y1) = if x0 > x1 {
            (x1, y1, x0, y0)
        } else {
            (x0, y0, x1, y1)
        };

        // force conversion into ints for processing & plotting
        let (x0, y0, x1, y1) = (
            x0.round() as i32,
            y0.round() as i32,
            x1.round() as i32,
            y1.round() as i32,
        );

        // calculate  values and then truncate
        let (dy, ndx) = (y1 - y0, -(x1 - x0));

        // deal with special s:
        if ndx == 0 {
            // vertical line
            let (y0, y1) = if y0 < y1 { (y0, y1) } else { (y1, y0) };

            for y in y0..=y1 {
                self.plot(x0, y);
            }

            return ();
        }

        if dy == 0 {
            // horizontal line
            // x vals are already in the right order, so we don't flip
            for x in x0..=x1 {
                self.plot(x, y0);
            }
            return ();
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

            for x in x0..=x1 {
                self.plot(x, y);
                if d > 0 {
                    y += y_inc;
                    d += 2 * ndx;
                }
                d += 2 * dy;
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
                (-1, x - ndx, y1, y0, -dy)
            };

            for y in ystart..=yend {
                self.plot(x, y);
                if d > 0 {
                    x += x_inc;
                    d -= 2 * dy;
                }
                d -= 2 * ndx;
            }
        }
    }

    /// Draw a line from (x0, y0) with a certain magnitude and angle
    /// ## Note
    /// Angle goes counter clockwise from x axis.
    ///
    /// Returns the other endpoint of the line (x1, y1) as a tuple
    fn draw_line_degrees(&mut self, x0: f64, y0: f64, angle_degrees: f64, mag: f64) -> (f64, f64) {
        let (dx, dy) = polar_to_xy(mag, angle_degrees);
        let (x1, y1) = (x0 + dx, y0 + dy);

        self.draw_line(x0, y0, x1, y1);
        return (x1, y1);
    }

    //----------------------------------------- render edge matrix on screen

    /// Draws an edge matrix
    ///
    /// Number of edges must be a multiple of 2
    fn render_edge_matrix(&mut self, m: &Matrix) {
        let mut iter = m.iter_by_row();
        while let Some(point) = iter.next() {
            let (x0, y0, _z0) = (point[0], point[1], point[2]);
            let (x1, y1, _z1) = match iter.next() {
                Some(p1) => (p1[0], p1[1], p1[2]),
                None => panic!("Number of edges must be a multiple of 2"),
            };

            self.draw_line(x0, y0, x1, y1);
        }
    }

    fn render_ndc_edges_n1to1(&mut self, m: &Matrix) {
        let map_width = mapper(-1., 1., 0., self.width() as f64);
        let map_height = mapper(-1., 1., 0., self.height() as f64);
        let mut iter = m.iter_by_row();
        while let Some(point) = iter.next() {
            let (x0, y0, _z0) = (point[0], point[1], point[2]);
            let (x1, y1, _z1) = match iter.next() {
                Some(p1) => (p1[0], p1[1], p1[2]),
                None => panic!("Number of edges must be a multiple of 2"),
            };

            self.draw_line(
                map_width(-x0),
                map_height(y0),
                map_width(-x1),
                map_height(y1),
            );
        }
    }

    /// Renders polygon matrix `m` onto screen.
    ///
    /// Removes hidden surface with back-face culling
    fn render_polygon_matrix(&mut self, m: &Matrix) {
        // view vector for now: v = <0, 0, 1>, not needed for computation

        let mut iter = m.iter_by_row();
        while let Some(point) = iter.next() {
            let (x0, y0, z0) = (point[0], point[1], point[2]);
            let (x1, y1, z1) = match iter.next() {
                Some(p1) => (p1[0], p1[1], p1[2]),
                None => panic!("Number of points must be a multiple of 2 for edge matrix"),
            };
            let (x2, y2, z2) = match iter.next() {
                Some(p2) => (p2[0], p2[1], p2[2]),
                None => panic!("Number of points must be a multiple of 3 for polygon matrix"),
            };

            let v0 = Vec3(x0, y0, z0);
            let v1 = Vec3(x1, y1, z1);
            let v2 = Vec3(x2, y2, z2);

            let vn = (v1 - v0).cross(v2 - v0);

            if vn.2 > 0. {
                self.draw_line(x0, y0, x1, y1);
                self.draw_line(x1, y1, x2, y2);
                self.draw_line(x2, y2, x0, y0);
            }
        }
    }

    fn render_polygon_with_stack(&mut self, stack: &impl MStack<Matrix>, m: &Matrix) {
        self.render_polygon_matrix(&(m * stack.get_top()));
    }

    fn render_edges_with_stack(&mut self, stack: &impl MStack<Matrix>, m: &Matrix) {
        self.render_edge_matrix(&(m * stack.get_top()));
    }
}

#[cfg(test)]
mod tests {
    use super::super::PPMImg;
    use super::*;

    #[test]
    fn test_render_polygon_triangle() {
        let (x0, y0, z0) = (10., 10., 10.);
        let (x1, y1, z1) = (100., 100., 100.);
        let (x2, y2, z2) = (0., 50., 10.);

        let (h, w, d) = (500, 500, 255);
        let mut img_ln = PPMImg::new(h, w, d);
        let mut img_polygon = PPMImg::new(h, w, d);

        let mut m = Matrix::new_polygon_matrix();
        m.append_polygon((x0, y0, z0), (x1, y1, z1), (x2, y2, z2));

        img_polygon.render_polygon_matrix(&m);
        img_ln.draw_line(x0, y0, x1, y1);
        img_ln.draw_line(x1, y1, x2, y2);
        img_ln.draw_line(x2, y2, x0, y0);

        assert_eq!(
            img_ln, img_polygon,
            "Expect equivalent images by adding lines vs. drawing polygon"
        );
    }
}
