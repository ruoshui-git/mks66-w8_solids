use crate::graphics::{Canvas, Matrix, RGB};
use std::io;

/// A procedural interface to simplfy drawing
pub struct Drawer {
    stack: Vec<Matrix>,
    canvas: Box<dyn Canvas>,
}

// helpers
impl Drawer {
    pub fn render_edges_with_stack(&mut self, m: &Matrix) {
        self.canvas.render_edge_matrix(&(m * self.get_top_matrix()))
    }

    pub fn render_polygons_with_stack(&mut self, m: &Matrix) {
        self.canvas
            .render_polygon_matrix(&(m * self.get_top_matrix()))
    }

    fn get_top_matrix(&self) -> &Matrix {
        self.stack
            .last()
            .expect("Error trying to get the last stack")
    }

    fn new_stack() -> Vec<Matrix> {
        vec![Matrix::ident(4)]
    }
}

// colors
impl Drawer {
    pub fn set_fg_color(&mut self, color: RGB) {
        self.canvas.set_fg_color(color);
    }
    pub fn set_bg_color(&mut self, color: RGB) {
        self.canvas.set_bg_color(color);
    }
    pub fn get_fg_color(&self) -> RGB {
        self.canvas.get_fg_color()
    }
    pub fn get_bg_color(&self) -> RGB {
        self.canvas.get_bg_color()
    }
}

impl Drawer {
    pub fn new(canvas: Box<dyn Canvas>) -> Self {
        Drawer {
            stack: Drawer::new_stack(),
            canvas,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.clear();

        // this will cause unexpected behaviors
        // self.stack = Self::new_stack();
    }

    pub fn reset_stack(&mut self) {
        self.stack = Self::new_stack();
    }

    pub fn save(&self, filepath: &str) -> io::Result<()> {
        self.canvas.save(filepath)
    }

    pub fn display(&self) {
        self.canvas.display();
    }
    
    pub fn write_to_buf(&self, writer: &mut dyn io::Write) -> io::Result<()> {
        self.canvas.write_to_buf(writer)
    }
}

// one dimensional stuff
impl Drawer {
    pub fn draw_line(&mut self, p0: (f64, f64, f64), p1: (f64, f64, f64)) {
        let mut edges = Matrix::new_edge_matrix();
        edges.append_edge(&[p0.0, p0.1, p0.2, p1.0, p1.1, p1.2]);
        self.render_edges_with_stack(&edges);
    }
    pub fn draw_circle(&mut self, c: (f64, f64, f64), r: f64) {
        let mut edges = Matrix::new_edge_matrix();
        edges.add_circle(c, r);
        self.render_edges_with_stack(&edges);
    }

    pub fn draw_hermite(&mut self, p0: (f64, f64), p1: (f64, f64), r0: (f64, f64), r1: (f64, f64)) {
        let mut edges = Matrix::new_edge_matrix();
        edges.add_hermite3(p0, p1, r0, r1);
        self.render_edges_with_stack(&edges);
    }
    pub fn draw_bezier(&mut self, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) {
        let mut edges = Matrix::new_edge_matrix();
        edges.add_bezier3(p0, p1, p2, p3);
        self.render_edges_with_stack(&edges);
    }
}

// transformations
impl Drawer {
    pub fn transform_by(&mut self, trans: &Matrix) {
        *self
            .stack
            .last_mut()
            .expect("Error trying to get the last stack") = trans * self.get_top_matrix();
    }
}

// 2d shapes
impl Drawer {
    pub fn add_box(&mut self, (x, y, z): (f64, f64, f64), dx: f64, dy: f64, dz: f64) {
        let mut m = Matrix::new_polygon_matrix();
        m.add_box((x, y, z), dx, dy, dz);
        self.render_polygons_with_stack(&m);
    }
    pub fn add_sphere(&mut self, center: (f64, f64, f64), radius: f64) {
        let mut m = Matrix::new_polygon_matrix();
        m.add_sphere(center, radius);
        self.render_polygons_with_stack(&m);
    }
    pub fn add_torus(&mut self, center: (f64, f64, f64), radius1: f64, radius2: f64) {
        let mut m = Matrix::new_polygon_matrix();
        m.add_torus(center, radius1, radius2);
        self.render_polygons_with_stack(&m);
    }
}

// coordinate stack related
impl Drawer {
    pub fn push_matrix(&mut self) {
        self.stack.push(self.get_top_matrix().clone());
    }

    pub fn pop_matrix(&mut self) {
        self.stack.pop();
    }
}
