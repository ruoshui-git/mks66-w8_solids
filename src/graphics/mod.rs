#![allow(dead_code)]

pub mod canvas;
pub mod colors;
pub mod drawer;
pub mod matrix;
pub mod parametrics;
pub mod parser;
pub mod processes;
pub mod utils;
pub mod vector;
pub mod img;


// re-exports
pub use canvas::Canvas;
pub use colors::{HSL, RGB};
pub use matrix::Matrix;
pub use drawer::Drawer;
pub use img::PPMImg;