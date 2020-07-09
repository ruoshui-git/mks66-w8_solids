use super::Canvas;
use crate::graphics::{utils::polar_to_xy, RGB};

pub struct Turtle {
    x: f64,
    y: f64,
    pub angle_deg: f64,
    pub pen_down: bool,
    img: Box<dyn Canvas>,
}

impl Turtle {
    /// Creates a turtle for Canvas
    /// ## Warning
    /// Img will move into a Turtle, so any new bindings to the current instance of PPMImg will be invalid.
    ///
    /// And therefore only one Turtle is allowed at a time for an Img.
    fn new(screen: Box<dyn Canvas>, x: f64, y: f64) -> Turtle {
        Turtle {
            x,
            y,
            angle_deg: 0.0,
            pen_down: false,
            img: screen,
        }
    }

    pub fn forward(&mut self, steps: i32) {
        let (x0, y0) = (self.x, self.y);
        let (dx, dy) = polar_to_xy(steps.into(), self.angle_deg);
        let (x1, y1) = (x0 as f64 + dx, y0 as f64 + dy);
        if self.pen_down {
            self.img.draw_line(x0 as f64, y0 as f64, x1, y1);
        }
        self.x = x1;
        self.y = y1;
    }

    pub fn turn_rt(&mut self, angle_deg: f64) {
        self.angle_deg = (self.angle_deg + angle_deg) % 360.0;
    }

    pub fn set_color(&mut self, rgb: RGB) {
        self.img.set_fg_color(rgb);
    }

    pub fn get_color(&self) -> RGB {
        self.img.get_fg_color()
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        if self.pen_down {
            self.img.draw_line(self.x as f64, self.y as f64, x, y);
        }
        self.x = x;
        self.y = y;
    }

    /// Get the inner PPMImg instance
    ///
    /// This method will move the turtle
    pub fn get_ppm_img(self) -> Box<dyn Canvas> {
        self.img
    }
}
