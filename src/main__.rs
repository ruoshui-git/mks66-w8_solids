mod graphics;

use graphics::{
    matrix::transform as tr,
    processes::pipe_to_magick,
    Drawer,
    // parser::DWScript,
    // utils::display_ppm,
    Matrix,
    PPMImg, RGB,
};

// # compilation:
// cargo run --release



fn main() {
    let mut convert = pipe_to_magick(vec!["ppm:-", "img.gif"]);

    // child should have a stdin, so we directly unwrap
    let mut magick_in = convert.stdin.take().unwrap();

    let mut drawer = Drawer::new(Box::new(
        PPMImg::new_with_bg(500, 500, 255, RGB::new(253, 255, 186))));

    // colors!
    // let default_fg = img.fg_color;
    // let light_yellow = RGB::new(245, 236, 66);
    // let blue = RGB::new(66, 135, 245);
    // let magenta = RGB::new(239, 66, 245);
    // let purple = RGB::new(209, 66, 245);
    // let brown = RGB::new(212, 143, 78);

    for rot in (0..360).into_iter().step_by(10) {

        // moving to the center
        drawer.push_matrix();
        drawer.transform_by(&tr::mv(250., 250., 0.));
        // drawer.transform_by(&tr::rotatex(rot as f64));

        // drawing center sphere, rotate on rot
        drawer.push_matrix();
        {
            drawer.transform_by(
                &(tr::rotatex(rot as f64) * tr::rotatey(rot as f64) * tr::rotatez(rot as f64)),
            );
            drawer.add_sphere((0., 0., 0.), 200.);
        }

        // // draw the torus around the sphere, rotate on rot
        // drawer.push_matrix();
        // {
        //     drawer.transform_by(&(tr::rotatez(45.) * tr::rotatey(rot as f64)));
        //     polygons.add_torus((0., 0., 0.), 10., 70.);
        //     img.render_polygon_with_stack(&stack, &polygons);
        //     polygons.clear();
        // }
        // stack.pop_matrix();

        println!("Iteration at {}", rot);

        drawer.write_to_buf(&mut magick_in)
            .expect("Error writing img data");

        // sets everything to bg_color, clear zbuf
        drawer.clear();
    }

    drop(magick_in);

    println!("Waiting for convert/magick to exit...");
    let output = convert.wait().expect("Failed to wait on convert/magick");
    println!("convert/magick {}", output);
}
