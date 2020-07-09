#![allow(dead_code)]

use std::cmp;
use std::convert;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RGB {
    pub red: u16,
    pub blue: u16,
    pub green: u16,
}

// Constructor and some useful "constants"
impl RGB {
    pub fn gray(depth: u16) -> Self {
        RGB {
            red: depth,
            green: depth,
            blue: depth,
        }
    }

    pub fn new(red: u16, green: u16, blue: u16) -> Self {
        RGB { red, green, blue }
    }
}

/// Hue, Saturation, Luminosity
///
/// range: [0, 1]
#[derive(Copy, Clone)]
pub struct HSL {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl convert::From<HSL> for RGB {
    // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB_alternative
    fn from(hsl: HSL) -> RGB {
        let hue = (hsl.h * 360.0).round() as i32;
        let a = hsl.s * fmin2(hsl.l, 1.0 - hsl.l, 1000);
        let f = |n| {
            hsl.l
                - a * fmax2(
                    -1.0,
                    cmp::min(
                        cmp::min((n + hue / 30) % 12 - 3, 9 - (n + hue / 30) % 12),
                        1,
                    ) as f64,
                    1000,
                )
        };
        let (r, g, b) = (
            f(0).round() as i32,
            f(8).round() as i32,
            f(4).round() as i32,
        );
        assert!(r == 1 || r == 0);
        assert!(g == 1 || g == 0);
        assert!(b == 1 || b == 0);
        RGB {
            red: (f(0) * 255.0) as u16,
            green: (f(8) * 255.0) as u16,
            blue: (f(4) * 255.0) as u16,
        }
    }
}

fn fmax2(a: f64, b: f64, prec: i32) -> f64 {
    let fprec = prec as f64;
    cmp::max((a * fprec).round() as i32, (b * fprec).round() as i32) as f64 / fprec
}

fn fmin2(a: f64, b: f64, prec: i32) -> f64 {
    let fprec = prec as f64;
    cmp::min((a * fprec).round() as i32, (b * fprec).round() as i32) as f64 / fprec
}

fn fmin3(a: f64, b: f64, c: f64, prec: i32) -> f64 {
    fmin2(fmin2(a, b, prec), c, prec)
}
