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

use crate::graphics::{
    matrix::{mstack::MStack, transform},
    utils::{self, Dim},
    Canvas, Matrix, PPMImg,
};

pub struct DWScript {
    filename: String,
    edges: Matrix,
    stack: Vec<Matrix>,
    polygons: Matrix,
    img: PPMImg,
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
            edges: Matrix::new_edge_matrix(),
            polygons: Matrix::new_polygon_matrix(),
            stack: vec![Matrix::ident(4)],
            img: PPMImg::new(500, 500, 255),
            tmpfile_name: String::from("tmp.ppm"),
        }
    }

    fn render_with_top_stack(&mut self, m: &Matrix, dim: Dim) {
        match dim {
            Dim::D3 => self.img.render_polygon_matrix(&(m * self.stack.get_top())),
            Dim::D2 => self.img.render_edge_matrix(&(m * self.stack.get_top())),
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
                    let mut edges = Matrix::new_edge_matrix();
                    edges.append_edge(&pts);
                    self.render_with_top_stack(&edges, Dim::D2);
                }
                "circle" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let values = parse_floats(dline);
                    assert_eq!(4, values.len());
                    let mut edges = Matrix::new_edge_matrix();
                    edges.add_circle((values[0], values[1], values[2]), values[3]);
                    self.render_with_top_stack(&edges, Dim::D2);
                }
                "hermite" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(8, v.len());
                    let mut edges = Matrix::new_edge_matrix();
                    edges.add_hermite3((v[0], v[1]), (v[2], v[3]), (v[4], v[5]), (v[6], v[7]));
                    self.render_with_top_stack(&edges, Dim::D2);
                }
                "bezier" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(8, v.len());
                    let mut edges = Matrix::new_edge_matrix();
                    edges.add_bezier3((v[0], v[1]), (v[2], v[3]), (v[4], v[5]), (v[6], v[7]));
                    self.render_with_top_stack(&edges, Dim::D2);
                }

                "scale" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let scale: Vec<f64> = parse_floats(dline);
                    assert_eq!(3, scale.len());
                    self.stack
                        .transform_top(&transform::scale(scale[0], scale[1], scale[2]));
                }
                "move" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let mv: Vec<f64> = parse_floats(dline);
                    assert_eq!(3, mv.len());
                    self.stack
                        .transform_top(&transform::mv(mv[0], mv[1], mv[2]));
                }
                "rotate" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v: Vec<&str> = dline.split(' ').collect();
                    let (axis, deg): (&str, f64) =
                        (v[0], v[1].parse().expect("Error parsing number"));
                    self.stack.transform_top(&match axis {
                        "x" => transform::rotatex(deg),
                        "y" => transform::rotatey(deg),
                        "z" => transform::rotatez(deg),
                        _ => panic!("Unknown rotation axis on line {}", _dnum),
                    });
                }
                "display" => {
                    utils::display_ppm(&self.img);
                }
                "save" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    self.img.save(dline.as_str()).expect("Error saving image");
                }
                "box" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(6, v.len());
                    let mut m = Matrix::new_polygon_matrix();
                    m.add_box((v[0], v[1], v[2]), v[3], v[4], v[5]);
                    self.render_with_top_stack(&m, Dim::D3);
                }
                "sphere" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(4, v.len());
                    let mut m = Matrix::new_polygon_matrix();
                    m.add_sphere((v[0], v[1], v[2]), v[3]);
                    self.render_with_top_stack(&m, Dim::D3);
                }
                "torus" => {
                    let (_dnum, dline) = getline_or_error(&mut lines);
                    let v = parse_floats(dline);
                    assert_eq!(5, v.len());
                    let mut m = Matrix::new_polygon_matrix();
                    m.add_torus((v[0], v[1], v[2]), v[3], v[4]);
                    self.render_with_top_stack(&m, Dim::D3);
                }
                // "clear" => {
                //     // self.edges.clear();
                //     // self.polygons.clear();
                // }
                "push" => {
                    // self.stack.push(self.stack.get_top().clone());
                    self.stack.push_matrix();
                }
                "pop" => {
                    self.stack.pop_matrix();
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
