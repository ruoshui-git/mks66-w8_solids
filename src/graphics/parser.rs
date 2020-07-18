//! Goes through the file named filename and performs all of the actions listed in that file.
//! The file follows the following format:
//!
//! - push
//!     - Push a copy of the current top of the coordinate system (cs) stack onto the cs stack (a full copy, not just a reference to the current top… I’m looking at you python people)
//! - pop
//!     - Removes the top of the cs stack (nothing needs to be done with this data)
//! - move/rotate/scale
//!     - create a translation/rotation/scale matrix
//!     - multiply the current top of the cs stack by it
//!     - The ordering of multiplication is important here. (see notes)
//! - box/sphere/torus
//!     - add a box/sphere/torus to a temporary polygon matrix
//!     - multiply it by the current top of the cs stack
//!     - draw it to the screen
//!     - clear the polygon matrix
//! - line/curve/circle
//!     - add a line to a temporary edge matrix
//!     - multiply it by the current top
//!     - draw it to the screen (note a line is not a solid, so avoid draw_polygons)
//!     - clear the edge matrix
//! - save
//!     - save the screen with the provided file name
//! - display
//!     - show the image
//!
//! Also note that the ident, apply and clear commands no longer have any use
//!
use std::{
    fs::File,
    io::{self, prelude::*, BufReader},
};

use crate::graphics::{drawer::Drawer, matrix::transform as tr, PPMImg};

pub struct DWScript {
    filename: String,
    drawer: Drawer,
    tmpfile_name: String,
}

/// Advances a line iterator and panic on error
fn getline_or_error(
    line: &mut impl Iterator<Item = (usize, io::Result<String>)>,
) -> (usize, String) {
    if let Some((num, line)) = line.next() {
        let line = line.expect("Error while reading line").trim().to_string();
        (num, line)
    } else {
        panic!("Error reading line");
    }
}

/// Parse floats from a line and return them in a vec. Panic on error.
fn parse_floats(line: String) -> Vec<f64> {
    line.split(' ')
        .map(|x| x.parse::<f64>().expect("Error parsing numbers"))
        .collect()
}

impl DWScript {
    pub fn new(filename: &str) -> Self {
        DWScript {
            filename: filename.to_string(),
            drawer: Drawer::new(Box::new(PPMImg::new(500, 500, 255))),
            tmpfile_name: String::from("tmp.ppm"),
        }
    }

    pub fn do_parse(&mut self) {
        let _f = File::open(&self.filename).expect("Error opening file");
        let f = BufReader::new(_f);
        let mut lines = f.lines().enumerate();
        while let Some((num, line)) = lines.next() {
            let line = line.expect("Error while reading file");
            match line.trim() {
                x if x.is_empty() || x.starts_with("\\") || x.starts_with("#") => {}
                "line" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let pts: Vec<f64> = parse_floats(dline);
                    assert_eq!(6, pts.len());
                    self.drawer
                        .draw_line((pts[0], pts[1], pts[2]), (pts[3], pts[4], pts[5]));
                }
                "circle" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let values = parse_floats(dline);
                    assert_eq!(4, values.len());
                    self.drawer
                        .draw_circle((values[0], values[1], values[2]), values[3]);
                }
                "hermite" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(8, v.len());
                    self.drawer.draw_hermite(
                        (v[0], v[1]),
                        (v[2], v[3]),
                        (v[4], v[5]),
                        (v[6], v[7]),
                    );
                }
                "bezier" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(8, v.len());
                    self.drawer
                        .draw_bezier((v[0], v[1]), (v[2], v[3]), (v[4], v[5]), (v[6], v[7]));
                }

                "scale" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let scale: Vec<f64> = parse_floats(dline);
                    assert_eq!(3, scale.len());
                    self.drawer
                        .transform_by(&tr::scale(scale[0], scale[1], scale[2]));
                }
                "move" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let mv: Vec<f64> = parse_floats(dline);
                    assert_eq!(3, mv.len());
                    self.drawer.transform_by(&tr::mv(mv[0], mv[1], mv[2]));
                }
                "rotate" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v: Vec<&str> = dline.split(' ').collect();
                    let (axis, deg): (&str, f64) =
                        (v[0], v[1].parse().expect("Error parsing number"));
                    self.drawer.transform_by(&match axis {
                        "x" => tr::rotatex(deg),
                        "y" => tr::rotatey(deg),
                        "z" => tr::rotatez(deg),
                        _ => panic!("Unknown rotation axis on line {}", _dnum),
                    });
                }
                "display" => {
                    self.drawer.display();
                }
                "save" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    self.drawer
                        .save(dline.as_str())
                        .expect("Error saving image");
                }
                "box" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(6, v.len());
                    self.drawer.add_box((v[0], v[1], v[2]), v[3], v[4], v[5]);
                }
                "sphere" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(4, v.len());
                    self.drawer.add_sphere((v[0], v[1], v[2]), v[3]);
                }
                "torus" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(5, v.len());
                    self.drawer.add_torus((v[0], v[1], v[2]), v[3], v[4]);
                }
                "clear" => {
                    self.drawer.clear();
                }
                "push" => {
                    // self.stack.push(self.stack.get_top().clone());
                    self.drawer.push_matrix();
                }
                "pop" => {
                    self.drawer.pop_matrix();
                }
                _ => panic!("Unrecognized command on line {}: {}", num, line),
            }
        }
        // (self.edges.clone(), self.polygons.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn script() {
        DWScript::new("script").do_parse();
    }
}
