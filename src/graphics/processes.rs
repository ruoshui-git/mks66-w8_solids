//! Utility fn to help working with various processes, especially imagemagick

use std::process::{Child, Command, Stdio};

/// Subprocess (and run) `(magick) convert` with the given `args`
pub fn magick(args: Vec<&str>) -> Child {
    Command::new(if cfg!(windows) { "magick" } else { "convert" })
        .args(args)
        .spawn()
        .expect("Can't spawn imagemagick")
}

/// Subprocess (and run) `(magick) convert` with a piped stdin with the given `args`
pub fn pipe_to_magick(args: Vec<&str>) -> Child {
    Command::new(if cfg!(windows) { "magick" } else { "convert" })
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Can't spawn imagemagick")
}

